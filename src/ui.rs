use crate::interpreter::evaluator::Evaluator;
use raylib::consts::{ConfigFlags, TextureFilter};
use raylib::core::text::RaylibFont;
use raylib::prelude::*;
use std::collections::VecDeque;
use std::path::Path;

const WINDOW_WIDTH: i32 = 900;
const WINDOW_HEIGHT: i32 = 650;
const BASE_FONT_SIZE: f32 = 18.0;
const BASE_FONT_SPACING: f32 = 1.0;
const BASE_TITLE_FONT_SIZE: f32 = 20.0;
const BASE_HELP_FONT_SIZE: f32 = 14.0;
const BASE_LINE_HEIGHT: i32 = 22;
const BASE_PADDING: i32 = 15;
const BASE_TITLE_BAR_HEIGHT: i32 = 35;
const BASE_TITLE_TEXT_OFFSET_Y: i32 = 8;
const BASE_HISTORY_START_Y: i32 = 45;
const BASE_HISTORY_CLIP_TOP: i32 = 35;
const BASE_HISTORY_MARGIN_BOTTOM: i32 = 100;
const BASE_INPUT_AREA_OFFSET: i32 = 65;
const BASE_INPUT_BOX_HEIGHT: i32 = 75;
const BASE_INPUT_BOX_PADDING: i32 = 10;
const BASE_PROMPT_OFFSET: i32 = 22;
const BASE_HELP_OFFSET: i32 = 20;
const BASE_CURSOR_WIDTH: i32 = 2;
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
                font.texture()
                    .set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_POINT);
                return Some(font);
            }
        }
    }

    None
}

