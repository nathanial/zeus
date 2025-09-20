use crate::ide::fonts::IdeFonts;
use crate::ide::pane::Pane;
use crate::ide::theme::Theme;
use crate::interpreter::evaluator::Evaluator;
use raylib::prelude::*;
use std::any::Any;
use std::cmp::{max, min};
use std::fs;
use std::path::PathBuf;

const MAX_UNDO_STACK: usize = 200;
const TITLE_HEIGHT: f32 = 25.0;
const CONTENT_FONT_SIZE: f32 = 14.0;
const LINE_HEIGHT: f32 = 20.0;
const H_PADDING: f32 = 5.0;
const V_PADDING: f32 = 5.0;
const CARET_WIDTH: i32 = 2;
const KEY_REPEAT_INITIAL_DELAY: f32 = 0.35;
const KEY_REPEAT_INTERVAL: f32 = 0.05;

#[derive(Clone, PartialEq)]
struct EditorSnapshot {
    content: String,
    cursor: usize,
    selection: Option<(usize, usize)>,
}

#[derive(Clone)]
enum PendingCommand {
    Open { buffer: String },
    SaveAs { buffer: String },
}

#[derive(Clone)]
struct KeyRepeatState {
    key: KeyboardKey,
    timer: f32,
    repeating: bool,
}

pub struct EditorPane {
    id: String,
    title: String,
    content: String,
    cursor_position: usize,
    selection: Option<(usize, usize)>,
    selection_anchor: Option<usize>,
    has_focus: bool,
    evaluator: Evaluator,
    last_result: Option<String>,
    show_result: bool,
    undo_stack: Vec<EditorSnapshot>,
    redo_stack: Vec<EditorSnapshot>,
    current_file: Option<PathBuf>,
    saved_content: Option<String>,
    is_dirty: bool,
    indent_with_spaces: bool,
    tab_width: usize,
    smart_indent: bool,
    preferred_column: Option<usize>,
    pending_command: Option<PendingCommand>,
    key_repeat_state: Option<KeyRepeatState>,
}

impl EditorPane {
    pub fn new(id: String) -> Self {
        let mut pane = Self {
            id,
            title: "Editor".to_string(),
            content: String::new(),
            cursor_position: 0,
            selection: None,
            selection_anchor: None,
            has_focus: false,
            evaluator: Evaluator::new(),
            last_result: None,
            show_result: false,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            current_file: None,
            saved_content: Some(String::new()),
            is_dirty: false,
            indent_with_spaces: true,
            tab_width: 4,
            smart_indent: true,
            preferred_column: None,
            pending_command: None,
            key_repeat_state: None,
        };
        pane.capture_initial_state();
        pane
    }

    fn capture_initial_state(&mut self) {
        self.undo_stack.clear();
        self.redo_stack.clear();
        self.undo_stack.push(self.snapshot());
    }

    fn snapshot(&self) -> EditorSnapshot {
        EditorSnapshot {
            content: self.content.clone(),
            cursor: self.cursor_position,
            selection: self.selection,
        }
    }

    fn restore_snapshot(&mut self, snapshot: EditorSnapshot) {
        self.content = snapshot.content;
        let len = self.content.len();
        self.cursor_position = min(snapshot.cursor, len);
        self.selection = snapshot.selection.and_then(|(start, end)| {
            if start == end {
                None
            } else {
                Some((min(start, len), min(end, len)))
            }
        });
        self.selection_anchor = self.selection.map(|(start, _)| start);
        self.preferred_column = None;
        self.mark_dirty();
    }

    fn push_undo_state(&mut self) {
        let snapshot = self.snapshot();
        if self
            .undo_stack
            .last()
            .map(|s| s == &snapshot)
            .unwrap_or(false)
        {
            return;
        }
        self.undo_stack.push(snapshot);
        if self.undo_stack.len() > MAX_UNDO_STACK {
            self.undo_stack.remove(0);
        }
        self.redo_stack.clear();
    }

    fn mark_dirty(&mut self) {
        self.is_dirty = match &self.saved_content {
            Some(saved) => *saved != self.content,
            None => !self.content.is_empty(),
        };
        self.update_title();
    }

    fn update_title(&mut self) {
        let mut title = if let Some(path) = &self.current_file {
            match path.file_name().and_then(|n| n.to_str()) {
                Some(name) => format!("Editor - {}", name),
                None => "Editor".to_string(),
            }
        } else {
            "Editor".to_string()
        };

        if self.is_dirty {
            title.push('*');
        }
        self.title = title;
    }

    fn selection_range(&self) -> Option<(usize, usize)> {
        self.selection
            .map(|(start, end)| (min(start, end), max(start, end)))
    }

