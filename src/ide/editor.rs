use crate::ide::pane::Pane;
use crate::ide::theme::Theme;
use crate::interpreter::evaluator::Evaluator;
use raylib::prelude::*;
use std::any::Any;

pub struct EditorPane {
    id: String,
    title: String,
    content: String,
    cursor_position: usize,
    selection_start: Option<usize>,
    selection_end: Option<usize>,
    scroll_offset: i32,
    has_focus: bool,
    evaluator: Evaluator,
    last_result: Option<String>,
    show_result: bool,
    history: Vec<String>,
    history_index: Option<usize>,
}

impl EditorPane {
    pub fn new(id: String) -> Self {
        Self {
            id,
            title: "Editor".to_string(),
            content: String::new(),
            cursor_position: 0,
            selection_start: None,
            selection_end: None,
            scroll_offset: 0,
            has_focus: false,
            evaluator: Evaluator::new(),
            last_result: None,
            show_result: false,
            history: Vec::new(),
            history_index: None,
        }
    }

    fn insert_char(&mut self, ch: char) {
        self.content.insert(self.cursor_position, ch);
        self.cursor_position += 1;

        // Basic paredit: auto-close parentheses
        if ch == '(' {
            self.content.insert(self.cursor_position, ')');
        } else if ch == '[' {
            self.content.insert(self.cursor_position, ']');
        } else if ch == '"' && !self.in_string() {
            self.content.insert(self.cursor_position, '"');
        }
    }

    fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            let ch = self.content.chars().nth(self.cursor_position - 1);

            // Basic paredit: delete matching pairs
            if let Some(ch) = ch {
                if ch == '(' && self.cursor_position < self.content.len() {
                    if let Some(next) = self.content.chars().nth(self.cursor_position) {
                        if next == ')' {
                            self.content.remove(self.cursor_position);
                        }
                    }
                }
            }

