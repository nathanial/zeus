use crate::ide::fonts::IdeFonts;
use crate::ide::pane::Pane;
use crate::ide::theme::Theme;
use crate::interpreter::evaluator::Evaluator;
use raylib::prelude::*;
use std::any::Any;
use std::collections::VecDeque;

struct ReplLine {
    text: String,
    is_input: bool,
    is_error: bool,
}

pub struct ReplPane {
    id: String,
    title: String,
    current_input: String,
    history: VecDeque<ReplLine>,
    command_history: Vec<String>,
    command_history_index: Option<usize>,
    cursor_position: usize,
    scroll_offset: i32,
    has_focus: bool,
    evaluator: Evaluator,
}

impl ReplPane {
    pub fn new(id: String, evaluator: Evaluator) -> Self {
        let mut history = VecDeque::new();
        history.push_back(ReplLine {
            text: "Zeus LISP REPL - Interactive Mode".to_string(),
            is_input: false,
            is_error: false,
        });
        history.push_back(ReplLine {
            text: "Type expressions and press Enter to evaluate".to_string(),
            is_input: false,
            is_error: false,
        });

        Self {
            id,
            title: "REPL".to_string(),
            current_input: String::new(),
            history,
            command_history: Vec::new(),
            command_history_index: None,
            cursor_position: 0,
            scroll_offset: 0,
            has_focus: false,
            evaluator,
        }
    }

    fn evaluate_input(&mut self) {
        if self.current_input.trim().is_empty() {
            return;
        }

        // Add input to history
        self.history.push_back(ReplLine {
            text: format!("> {}", self.current_input),
            is_input: true,
            is_error: false,
        });

        // Add to command history
        self.command_history.push(self.current_input.clone());
        self.command_history_index = None;

        // Evaluate the expression
        match self.evaluator.eval_str(&self.current_input) {
            Ok(result) => {
                let formatted = self.format_expr(&result);
                self.history.push_back(ReplLine {
                    text: formatted,
                    is_input: false,
                    is_error: false,
                });
            }
            Err(error) => {
                self.history.push_back(ReplLine {
                    text: format!("Error: {}", error),
                    is_input: false,
                    is_error: true,
                });
            }
        }

        // Clear input
        self.current_input.clear();
        self.cursor_position = 0;

        // Maintain history size
        while self.history.len() > 500 {
            self.history.pop_front();
        }

        // Auto-scroll to bottom
        self.scroll_offset = 0;
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
            Expr::Character(ch) => format!(
                "#\\{}",
                match *ch {
                    ' ' => "space".to_string(),
                    '\n' => "newline".to_string(),
                    '\t' => "tab".to_string(),
                    '\r' => "return".to_string(),
                    c => c.to_string(),
                }
            ),
            Expr::Symbol(sym_data) => match sym_data {
                SymbolData::Keyword(name) => format!(":{}", name),
                SymbolData::Uninterned(name, id) => format!("#:{}#{}", name, id),
                SymbolData::Interned(name) => name.clone(),
            },
            Expr::String(s) => format!("\"{}\"", s),
            Expr::Vector(vec) => {
                let items: Vec<String> = vec.iter().map(|e| self.format_expr(e)).collect();
                format!("[{}]", items.join(" "))
            }
            Expr::HashTable(h) => {
                format!("#<hash-table:{}>", h.len())
            }
            Expr::List(list) => {
                if list.is_empty() {
                    "()".to_string()
                } else {
                    let items: Vec<String> = list.iter().map(|e| self.format_expr(e)).collect();
                    format!("({})", items.join(" "))
                }
            }
            Expr::Cons(car, cdr) => {
                let mut repr = String::from("(");
                repr.push_str(&self.format_expr(car));

                let mut tail = cdr.as_ref();
                loop {
                    match tail {
                        Expr::Cons(next_car, next_cdr) => {
                            repr.push(' ');
                            repr.push_str(&self.format_expr(next_car));
                            tail = next_cdr.as_ref();
                        }
                        Expr::List(list) => {
                            for item in list {
                                repr.push(' ');
                                repr.push_str(&self.format_expr(item));
                            }
                            repr.push(')');
                            break;
                        }
                        other => {
                            repr.push_str(" . ");
                            repr.push_str(&self.format_expr(other));
                            repr.push(')');
                            break;
                        }
                    }
                }
                repr
            }
        }
    }

    fn insert_char(&mut self, ch: char) {
        self.current_input.insert(self.cursor_position, ch);
        self.cursor_position += 1;
    }

    fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
            self.current_input.remove(self.cursor_position);
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_position < self.current_input.len() {
            self.cursor_position += 1;
        }
    }

    fn history_up(&mut self) {
        if self.command_history.is_empty() {
            return;
        }

        if let Some(idx) = self.command_history_index {
            if idx > 0 {
                self.command_history_index = Some(idx - 1);
                self.current_input = self.command_history[idx - 1].clone();
                self.cursor_position = self.current_input.len();
            }
        } else if !self.command_history.is_empty() {
            self.command_history_index = Some(self.command_history.len() - 1);
            self.current_input = self.command_history.last().unwrap().clone();
            self.cursor_position = self.current_input.len();
        }
    }

    fn history_down(&mut self) {
        if let Some(idx) = self.command_history_index {
            if idx < self.command_history.len() - 1 {
                self.command_history_index = Some(idx + 1);
                self.current_input = self.command_history[idx + 1].clone();
                self.cursor_position = self.current_input.len();
            } else {
                self.command_history_index = None;
                self.current_input.clear();
                self.cursor_position = 0;
            }
        }
    }
}