    fn has_selection(&self) -> bool {
        self.selection_range()
            .map(|(start, end)| start < end)
            .unwrap_or(false)
    }

    fn clear_selection(&mut self) {
        self.selection = None;
        self.selection_anchor = None;
    }

    fn reset_key_repeat_if_released(&mut self, rl: &RaylibHandle) {
        if let Some(state) = &self.key_repeat_state {
            if !rl.is_key_down(state.key) {
                self.key_repeat_state = None;
            }
        }
    }

    fn key_triggered(&mut self, rl: &mut RaylibHandle, key: KeyboardKey) -> bool {
        if rl.is_key_pressed(key) {
            self.key_repeat_state = Some(KeyRepeatState {
                key,
                timer: 0.0,
                repeating: false,
            });
            return true;
        }

        if let Some(state) = self.key_repeat_state.as_mut() {
            if !rl.is_key_down(state.key) {
                self.key_repeat_state = None;
                return false;
            }

            if state.key == key && rl.is_key_down(key) {
                state.timer += rl.get_frame_time();
                if !state.repeating {
                    if state.timer >= KEY_REPEAT_INITIAL_DELAY {
                        state.timer = 0.0;
                        state.repeating = true;
                        return true;
                    }
                } else if state.timer >= KEY_REPEAT_INTERVAL {
                    state.timer -= KEY_REPEAT_INTERVAL;
                    return true;
                }
            }
        }

        false
    }

    fn delete_selection_internal(&mut self) -> Option<String> {
        if let Some((start, end)) = self.selection_range() {
            if start < end {
                let removed = self.content[start..end].to_string();
                self.content.replace_range(start..end, "");
                self.cursor_position = start;
                self.clear_selection();
                return Some(removed);
            }
        }
        None
    }

    fn clamp_to_char_boundary(&self, idx: usize) -> usize {
        let mut idx = min(idx, self.content.len());
        while idx > 0 && !self.content.is_char_boundary(idx) {
            idx -= 1;
        }
        idx
    }

    fn prev_char_boundary(&self, idx: usize) -> usize {
        if idx == 0 {
            return 0;
        }
        let mut i = self.clamp_to_char_boundary(idx);
        if i == 0 {
            return 0;
        }
        i -= 1;
        while i > 0 && !self.content.is_char_boundary(i) {
            i -= 1;
        }
        i
    }

    fn next_char_boundary(&self, idx: usize) -> usize {
        let mut i = self.clamp_to_char_boundary(idx);
        if i >= self.content.len() {
            return self.content.len();
        }
        i += 1;
        while i < self.content.len() && !self.content.is_char_boundary(i) {
            i += 1;
        }
        i
    }

    fn line_start(&self, idx: usize) -> usize {
        let idx = min(idx, self.content.len());
        self.content[..idx]
            .rfind('\n')
            .map(|pos| pos + 1)
            .unwrap_or(0)
    }

    fn line_end(&self, idx: usize) -> usize {
        let idx = min(idx, self.content.len());
        let remainder = &self.content[idx..];
        remainder
            .find('\n')
            .map(|offset| idx + offset)
            .unwrap_or(self.content.len())
    }

    fn column_at(&self, idx: usize) -> usize {
        let line_start = self.line_start(idx);
        self.content[line_start..idx].chars().count()
    }

    fn index_for_column(&self, line_start: usize, column: usize) -> usize {
        let mut idx = line_start;
        let mut col = 0;
        while idx < self.content.len() {
            if col >= column {
                break;
            }
            let ch = match self.content[idx..].chars().next() {
                Some(c) => c,
                None => break,
            };
            if ch == '\n' {
                break;
            }
            idx += ch.len_utf8();
            col += 1;
        }
        idx
    }

    fn move_cursor_to(&mut self, position: usize, selecting: bool) {
        let position = self.clamp_to_char_boundary(position);
        if selecting {
            let anchor = self.selection_anchor.unwrap_or(self.cursor_position);
            self.selection_anchor = Some(anchor);
            if anchor == position {
                self.selection = None;
            } else {
                self.selection = Some((min(anchor, position), max(anchor, position)));
            }
        } else {
            self.clear_selection();
        }
        self.cursor_position = position;
    }

    fn move_cursor_left(&mut self, selecting: bool) {
        let new_pos = self.prev_char_boundary(self.cursor_position);
        self.move_cursor_to(new_pos, selecting);
        if !selecting {
            self.preferred_column = None;
        }
    }

