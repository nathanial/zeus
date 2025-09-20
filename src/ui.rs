use raylib::prelude::*;
use crate::interpreter::evaluator::Evaluator;
use std::collections::VecDeque;

const WINDOW_WIDTH: i32 = 800;
const WINDOW_HEIGHT: i32 = 600;
const FONT_SIZE: i32 = 16;
const LINE_HEIGHT: i32 = 20;
const PADDING: i32 = 10;
const MAX_HISTORY: usize = 100;
const INPUT_COLOR: Color = Color::WHITE;
const OUTPUT_COLOR: Color = Color::GREEN;
const ERROR_COLOR: Color = Color::RED;
const BACKGROUND_COLOR: Color = Color::new(20, 20, 30, 255);
const INPUT_BOX_COLOR: Color = Color::new(30, 30, 40, 255);

struct ReplLine {
    text: String,
    is_input: bool,
    is_error: bool,
}

pub fn run_ui() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Zeus LISP - Graphical REPL")
        .build();

    let mut evaluator = Evaluator::new();
    let mut current_input = String::new();
    let mut history: VecDeque<ReplLine> = VecDeque::new();
    let mut cursor_visible = true;
    let mut cursor_timer = 0.0;
    let mut scroll_offset = 0;

    // Add welcome messages
    history.push_back(ReplLine {
        text: "Zeus LISP v0.1.0 - Graphical Interface".to_string(),
        is_input: false,
        is_error: false,
    });
    history.push_back(ReplLine {
        text: "Type expressions and press Enter to evaluate".to_string(),
        is_input: false,
        is_error: false,
    });
    history.push_back(ReplLine {
        text: "Press ESC to exit".to_string(),
        is_input: false,
        is_error: false,
    });
    history.push_back(ReplLine {
        text: "".to_string(),
        is_input: false,
        is_error: false,
    });

    while !rl.window_should_close() {
        // Handle keyboard input
        if let Some(key) = rl.get_key_pressed() {
            match key {
                KeyboardKey::KEY_ENTER => {
                    if !current_input.is_empty() {
                        // Add input to history
                        history.push_back(ReplLine {
                            text: format!("> {}", current_input),
                            is_input: true,
                            is_error: false,
                        });

                        // Evaluate the expression
                        match evaluator.eval_str(&current_input) {
                            Ok(result) => {
                                let formatted = format_expr(&result);
                                history.push_back(ReplLine {
                                    text: formatted,
                                    is_input: false,
                                    is_error: false,
                                });
                            }
                            Err(error) => {
                                history.push_back(ReplLine {
                                    text: format!("Error: {}", error),
                                    is_input: false,
                                    is_error: true,
                                });
                            }
                        }

                        // Clear input
                        current_input.clear();

                        // Maintain history size
                        while history.len() > MAX_HISTORY {
                            history.pop_front();
                        }

                        // Auto-scroll to bottom
                        scroll_offset = 0;
                    }
                }
                KeyboardKey::KEY_BACKSPACE => {
                    current_input.pop();
                }
                _ => {}
            }
        }

        // Handle text input
        if let Some(char) = rl.get_char_pressed() {
            if char.is_ascii() && !char.is_control() {
                current_input.push(char);
            }
        }

        // Handle scrolling
        let wheel_move = rl.get_mouse_wheel_move();
        if wheel_move != 0.0 {
            scroll_offset += (wheel_move * 3.0) as i32;
            scroll_offset = scroll_offset.max(0);
        }

        // Update cursor blink
        cursor_timer += rl.get_frame_time();
        if cursor_timer >= 0.5 {
            cursor_visible = !cursor_visible;
            cursor_timer = 0.0;
        }

        // Draw
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BACKGROUND_COLOR);

        // Draw title bar
        d.draw_rectangle(0, 0, WINDOW_WIDTH, 30, Color::new(30, 30, 50, 255));
        d.draw_text("Zeus LISP - Graphical REPL", 10, 5, 20, Color::WHITE);

        // Draw history area
        let history_height = WINDOW_HEIGHT - 100;
        let mut y = 40 - scroll_offset;

        for line in history.iter() {
            if y >= 40 && y < history_height {
                let color = if line.is_error {
                    ERROR_COLOR
                } else if line.is_input {
                    INPUT_COLOR
                } else {
                    OUTPUT_COLOR
                };

                d.draw_text(&line.text, PADDING, y, FONT_SIZE, color);
            }
            y += LINE_HEIGHT;
        }

        // Draw input box
        let input_y = WINDOW_HEIGHT - 60;
        d.draw_rectangle(0, input_y - 10, WINDOW_WIDTH, 70, INPUT_BOX_COLOR);
        d.draw_text("Input:", PADDING, input_y, FONT_SIZE, Color::GRAY);
        d.draw_text(&format!("> {}", current_input), PADDING, input_y + 20, FONT_SIZE, INPUT_COLOR);

        // Draw cursor
        if cursor_visible {
            let cursor_x = PADDING + d.measure_text(&format!("> {}", current_input), FONT_SIZE);
            d.draw_rectangle(cursor_x, input_y + 20, 2, FONT_SIZE, INPUT_COLOR);
        }

        // Draw help text at bottom
        d.draw_text("ESC: Exit | Enter: Evaluate | Mouse Wheel: Scroll",
                   PADDING, WINDOW_HEIGHT - 20, 12, Color::GRAY);
    }
}

fn format_expr(expr: &crate::interpreter::types::Expr) -> String {
    use crate::interpreter::types::Expr;

    match expr {
        Expr::Number(n) => {
            if n.fract() == 0.0 && n.abs() < 1e10 {
                format!("{}", *n as i64)
            } else {
                format!("{}", n)
            }
        }
        Expr::Symbol(s) => s.clone(),
        Expr::String(s) => format!("\"{}\"", s),
        Expr::List(list) => {
            if list.is_empty() {
                "()".to_string()
            } else {
                let items: Vec<String> = list.iter().map(|e| format_expr(e)).collect();
                format!("({})", items.join(" "))
            }
        }
    }
}