impl Pane for ReplPane {
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
        // Draw background
        d.draw_rectangle_rec(bounds, theme.surface);

        // Draw border
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
        fonts.draw_text(
            d,
            &self.title,
            Vector2::new(bounds.x + 5.0, bounds.y + 5.0),
            16.0,
            theme.text,
        );

        // Calculate content area
        let content_y = bounds.y + title_height + 5.0;
        let content_height = bounds.height - title_height - 40.0; // Leave space for input
        let line_height = 18.0;

        // Draw history with scrolling
        let mut y = content_y - (self.scroll_offset as f32);

        // Use scissor mode to clip content
        let mut scissor = d.begin_scissor_mode(
            bounds.x as i32,
            content_y as i32,
            bounds.width as i32,
            content_height as i32,
        );

        for line in self.history.iter() {
            if y >= content_y - line_height && y < content_y + content_height {
                let color = if line.is_error {
                    theme.error
                } else if line.is_input {
                    theme.text
                } else {
                    theme.success
                };

                fonts.draw_text(
                    &mut scissor,
                    &line.text,
                    Vector2::new(bounds.x + 5.0, y),
                    14.0,
                    color,
                );
            }
            y += line_height;
        }

        drop(scissor);

        // Draw input area
        let input_y = bounds.y + bounds.height - 35.0;
        d.draw_rectangle(
            bounds.x as i32,
            input_y as i32,
            bounds.width as i32,
            35,
            theme.panel,
        );

        // Draw prompt and input
        let prompt = format!("> {}", self.current_input);
        fonts.draw_text(
            d,
            &prompt,
            Vector2::new(bounds.x + 5.0, input_y + 8.0),
            14.0,
            theme.text,
        );

        // Draw cursor
        if self.has_focus {
            let cursor_pos = self.cursor_position.min(self.current_input.len());
            let cursor_slice = &self.current_input[..cursor_pos];
            let cursor_text = format!("> {}", cursor_slice);
            let cursor_metrics = fonts.measure_text(&cursor_text, 14.0);
            let cursor_x = bounds.x + 5.0 + cursor_metrics.x;
            d.draw_rectangle(
                cursor_x.round() as i32,
                (input_y + 8.0) as i32,
                2,
                16,
                theme.cursor,
            );
        }
    }

    fn handle_input(&mut self, rl: &mut RaylibHandle, bounds: Rectangle) -> bool {
        if !self.has_focus {
            return false;
        }

        let mut handled = false;

        // Handle scrolling
        let wheel_move = rl.get_mouse_wheel_move();
        if wheel_move != 0.0 {
            self.scroll_offset -= (wheel_move * 20.0) as i32;
            self.scroll_offset = self.scroll_offset.max(0);

            // Calculate max scroll
            let total_lines = self.history.len() as i32;
            let visible_lines = ((bounds.height - 60.0) / 18.0) as i32;
            let max_scroll = ((total_lines - visible_lines) * 18).max(0);
            self.scroll_offset = self.scroll_offset.min(max_scroll);
            handled = true;
        }

        // Handle keyboard input
        if let Some(key) = rl.get_key_pressed() {
            match key {
                KeyboardKey::KEY_ENTER => {
                    self.evaluate_input();
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
                KeyboardKey::KEY_UP => {
                    self.history_up();
                    handled = true;
                }
                KeyboardKey::KEY_DOWN => {
                    self.history_down();
                    handled = true;
                }
                KeyboardKey::KEY_HOME => {
                    self.cursor_position = 0;
                    handled = true;
                }
                KeyboardKey::KEY_END => {
                    self.cursor_position = self.current_input.len();
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
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