    fn move_cursor_right(&mut self, selecting: bool) {
        let new_pos = self.next_char_boundary(self.cursor_position);
        self.move_cursor_to(new_pos, selecting);
        if !selecting {
            self.preferred_column = None;
        }
    }

    fn move_cursor_to_line_start(&mut self, selecting: bool) {
        let new_pos = self.line_start(self.cursor_position);
        self.move_cursor_to(new_pos, selecting);
        if !selecting {
            self.preferred_column = None;
        }
    }

    fn move_cursor_to_line_end(&mut self, selecting: bool) {
        let new_pos = self.line_end(self.cursor_position);
        self.move_cursor_to(new_pos, selecting);
        if !selecting {
            self.preferred_column = None;
        }
    }

    fn move_cursor_up(&mut self, selecting: bool) {
        let current_line_start = self.line_start(self.cursor_position);
        if current_line_start == 0 {
            self.move_cursor_to(0, selecting);
            self.preferred_column = Some(self.column_at(self.cursor_position));
            return;
        }
        let previous_line_end = current_line_start.saturating_sub(1);
        let previous_line_start = self.line_start(previous_line_end);
        let target_column = self
            .preferred_column
            .unwrap_or_else(|| self.column_at(self.cursor_position));
        let new_pos = self.index_for_column(previous_line_start, target_column);
        self.move_cursor_to(new_pos, selecting);
        self.preferred_column = Some(target_column);
    }

    fn move_cursor_down(&mut self, selecting: bool) {
        let current_line_end = self.line_end(self.cursor_position);
        if current_line_end >= self.content.len() {
            self.move_cursor_to(self.content.len(), selecting);
            self.preferred_column = Some(self.column_at(self.cursor_position));
            return;
        }
        let next_line_start = min(current_line_end + 1, self.content.len());
        let target_column = self
            .preferred_column
            .unwrap_or_else(|| self.column_at(self.cursor_position));
        let new_pos = self.index_for_column(next_line_start, target_column);
        self.move_cursor_to(new_pos, selecting);
        self.preferred_column = Some(target_column);
    }

    fn select_all(&mut self) {
        if self.content.is_empty() {
            return;
        }
        self.selection = Some((0, self.content.len()));
        self.selection_anchor = Some(0);
        self.cursor_position = self.content.len();
        self.preferred_column = None;
    }

    fn insert_char(&mut self, ch: char) {
        self.push_undo_state();
        self.delete_selection_internal();
        let insert_at = self.cursor_position;
        self.content.insert(insert_at, ch);
        self.cursor_position = insert_at + ch.len_utf8();

        if ch == '(' {
            self.content.insert(self.cursor_position, ')');
        } else if ch == '[' {
            self.content.insert(self.cursor_position, ']');
        } else if ch == '"' && !self.in_string() {
            self.content.insert(self.cursor_position, '"');
        }

        self.clear_selection();
        self.preferred_column = None;
        self.mark_dirty();
    }

