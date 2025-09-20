use crate::interpreter::evaluator::Evaluator;
use raylib::core::text::RaylibFont;
use raylib::prelude::*;
use std::collections::VecDeque;
use std::path::Path;

const WINDOW_WIDTH: i32 = 900;
const WINDOW_HEIGHT: i32 = 650;
const FONT_SIZE: i32 = 18;
const LINE_HEIGHT: i32 = 22;
const PADDING: i32 = 15;
const FONT_SPACING: f32 = 1.0;
const TITLE_FONT_SIZE: f32 = 20.0;
const HELP_FONT_SIZE: f32 = 14.0;
const MAX_HISTORY: usize = 100;
const INPUT_COLOR: Color = Color::WHITE;
const OUTPUT_COLOR: Color = Color::new(100, 255, 100, 255);
const ERROR_COLOR: Color = Color::new(255, 100, 100, 255);
const BACKGROUND_COLOR: Color = Color::new(25, 25, 35, 255);
const INPUT_BOX_COLOR: Color = Color::new(35, 35, 45, 255);

struct ReplLine {
    text: String,
    is_input: bool,
    is_error: bool,
}

fn load_monospace_font(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    base_size: i32,
) -> Option<Font> {
    const FONT_PATH_CANDIDATES: [&str; 5] = [
        "/System/Applications/Utilities/Terminal.app/Contents/Resources/Fonts/SF-Mono-Regular.otf",
        "/System/Library/Fonts/SF-Mono-Regular.otf",
        "/System/Library/Fonts/SFMono-Regular.otf",
        "/Library/Fonts/Menlo.ttc",
        "/System/Library/Fonts/Menlo.ttc",
    ];

    for path in FONT_PATH_CANDIDATES.iter() {
        if Path::new(path).exists() {
            if let Ok(font) = rl.load_font_ex(thread, path, base_size, None) {
                return Some(font);
            }
        }
    }

    None
}

pub fn run_ui() {
    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Zeus LISP - Graphical REPL")
        .resizable()
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

    let custom_font = load_monospace_font(&mut rl, &thread, FONT_SIZE);
    let fallback_font = rl.get_font_default();

    rl.set_target_fps(60);

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
            scroll_offset -= (wheel_move * 3.0) as i32;
            scroll_offset = scroll_offset.max(0);

            // Calculate max scroll
            let total_lines = history.len() as i32;
            let visible_lines = (WINDOW_HEIGHT - 145) / LINE_HEIGHT;
            let max_scroll = ((total_lines - visible_lines) * LINE_HEIGHT).max(0);
            scroll_offset = scroll_offset.min(max_scroll);
        }

        // Update cursor blink
        cursor_timer += rl.get_frame_time();
        if cursor_timer >= 0.5 {
            cursor_visible = !cursor_visible;
            cursor_timer = 0.0;
        }

        // Draw
        let mut d = rl.begin_drawing(&thread);
        let font_ref = custom_font.as_ref();
        d.clear_background(BACKGROUND_COLOR);

        // Draw title bar
        d.draw_rectangle(0, 0, d.get_screen_width(), 35, Color::new(35, 35, 55, 255));
        if let Some(font) = font_ref {
            d.draw_text_ex(
                font,
                "Zeus LISP - Graphical REPL",
                Vector2::new(15.0, 8.0),
                TITLE_FONT_SIZE,
                FONT_SPACING,
                Color::WHITE,
            );
        } else {
            d.draw_text_ex(
                &fallback_font,
                "Zeus LISP - Graphical REPL",
                Vector2::new(15.0, 8.0),
                TITLE_FONT_SIZE,
                FONT_SPACING,
                Color::WHITE,
            );
        }

        // Draw history area
        let screen_width = d.get_screen_width();
        let screen_height = d.get_screen_height();
        let history_height = screen_height - 100;
        let mut y = 45 - scroll_offset;

        // Set scissor mode to clip text that goes outside the history area
        {
            let mut scissor = d.begin_scissor_mode(0, 35, screen_width, history_height - 35);

            for line in history.iter() {
                if y + LINE_HEIGHT >= 35 && y < history_height {
                    let color = if line.is_error {
                        ERROR_COLOR
                    } else if line.is_input {
                        INPUT_COLOR
                    } else {
                        OUTPUT_COLOR
                    };

                    if let Some(font) = font_ref {
                        scissor.draw_text_ex(
                            font,
                            &line.text,
                            Vector2::new(PADDING as f32, y as f32),
                            FONT_SIZE as f32,
                            FONT_SPACING,
                            color,
                        );
                    } else {
                        scissor.draw_text_ex(
                            &fallback_font,
                            &line.text,
                            Vector2::new(PADDING as f32, y as f32),
                            FONT_SIZE as f32,
                            FONT_SPACING,
                            color,
                        );
                    }
                }
                y += LINE_HEIGHT;
            }
        }

        // Draw input box
        let input_y = screen_height - 65;
        d.draw_rectangle(0, input_y - 10, screen_width, 75, INPUT_BOX_COLOR);
        if let Some(font) = font_ref {
            d.draw_text_ex(
                font,
                "Input:",
                Vector2::new(PADDING as f32, input_y as f32),
                FONT_SIZE as f32,
                FONT_SPACING,
                Color::GRAY,
            );
        } else {
            d.draw_text_ex(
                &fallback_font,
                "Input:",
                Vector2::new(PADDING as f32, input_y as f32),
                FONT_SIZE as f32,
                FONT_SPACING,
                Color::GRAY,
            );
        }

        let prompt = format!("> {}", current_input);
        let prompt_position = Vector2::new(PADDING as f32, (input_y + 22) as f32);
        if let Some(font) = font_ref {
            d.draw_text_ex(
                font,
                &prompt,
                prompt_position,
                FONT_SIZE as f32,
                FONT_SPACING,
                INPUT_COLOR,
            );
        } else {
            d.draw_text_ex(
                &fallback_font,
                &prompt,
                prompt_position,
                FONT_SIZE as f32,
                FONT_SPACING,
                INPUT_COLOR,
            );
        }

        // Draw cursor
        if cursor_visible {
            let prompt_metrics = if let Some(font) = font_ref {
                font.measure_text(&prompt, FONT_SIZE as f32, FONT_SPACING)
            } else {
                fallback_font.measure_text(&prompt, FONT_SIZE as f32, FONT_SPACING)
            };
            let cursor_x = (PADDING as f32 + prompt_metrics.x).round() as i32;
            d.draw_rectangle(cursor_x, input_y + 22, 2, FONT_SIZE, INPUT_COLOR);
        }

        // Draw help text at bottom
        if let Some(font) = font_ref {
            d.draw_text_ex(
                font,
                "ESC: Exit | Enter: Evaluate | Mouse Wheel: Scroll",
                Vector2::new(PADDING as f32, (screen_height - 20) as f32),
                HELP_FONT_SIZE,
                FONT_SPACING,
                Color::GRAY,
            );
        } else {
            d.draw_text_ex(
                &fallback_font,
                "ESC: Exit | Enter: Evaluate | Mouse Wheel: Scroll",
                Vector2::new(PADDING as f32, (screen_height - 20) as f32),
                HELP_FONT_SIZE,
                FONT_SPACING,
                Color::GRAY,
            );
        }
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