            self.cursor_position -= 1;
            self.content.remove(self.cursor_position);
        }
    }

    fn in_string(&self) -> bool {
        let mut in_string = false;
        let mut escaped = false;
        for (i, ch) in self.content.chars().enumerate() {
            if i >= self.cursor_position {
                break;
            }
            if escaped {
                escaped = false;
                continue;
            }
            if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = !in_string;
            }
        }
        in_string
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_position < self.content.len() {
            self.cursor_position += 1;
        }
    }

    fn move_cursor_to_line_start(&mut self) {
        while self.cursor_position > 0 {
            if let Some(ch) = self.content.chars().nth(self.cursor_position - 1) {
                if ch == '\n' {
                    break;
                }
                self.cursor_position -= 1;
            } else {
                break;
            }
        }
    }

    fn move_cursor_to_line_end(&mut self) {
        while self.cursor_position < self.content.len() {
            if let Some(ch) = self.content.chars().nth(self.cursor_position) {
                if ch == '\n' {
                    break;
                }
                self.cursor_position += 1;
            } else {
                break;
            }
        }
    }

    fn evaluate_expression(&mut self) {
        let expr = if let (Some(start), Some(end)) = (self.selection_start, self.selection_end) {
            self.content.chars().skip(start).take(end - start).collect()
        } else {
            self.content.clone()
        };

        if !expr.trim().is_empty() {
            match self.evaluator.eval_str(&expr) {
                Ok(result) => {
                    self.last_result = Some(format!("=> {}", self.format_expr(&result)));
                    self.show_result = true;

                    // Add to history
                    self.history.push(expr);
                    self.history_index = None;
                }
                Err(error) => {
                    self.last_result = Some(format!("Error: {}", error));
                    self.show_result = true;
                }
            }
        }
    }

    fn format_expr(&self, expr: &crate::interpreter::types::Expr) -> String {
        use crate::interpreter::types::{Expr, SymbolData};

        match expr {
            Expr::Integer(n) => format!("{}", n),
            Expr::Float(f) => format!("{}", f),
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

    fn paredit_slurp_forward(&mut self) {
        // Find closing paren and next expression
        let mut paren_depth = 0;
        let mut close_paren_pos = None;

        for (i, ch) in self.content.chars().enumerate().skip(self.cursor_position) {
            match ch {
                '(' => paren_depth += 1,
                ')' => {
                    if paren_depth == 0 {
                        close_paren_pos = Some(self.cursor_position + i);
                        break;
                    }
                    paren_depth -= 1;
                }
                _ => {}
            }
        }

        if let Some(pos) = close_paren_pos {
            // Find next non-whitespace expression
            let mut expr_end = pos + 1;
            let mut found_expr = false;
            let mut depth = 0;

            for (i, ch) in self.content.chars().enumerate().skip(pos + 1) {
                if !ch.is_whitespace() && !found_expr {
                    found_expr = true;
                }
                if found_expr {
                    match ch {
                        '(' | '[' => depth += 1,
                        ')' | ']' => {
                            depth -= 1;
                            if depth < 0 || (depth == 0 && ch.is_whitespace()) {
                                expr_end = pos + 1 + i;
                                break;
                            }
                        }
                        _ if ch.is_whitespace() && depth == 0 => {
                            expr_end = pos + 1 + i;
                            break;
                        }
                        _ => {}
                    }
                }
            }

            if found_expr && expr_end > pos + 1 {
                // Move closing paren to after the expression
                self.content.remove(pos);
                if expr_end > pos {
                    self.content.insert(expr_end - 1, ')');
                }
            }
        }
    }

    fn paredit_barf_forward(&mut self) {
        // Find closing paren and previous expression
        let mut paren_depth = 0;
        let mut close_paren_pos = None;

        for (i, ch) in self.content.chars().enumerate().skip(self.cursor_position) {
            match ch {
                '(' => paren_depth += 1,
                ')' => {
                    if paren_depth == 0 {
                        close_paren_pos = Some(self.cursor_position + i);
                        break;
                    }
                    paren_depth -= 1;
                }
                _ => {}
            }
        }

        if let Some(pos) = close_paren_pos {
            // Find last expression before closing paren
            let mut expr_start = pos;
            let mut depth = 0;

            for i in (0..pos).rev() {
                let ch = self.content.chars().nth(i).unwrap_or(' ');
                match ch {
                    ')' | ']' => depth += 1,
                    '(' | '[' => {
                        depth -= 1;
                        if depth < 0 {
                            break;
                        }
                    }
                    _ if ch.is_whitespace() && depth == 0 => {
                        expr_start = i + 1;
                        break;
                    }
                    _ => {}
                }
            }

            if expr_start < pos {
                // Move closing paren to before the last expression
                self.content.remove(pos);
                self.content.insert(expr_start, ')');
            }
        }
    }
}

impl Pane for EditorPane {
    fn id(&self) -> &str {
        &self.id
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle, bounds: Rectangle, theme: &Theme) {
        // Draw background
        d.draw_rectangle_rec(bounds, theme.surface);

        // Draw border if focused
        if self.has_focus {
            d.draw_rectangle_lines_ex(bounds, 2.0, theme.focus_indicator);
        } else {
            d.draw_rectangle_lines_ex(bounds, 1.0, theme.border);
        }

        // Draw title bar
        let title_height = 25.0;
        d.draw_rectangle(
            bounds.x as i32,
            bounds.y as i32,
            bounds.width as i32,
            title_height as i32,
            theme.panel,
        );
        d.draw_text(
            &self.title,
            (bounds.x + 5.0) as i32,
            (bounds.y + 5.0) as i32,
            16,
            theme.text,
        );

        // Draw content
        let content_y = bounds.y + title_height + 5.0;
        let line_height = 20.0;
        let mut y = content_y;

        for line in self.content.lines() {
            if y < bounds.y + bounds.height - 30.0 {
                d.draw_text(
                    line,
                    (bounds.x + 5.0) as i32,
                    y as i32,
                    14,
                    theme.text,
                );
                y += line_height;
            }
        }

        // Draw cursor
        if self.has_focus {
            let cursor_x = bounds.x + 5.0 + (self.cursor_position as f32 * 8.0);
            let cursor_y = content_y;
            d.draw_rectangle(
                cursor_x as i32,
                cursor_y as i32,
                2,
                16,
                theme.cursor,
            );
        }

        // Draw result if available
        if self.show_result && self.last_result.is_some() {
            let result = self.last_result.as_ref().unwrap();
            let result_y = bounds.y + bounds.height - 25.0;
            d.draw_rectangle(
                bounds.x as i32,
                result_y as i32,
                bounds.width as i32,
                25,
                theme.panel,
            );
            d.draw_text(
                result,
                (bounds.x + 5.0) as i32,
                (result_y + 5.0) as i32,
                12,
                if result.starts_with("Error") {
                    theme.error
                } else {
                    theme.success
                },
            );
        }
    }

    fn handle_input(&mut self, rl: &mut RaylibHandle, _bounds: Rectangle) -> bool {
        if !self.has_focus {
            return false;
        }

        let mut handled = false;

        // Handle keyboard shortcuts
        if rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) || rl.is_key_down(KeyboardKey::KEY_LEFT_SUPER) {
            if rl.is_key_pressed(KeyboardKey::KEY_E) {
                // Evaluate expression
                self.evaluate_expression();
                handled = true;
            } else if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) && rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
                // Paredit slurp forward
                self.paredit_slurp_forward();
                handled = true;
            } else if rl.is_key_pressed(KeyboardKey::KEY_LEFT) && rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) {
                // Paredit barf forward
                self.paredit_barf_forward();
                handled = true;
            }
        }

        // Handle regular keyboard input
        if let Some(key) = rl.get_key_pressed() {
            match key {
                KeyboardKey::KEY_ENTER => {
                    self.insert_char('\n');
                    handled = true;
                }
                KeyboardKey::KEY_BACKSPACE => {
                    self.delete_char();
                    handled = true;
                }
                KeyboardKey::KEY_LEFT => {
                    self.move_cursor_left();
                    handled = true;
                }
                KeyboardKey::KEY_RIGHT => {
                    self.move_cursor_right();
                    handled = true;
                }
                KeyboardKey::KEY_HOME => {
                    self.move_cursor_to_line_start();
                    handled = true;
                }
                KeyboardKey::KEY_END => {
                    self.move_cursor_to_line_end();
                    handled = true;
                }
                KeyboardKey::KEY_UP => {
                    // Navigate history
                    if !self.history.is_empty() {
                        if let Some(idx) = self.history_index {
                            if idx > 0 {
                                self.history_index = Some(idx - 1);
                                self.content = self.history[idx - 1].clone();
                                self.cursor_position = self.content.len();
                            }
                        } else {
                            self.history_index = Some(self.history.len() - 1);
                            self.content = self.history.last().unwrap().clone();
                            self.cursor_position = self.content.len();
                        }
                    }
                    handled = true;
                }
                KeyboardKey::KEY_DOWN => {
                    // Navigate history
                    if let Some(idx) = self.history_index {
                        if idx < self.history.len() - 1 {
                            self.history_index = Some(idx + 1);
                            self.content = self.history[idx + 1].clone();
                            self.cursor_position = self.content.len();
                        } else {
                            self.history_index = None;
                            self.content.clear();
                            self.cursor_position = 0;
                        }
                    }
                    handled = true;
                }
                _ => {}
            }
        }

        // Handle text input
        if let Some(char) = rl.get_char_pressed() {
            if char.is_ascii() && !char.is_control() {
                self.insert_char(char);
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
        self.show_result = false;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}