    fn insert_text(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }
        self.push_undo_state();
        self.delete_selection_internal();
        self.content.insert_str(self.cursor_position, text);
        self.cursor_position += text.len();
        self.clear_selection();
        self.preferred_column = None;
        self.mark_dirty();
    }

    fn insert_newline(&mut self) {
        self.push_undo_state();
        self.delete_selection_internal();
        let indent = self.indent_for_newline();
        self.content.insert(self.cursor_position, '\n');
        self.cursor_position += 1;
        if !indent.is_empty() {
            self.content.insert_str(self.cursor_position, &indent);
            self.cursor_position += indent.len();
        }
        self.clear_selection();
        self.preferred_column = Some(self.column_at(self.cursor_position));
        self.mark_dirty();
    }

    fn indent_unit(&self) -> String {
        if self.indent_with_spaces {
            " ".repeat(self.tab_width)
        } else {
            "\t".to_string()
        }
    }

    fn indent_for_newline(&self) -> String {
        let line_start = self.line_start(self.cursor_position);
        let line = &self.content[line_start..self.cursor_position];
        let mut indent: String = line
            .chars()
            .take_while(|c| *c == ' ' || *c == '\t')
            .collect();

        if self.smart_indent {
            let mut balance: i32 = 0;
            let mut in_string = false;
            let mut escaped = false;
            for ch in line.chars() {
                if escaped {
                    escaped = false;
                    continue;
                }
                match ch {
                    '\\' => escaped = true,
                    '"' => in_string = !in_string,
                    '(' | '[' if !in_string => balance += 1,
                    ')' | ']' if !in_string && balance > 0 => balance -= 1,
                    _ => {}
                }
            }
            if balance > 0 {
                indent.push_str(&self.indent_unit());
            }
        }

        indent
    }

    fn indent_selection_or_line(&mut self) {
        let indent = self.indent_unit();
        if indent.is_empty() {
            return;
        }

        let (start, end, had_selection) = match self.selection_range() {
            Some((s, e)) if s < e => (s, e, true),
            _ => {
                let line_start = self.line_start(self.cursor_position);
                let line_end = self.line_end(self.cursor_position);
                (line_start, line_end, false)
            }
        };

        let mut indices = Vec::new();
        let mut line_start = self.line_start(start);

        loop {
            indices.push(line_start);
            let line_end = self.line_end(line_start);
            if line_end >= end || line_end >= self.content.len() {
                break;
            }
            if line_end + 1 >= self.content.len() {
                break;
            }
            line_start = line_end + 1;
        }

        self.push_undo_state();

        for &insert_idx in indices.iter().rev() {
            self.content.insert_str(insert_idx, &indent);
        }

        let indent_len = indent.len();

        if had_selection {
            let mut new_start = start;
            let mut new_end = end;
            for &idx in &indices {
                if idx <= new_start {
                    new_start += indent_len;
                }
                if idx <= new_end {
                    new_end += indent_len;
                }
            }
            self.selection = Some((new_start, new_end));
            self.selection_anchor = Some(new_start);
            self.cursor_position = new_end;
        } else {
            let mut new_cursor = self.cursor_position;
            for &idx in &indices {
                if idx <= new_cursor {
                    new_cursor += indent_len;
                }
            }
            self.cursor_position = new_cursor;
            self.clear_selection();
        }

        self.preferred_column = None;
        self.mark_dirty();
    }

    fn outdent_selection_or_line(&mut self) {
        let (start, end, had_selection) = match self.selection_range() {
            Some((s, e)) if s < e => (s, e, true),
            _ => {
                let line_start = self.line_start(self.cursor_position);
                let line_end = self.line_end(self.cursor_position);
                (line_start, line_end, false)
            }
        };

        let mut indices = Vec::new();
        let mut line_start = self.line_start(start);

        loop {
            indices.push(line_start);
            let line_end = self.line_end(line_start);
            if line_end >= end || line_end >= self.content.len() {
                break;
            }
            if line_end + 1 >= self.content.len() {
                break;
            }
            line_start = line_end + 1;
        }

        self.push_undo_state();

        let mut total_shift = 0usize;
        let mut new_start = start;
        let mut new_end = end;
        let mut new_cursor = self.cursor_position;

        for &original_idx in &indices {
            let adjusted_idx = original_idx.saturating_sub(total_shift);
            let removal = self.outdent_amount(adjusted_idx);
            if removal == 0 {
                continue;
            }
            self.content.drain(adjusted_idx..adjusted_idx + removal);
            total_shift += removal;

            if had_selection {
                if original_idx <= new_start {
                    new_start = new_start.saturating_sub(removal);
                }
                if original_idx < new_end {
                    new_end = new_end.saturating_sub(removal);
                }
            } else {
                if original_idx <= new_cursor {
                    new_cursor = new_cursor.saturating_sub(removal);
                }
            }
        }

        if had_selection {
            if new_start >= new_end {
                self.clear_selection();
                self.cursor_position = new_start;
            } else {
                self.selection = Some((new_start, new_end));
                self.selection_anchor = Some(new_start);
                self.cursor_position = new_end;
            }
        } else {
            self.cursor_position = min(new_cursor, self.content.len());
            self.clear_selection();
        }

        self.preferred_column = None;
        self.mark_dirty();
    }

    fn outdent_amount(&self, idx: usize) -> usize {
        if idx >= self.content.len() {
            return 0;
        }
        let slice = &self.content[idx..];
        if self.indent_with_spaces {
            let mut count = 0;
            for ch in slice.chars() {
                if ch == ' ' && count < self.tab_width {
                    count += 1;
                } else {
                    break;
                }
            }
            count
        } else if slice.starts_with('\t') {
            1
        } else {
            0
        }
    }

    fn toggle_indent_mode(&mut self) {
        self.indent_with_spaces = !self.indent_with_spaces;
        if self.indent_with_spaces {
            self.show_status_message(format!("Indent mode: spaces (width {})", self.tab_width));
        } else {
            self.show_status_message("Indent mode: tabs");
        }
    }

    fn adjust_tab_width(&mut self, delta: i32) {
        let mut width = self.tab_width as i32 + delta;
        width = width.clamp(2, 8);
        self.tab_width = width as usize;
        if self.indent_with_spaces {
            self.show_status_message(format!("Indent width: {} spaces", self.tab_width));
        } else {
            self.show_status_message("Indent mode: tabs");
        }
    }

    fn delete_backward(&mut self) {
        if self.has_selection() {
            self.push_undo_state();
            self.delete_selection_internal();
            self.mark_dirty();
            return;
        }
        if self.cursor_position == 0 {
            return;
        }
        let prev = self.prev_char_boundary(self.cursor_position);
        self.push_undo_state();
        self.content.drain(prev..self.cursor_position);
        self.cursor_position = prev;
        self.preferred_column = None;
        self.mark_dirty();
    }

    fn delete_forward(&mut self) {
        if self.has_selection() {
            self.push_undo_state();
            self.delete_selection_internal();
            self.mark_dirty();
            return;
        }
        if self.cursor_position >= self.content.len() {
            return;
        }
        let next = self.next_char_boundary(self.cursor_position);
        self.push_undo_state();
        self.content.drain(self.cursor_position..next);
        self.preferred_column = None;
        self.mark_dirty();
    }

    fn copy_selection(&mut self, rl: &mut RaylibHandle) {
        if let Some((start, end)) = self.selection_range() {
            if start < end {
                let text = &self.content[start..end];
                if let Err(err) = rl.set_clipboard_text(text) {
                    self.show_status_message(format!("Clipboard error: {}", err));
                }
            }
        }
    }

    fn cut_selection(&mut self, rl: &mut RaylibHandle) {
        if let Some((start, end)) = self.selection_range() {
            if start < end {
                let text = self.content[start..end].to_string();
                if let Err(err) = rl.set_clipboard_text(&text) {
                    self.show_status_message(format!("Clipboard error: {}", err));
                    return;
                }
                self.push_undo_state();
                self.content.replace_range(start..end, "");
                self.cursor_position = start;
                self.clear_selection();
                self.preferred_column = None;
                self.mark_dirty();
            }
        }
    }

    fn paste_from_clipboard(&mut self, rl: &mut RaylibHandle) {
        if let Ok(text) = rl.get_clipboard_text() {
            if !text.is_empty() {
                self.insert_text(&text);
            }
        }
    }

    fn undo(&mut self) {
        if self.undo_stack.len() <= 1 {
            return;
        }
        if let Some(snapshot) = self.undo_stack.pop() {
            let current = self.snapshot();
            self.redo_stack.push(current);
            self.restore_snapshot(snapshot);
        }
    }

    fn redo(&mut self) {
        if let Some(snapshot) = self.redo_stack.pop() {
            let current = self.snapshot();
            self.undo_stack.push(current);
            if self.undo_stack.len() > MAX_UNDO_STACK {
                self.undo_stack.remove(0);
            }
            self.restore_snapshot(snapshot);
        }
    }

    fn begin_open_command(&mut self) {
        self.pending_command = Some(PendingCommand::Open {
            buffer: String::new(),
        });
        self.update_command_status();
    }

    fn begin_save_as_command(&mut self) {
        let preset = self
            .current_file
            .as_ref()
            .map(|p| p.display().to_string())
            .unwrap_or_else(|| "untitled.lisp".to_string());
        self.pending_command = Some(PendingCommand::SaveAs { buffer: preset });
        self.update_command_status();
    }

    fn pending_buffer_mut(&mut self) -> Option<&mut String> {
        match self.pending_command.as_mut() {
            Some(PendingCommand::Open { buffer }) => Some(buffer),
            Some(PendingCommand::SaveAs { buffer }) => Some(buffer),
            None => None,
        }
    }

    fn update_command_status(&mut self) {
        if let Some(command) = &self.pending_command {
            match command {
                PendingCommand::Open { buffer } => {
                    self.show_status_message(format!("Open file: {}", buffer));
                }
                PendingCommand::SaveAs { buffer } => {
                    self.show_status_message(format!("Save as: {}", buffer));
                }
            }
        }
    }

    fn cancel_pending_command(&mut self) {
        if self.pending_command.is_some() {
            self.pending_command = None;
            self.show_status_message("Command cancelled");
        }
    }

    fn execute_pending_command(&mut self) {
        if let Some(command) = self.pending_command.take() {
            match command {
                PendingCommand::Open { buffer } => {
                    let trimmed = buffer.trim();
                    if trimmed.is_empty() {
                        self.show_status_message("Open cancelled: empty path");
                    } else {
                        self.load_file(PathBuf::from(trimmed));
                    }
                }
                PendingCommand::SaveAs { buffer } => {
                    let trimmed = buffer.trim();
                    if trimmed.is_empty() {
                        self.show_status_message("Save cancelled: empty path");
                    } else {
                        self.write_to_path(PathBuf::from(trimmed));
                    }
                }
            }
        }
    }

    fn handle_pending_command_input(&mut self, rl: &mut RaylibHandle) -> bool {
        if self.pending_command.is_none() {
            return false;
        }

        self.reset_key_repeat_if_released(rl);

        let mut handled = false;
        let ctrl = rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL)
            || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL)
            || rl.is_key_down(KeyboardKey::KEY_LEFT_SUPER)
            || rl.is_key_down(KeyboardKey::KEY_RIGHT_SUPER);

        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            self.cancel_pending_command();
            return true;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            self.execute_pending_command();
            return true;
        }

        if self.key_triggered(rl, KeyboardKey::KEY_BACKSPACE) {
            if let Some(buffer) = self.pending_buffer_mut() {
                buffer.pop();
                handled = true;
            }
        }

        if ctrl && rl.is_key_pressed(KeyboardKey::KEY_V) {
            if let Ok(text) = rl.get_clipboard_text() {
                if let Some(buffer) = self.pending_buffer_mut() {
                    buffer.push_str(&text);
                    handled = true;
                }
            }
        }

        while let Some(ch) = rl.get_char_pressed() {
            if ch == '\r' || ch == '\n' {
                continue;
            }
            if !ch.is_control() {
                if let Some(buffer) = self.pending_buffer_mut() {
                    buffer.push(ch);
                    handled = true;
                }
            }
        }

        if handled {
            self.update_command_status();
        }

        handled
    }

    fn load_file(&mut self, path: PathBuf) {
        match fs::read_to_string(&path) {
            Ok(contents) => {
                self.content = contents;
                self.cursor_position = self.content.len();
                self.clear_selection();
                self.current_file = Some(path.clone());
                self.saved_content = Some(self.content.clone());
                self.is_dirty = false;
                self.capture_initial_state();
                self.update_title();
                self.show_status_message(format!("Opened {}", path.display()));
            }
            Err(err) => {
                self.show_status_message(format!("Failed to open file: {}", err));
            }
        }
    }

    fn save_file(&mut self) {
        if let Some(path) = self.current_file.clone() {
            self.write_to_path(path);
        } else {
            self.begin_save_as_command();
        }
    }

    fn write_to_path(&mut self, path: PathBuf) {
        if let Err(err) = fs::write(&path, &self.content) {
            self.show_status_message(format!("Failed to save file: {}", err));
            return;
        }
        self.current_file = Some(path.clone());
        self.saved_content = Some(self.content.clone());
        self.is_dirty = false;
        self.update_title();
        self.show_status_message(format!("Saved {}", path.display()));
    }

    fn show_status_message<S: Into<String>>(&mut self, message: S) {
        self.last_result = Some(message.into());
        self.show_result = true;
    }

    fn in_string(&self) -> bool {
        let mut in_string = false;
        let mut escaped = false;
        for ch in self.content[..self.cursor_position].chars() {
            if escaped {
                escaped = false;
                continue;
            }
            match ch {
                '\\' => escaped = true,
                '"' => in_string = !in_string,
                _ => {}
            }
        }
        in_string
    }

    fn evaluate_expression(&mut self) {
        let expr = if let Some((start, end)) = self.selection_range() {
            self.content[start..end].to_string()
        } else {
            self.content.clone()
        };

        if expr.trim().is_empty() {
            return;
        }

        match self.evaluator.eval_str(&expr) {
            Ok(result) => {
                let formatted = self.format_expr(&result);
                self.show_status_message(format!("=> {}", formatted));
            }
            Err(error) => {
                self.show_status_message(format!("Error: {}", error));
            }
        }
    }

    fn format_expr(&self, expr: &crate::interpreter::types::Expr) -> String {
        use crate::interpreter::types::{Expr, SymbolData};

        match expr {
            Expr::Integer(n) => format!("{}", n),
            Expr::Float(f) => format!("{}", f),
            Expr::Rational {
                numerator,
                denominator,
            } => format!("{}/{}", numerator, denominator),
            Expr::Symbol(sym_data) => match sym_data {
                SymbolData::Keyword(name) => format!(":{}", name),
                SymbolData::Uninterned(name, id) => format!("#:{}#{}", name, id),
                SymbolData::Interned(name) => name.clone(),
            },
            Expr::String(s) => format!("\"{}\"", s),
            Expr::List(list) => {
                if list.is_empty() {
                    "()".to_string()
                } else {
                    let items: Vec<String> = list.iter().map(|e| self.format_expr(e)).collect();
                    format!("({})", items.join(" "))
                }
            }
            _ => format!("{:?}", expr),
        }
    }

    fn draw_selection(
        &self,
        d: &mut RaylibDrawHandle,
        fonts: &IdeFonts,
        theme: &Theme,
        line_text: &str,
        line_start: usize,
        line_y: f32,
        selection: (usize, usize),
        bounds: Rectangle,
    ) {
        let (sel_start, sel_end) = selection;
        let line_end = line_start + line_text.len();
        if sel_end <= line_start || sel_start >= line_end {
            return;
        }

        let highlight_start = max(sel_start, line_start);
        let highlight_end = min(sel_end, line_end);
        if highlight_start >= highlight_end {
            return;
        }

        let start_offset = highlight_start - line_start;
        let end_offset = highlight_end - line_start;
        let prefix = &line_text[..start_offset];
        let highlight_segment = &line_text[start_offset..end_offset];

        let prefix_width = fonts.measure_text(prefix, CONTENT_FONT_SIZE).x;
        let mut highlight_width = fonts.measure_text(highlight_segment, CONTENT_FONT_SIZE).x;
        if highlight_width <= 0.5 {
            highlight_width = fonts.measure_text(" ", CONTENT_FONT_SIZE).x;
        }

        let rect = Rectangle {
            x: bounds.x + H_PADDING + prefix_width,
            y: line_y,
            width: highlight_width,
            height: LINE_HEIGHT,
        };
        d.draw_rectangle_rec(rect, theme.selection);
    }
}