pub fn run_ui() {
    // Request a high-DPI backbuffer before creating the window so text stays crisp.
    unsafe {
        raylib::ffi::SetConfigFlags(ConfigFlags::FLAG_WINDOW_HIGHDPI as u32);
    }

    let (mut rl, thread) = raylib::init()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .title("Zeus LISP - Graphical REPL")
        .resizable()
        .build();

    let dpi_scale = rl.get_window_scale_dpi();
    let atlas_scale = dpi_scale.x.max(dpi_scale.y).max(1.0);
    let atlas_font_size = (BASE_FONT_SIZE * atlas_scale).round() as i32;

    let font_size = BASE_FONT_SIZE;
    let font_size_i32 = font_size.round() as i32;
    let font_spacing = BASE_FONT_SPACING;
    let title_font_size = BASE_TITLE_FONT_SIZE;
    let help_font_size = BASE_HELP_FONT_SIZE;

    let line_height = BASE_LINE_HEIGHT;
    let padding = BASE_PADDING;
    let title_bar_height = BASE_TITLE_BAR_HEIGHT;
    let title_text_offset = BASE_TITLE_TEXT_OFFSET_Y;
    let history_start_y = BASE_HISTORY_START_Y;
    let history_clip_top = BASE_HISTORY_CLIP_TOP;
    let history_margin_bottom = BASE_HISTORY_MARGIN_BOTTOM;
    let input_area_offset = BASE_INPUT_AREA_OFFSET;
    let input_box_height = BASE_INPUT_BOX_HEIGHT;
    let input_box_padding = BASE_INPUT_BOX_PADDING;
    let prompt_offset = BASE_PROMPT_OFFSET;
    let help_offset = BASE_HELP_OFFSET;
    let cursor_width = BASE_CURSOR_WIDTH.max(2);
    let scroll_step = (line_height as f32 / 2.0).max(1.0);

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

    let custom_font = load_monospace_font(&mut rl, &thread, atlas_font_size);
    let fallback_font = rl.get_font_default();
    fallback_font
        .texture()
        .set_texture_filter(&thread, TextureFilter::TEXTURE_FILTER_POINT);

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
            scroll_offset -= (wheel_move * scroll_step).round() as i32;
            scroll_offset = scroll_offset.max(0);

            // Calculate max scroll
            let total_lines = history.len() as i32;
            let screen_height = rl.get_screen_height();
            let history_height = screen_height - history_margin_bottom;
            let scissor_height = (history_height - history_clip_top).max(line_height);
            let visible_lines = (scissor_height / line_height).max(1);
            let max_scroll = ((total_lines - visible_lines) * line_height).max(0);
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
        d.draw_rectangle(
            0,
            0,
            d.get_screen_width(),
            title_bar_height,
            Color::new(35, 35, 55, 255),
        );
        if let Some(font) = font_ref {
            d.draw_text_ex(
                font,
                "Zeus LISP - Graphical REPL",
                Vector2::new(padding as f32, title_text_offset as f32),
                title_font_size,
                font_spacing,
                Color::WHITE,
            );
        } else {
            d.draw_text_ex(
                &fallback_font,
                "Zeus LISP - Graphical REPL",
                Vector2::new(padding as f32, title_text_offset as f32),
                title_font_size,
                font_spacing,
                Color::WHITE,
            );
        }

        // Draw history area
        let screen_width = d.get_screen_width();
        let screen_height = d.get_screen_height();
        let history_height = (screen_height - history_margin_bottom).max(line_height);
        let mut y = history_start_y - scroll_offset;

        // Set scissor mode to clip text that goes outside the history area
        {
            let scissor_height = (history_height - history_clip_top).max(line_height);
            let mut scissor =
                d.begin_scissor_mode(0, history_clip_top, screen_width, scissor_height);

            for line in history.iter() {
                if y + line_height >= history_clip_top && y < history_height {
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
                            Vector2::new(padding as f32, y as f32),
                            font_size,
                            font_spacing,
                            color,
                        );
                    } else {
                        scissor.draw_text_ex(
                            &fallback_font,
                            &line.text,
                            Vector2::new(padding as f32, y as f32),
                            font_size,
                            font_spacing,
                            color,
                        );
                    }
                }
                y += line_height;
            }
        }

        // Draw input box
        let input_y = screen_height - input_area_offset;
        let input_box_top = (input_y - input_box_padding).max(0);
        d.draw_rectangle(
            0,
            input_box_top,
            screen_width,
            input_box_height,
            INPUT_BOX_COLOR,
        );
        if let Some(font) = font_ref {
            d.draw_text_ex(
                font,
                "Input:",
                Vector2::new(padding as f32, input_y as f32),
                font_size,
                font_spacing,
                Color::GRAY,
            );
        } else {
            d.draw_text_ex(
                &fallback_font,
                "Input:",
                Vector2::new(padding as f32, input_y as f32),
                font_size,
                font_spacing,
                Color::GRAY,
            );
        }

        let prompt = format!("> {}", current_input);
        let prompt_position = Vector2::new(padding as f32, (input_y + prompt_offset) as f32);
        if let Some(font) = font_ref {
            d.draw_text_ex(
                font,
                &prompt,
                prompt_position,
                font_size,
                font_spacing,
                INPUT_COLOR,
            );
        } else {
            d.draw_text_ex(
                &fallback_font,
                &prompt,
                prompt_position,
                font_size,
                font_spacing,
                INPUT_COLOR,
            );
        }

        // Draw cursor
        if cursor_visible {
            let prompt_metrics = if let Some(font) = font_ref {
                font.measure_text(&prompt, font_size, font_spacing)
            } else {
                fallback_font.measure_text(&prompt, font_size, font_spacing)
            };
            let cursor_x = (padding as f32 + prompt_metrics.x).round() as i32;
            d.draw_rectangle(
                cursor_x,
                input_y + prompt_offset,
                cursor_width,
                font_size_i32,
                INPUT_COLOR,
            );
        }

        // Draw help text at bottom
        if let Some(font) = font_ref {
            d.draw_text_ex(
                font,
                "ESC: Exit | Enter: Evaluate | Mouse Wheel: Scroll",
                Vector2::new(padding as f32, (screen_height - help_offset) as f32),
                help_font_size,
                font_spacing,
                Color::GRAY,
            );
        } else {
            d.draw_text_ex(
                &fallback_font,
                "ESC: Exit | Enter: Evaluate | Mouse Wheel: Scroll",
                Vector2::new(padding as f32, (screen_height - help_offset) as f32),
                help_font_size,
                font_spacing,
                Color::GRAY,
            );
        }
    }
}

fn format_expr(expr: &crate::interpreter::types::Expr) -> String {
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
            let items: Vec<String> = vec.iter().map(format_expr).collect();
            format!("[{}]", items.join(" "))
        }
        Expr::HashTable(h) => {
            format!("#<hash-table:{}>", h.len())
        }
        Expr::List(list) => {
            if list.is_empty() {
                "()".to_string()
            } else {
                let items: Vec<String> = list.iter().map(|e| format_expr(e)).collect();
                format!("({})", items.join(" "))
            }
        }
        Expr::Cons(car, cdr) => {
            let mut repr = String::from("(");
            repr.push_str(&format_expr(car));

            let mut tail = cdr.as_ref();
            loop {
                match tail {
                    Expr::Cons(next_car, next_cdr) => {
                        repr.push(' ');
                        repr.push_str(&format_expr(next_car));
                        tail = next_cdr.as_ref();
                    }
                    Expr::List(list) => {
                        for item in list {
                            repr.push(' ');
                            repr.push_str(&format_expr(item));
                        }
                        repr.push(')');
                        break;
                    }
                    other => {
                        repr.push_str(" . ");
                        repr.push_str(&format_expr(other));
                        repr.push(')');
                        break;
                    }
                }
            }

            repr
        }
    }
}