impl Pane for EditorPane {
    fn id(&self) -> &str {
        &self.id
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn draw(
        &mut self,
        d: &mut RaylibDrawHandle,
        bounds: Rectangle,
        theme: &Theme,
        fonts: &IdeFonts,
    ) {
        d.draw_rectangle_rec(bounds, theme.surface);

        if self.has_focus {
            d.draw_rectangle_lines_ex(bounds, 2.0, theme.focus_indicator);
        } else {
            d.draw_rectangle_lines_ex(bounds, 1.0, theme.border);
        }

        // Title bar
        d.draw_rectangle(
            bounds.x as i32,
            bounds.y as i32,
            bounds.width as i32,
            TITLE_HEIGHT as i32,
            theme.panel,
        );
        fonts.draw_text(
            d,
            &self.title,
            Vector2::new(bounds.x + H_PADDING, bounds.y + H_PADDING),
            16.0,
            theme.text,
        );

        let content_top = bounds.y + TITLE_HEIGHT + V_PADDING;
        let available_height = bounds.height - TITLE_HEIGHT - V_PADDING * 2.0;
        let mut line_y = content_top;
        let selection = self.selection_range();

        let mut line_start_idx = 0usize;
        let lines: Vec<&str> = self.content.split('\n').collect();

        for line in lines.iter() {
            if line_y + LINE_HEIGHT > bounds.y && line_y < bounds.y + bounds.height - LINE_HEIGHT {
                if let Some(sel) = selection {
                    self.draw_selection(d, fonts, theme, line, line_start_idx, line_y, sel, bounds);
                }
                fonts.draw_text(
                    d,
                    line,
                    Vector2::new(bounds.x + H_PADDING, line_y),
                    CONTENT_FONT_SIZE,
                    theme.text,
                );
            }
            line_y += LINE_HEIGHT;
            line_start_idx += line.len() + 1;
            if line_y > content_top + available_height {
                break;
            }
        }

        // Cursor
        if self.has_focus {
            let cursor_line_start = self.line_start(self.cursor_position);
            let line_number = self.content[..cursor_line_start]
                .chars()
                .filter(|c| *c == '\n')
                .count();
            let prefix = &self.content[cursor_line_start..self.cursor_position];
            let prefix_width = fonts.measure_text(prefix, CONTENT_FONT_SIZE).x;
            let cursor_x = bounds.x + H_PADDING + prefix_width;
            let cursor_y = content_top + line_number as f32 * LINE_HEIGHT;

            if cursor_y >= content_top && cursor_y <= bounds.y + bounds.height - LINE_HEIGHT {
                d.draw_rectangle(
                    cursor_x.round() as i32,
                    cursor_y as i32,
                    CARET_WIDTH,
                    CONTENT_FONT_SIZE.round() as i32,
                    theme.cursor,
                );
            }
        }

        // Status/result area
        if self.show_result {
            let status_y = bounds.y + bounds.height - LINE_HEIGHT - V_PADDING;
            d.draw_rectangle(
                bounds.x as i32,
                status_y as i32,
                bounds.width as i32,
                LINE_HEIGHT as i32,
                theme.panel,
            );
            if let Some(message) = &self.last_result {
                fonts.draw_text(
                    d,
                    message,
                    Vector2::new(bounds.x + H_PADDING, status_y + V_PADDING / 2.0),
                    12.0,
                    if message.starts_with("Error") {
                        theme.error
                    } else {
                        theme.text
                    },
                );
            }
        }
    }

    fn handle_input(&mut self, rl: &mut RaylibHandle, _bounds: Rectangle) -> bool {
        if !self.has_focus {
            return false;
        }

        if self.pending_command.is_some() {
            return self.handle_pending_command_input(rl);
        }

        self.reset_key_repeat_if_released(rl);

        let mut handled = false;
        let ctrl = rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL)
            || rl.is_key_down(KeyboardKey::KEY_RIGHT_CONTROL)
            || rl.is_key_down(KeyboardKey::KEY_LEFT_SUPER)
            || rl.is_key_down(KeyboardKey::KEY_RIGHT_SUPER);
        let alt =
            rl.is_key_down(KeyboardKey::KEY_LEFT_ALT) || rl.is_key_down(KeyboardKey::KEY_RIGHT_ALT);
        let shift = rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT)
            || rl.is_key_down(KeyboardKey::KEY_RIGHT_SHIFT);

        if ctrl {
            if rl.is_key_pressed(KeyboardKey::KEY_S) {
                if shift {
                    self.begin_save_as_command();
                } else {
                    self.save_file();
                }
                handled = true;
            } else if rl.is_key_pressed(KeyboardKey::KEY_O) {
                self.begin_open_command();
                handled = true;
            } else if rl.is_key_pressed(KeyboardKey::KEY_Z) {
                if shift {
                    self.redo();
                } else {
                    self.undo();
                }
                handled = true;
            } else if rl.is_key_pressed(KeyboardKey::KEY_Y) {
                self.redo();
                handled = true;
            } else if rl.is_key_pressed(KeyboardKey::KEY_C) {
                self.copy_selection(rl);
                handled = true;
            } else if rl.is_key_pressed(KeyboardKey::KEY_X) {
                self.cut_selection(rl);
                handled = true;
            } else if rl.is_key_pressed(KeyboardKey::KEY_V) {
                self.paste_from_clipboard(rl);
                handled = true;
            } else if rl.is_key_pressed(KeyboardKey::KEY_A) {
                self.select_all();
                handled = true;
            } else if rl.is_key_pressed(KeyboardKey::KEY_E) {
                self.evaluate_expression();
                handled = true;
            }
        }

        if ctrl && alt {
            if rl.is_key_pressed(KeyboardKey::KEY_I) {
                self.toggle_indent_mode();
                handled = true;
            } else if rl.is_key_pressed(KeyboardKey::KEY_EQUAL)
                || rl.is_key_pressed(KeyboardKey::KEY_KP_ADD)
            {
                self.adjust_tab_width(1);
                handled = true;
            } else if rl.is_key_pressed(KeyboardKey::KEY_MINUS)
                || rl.is_key_pressed(KeyboardKey::KEY_KP_SUBTRACT)
            {
                self.adjust_tab_width(-1);
                handled = true;
            }
        }

        if self.key_triggered(rl, KeyboardKey::KEY_TAB) {
            if shift {
                self.outdent_selection_or_line();
            } else {
                self.indent_selection_or_line();
            }
            handled = true;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            self.insert_newline();
            handled = true;
        }

        if self.key_triggered(rl, KeyboardKey::KEY_BACKSPACE) {
            self.delete_backward();
            handled = true;
        }

        if self.key_triggered(rl, KeyboardKey::KEY_DELETE) {
            self.delete_forward();
            handled = true;
        }

        let selecting = shift;

        if self.key_triggered(rl, KeyboardKey::KEY_LEFT) {
            self.move_cursor_left(selecting);
            handled = true;
        }

        if self.key_triggered(rl, KeyboardKey::KEY_RIGHT) {
            self.move_cursor_right(selecting);
            handled = true;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_HOME) {
            self.move_cursor_to_line_start(selecting);
            handled = true;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_END) {
            self.move_cursor_to_line_end(selecting);
            handled = true;
        }

        if self.key_triggered(rl, KeyboardKey::KEY_UP) {
            self.move_cursor_up(selecting);
            handled = true;
        }

        if self.key_triggered(rl, KeyboardKey::KEY_DOWN) {
            self.move_cursor_down(selecting);
            handled = true;
        }

        while let Some(ch) = rl.get_char_pressed() {
            if !ctrl && !ch.is_control() {
                self.insert_char(ch);
                handled = true;
            }
        }

        handled
    }

    fn on_focus(&mut self) {
        self.has_focus = true;
    }

    fn on_blur(&mut self) {
        self.has_focus = false;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
