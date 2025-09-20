use crate::ide::pane::Pane;
use crate::ide::theme::Theme;
use crate::interpreter::types::Expr;
use raylib::prelude::*;
use std::any::Any;
use std::collections::HashMap;

pub struct InspectorPane {
    id: String,
    title: String,
    current_value: Option<Expr>,
    expanded_nodes: HashMap<String, bool>,
    scroll_offset: i32,
    has_focus: bool,
}

impl InspectorPane {
    pub fn new(id: String) -> Self {
        Self {
            id,
            title: "Inspector".to_string(),
            current_value: None,
            expanded_nodes: HashMap::new(),
            scroll_offset: 0,
            has_focus: false,
        }
    }

    pub fn inspect(&mut self, value: Expr) {
        self.current_value = Some(value);
        self.scroll_offset = 0;
    }

    pub fn clear(&mut self) {
        self.current_value = None;
        self.expanded_nodes.clear();
        self.scroll_offset = 0;
    }

    fn draw_expr(&self, d: &mut RaylibDrawHandle, expr: &Expr, x: f32, y: &mut f32, indent: i32, theme: &Theme, bounds: &Rectangle) {
        use crate::interpreter::types::{Expr, SymbolData};

        let indent_width = 20.0;
        let line_height = 18.0;
        let x_pos = x + (indent as f32 * indent_width);

        if *y > bounds.y + bounds.height {
            return; // Skip drawing if outside visible area
        }

        match expr {
            Expr::Integer(n) => {
                d.draw_text(
                    &format!("{}", n),
                    x_pos as i32,
                    *y as i32,
                    14,
                    theme.number,
                );
                *y += line_height;
            }
            Expr::Float(f) => {
                d.draw_text(
                    &format!("{}", f),
                    x_pos as i32,
                    *y as i32,
                    14,
                    theme.number,
                );
                *y += line_height;
            }
            Expr::Rational {
                numerator,
                denominator,
            } => {
                d.draw_text(
                    &format!("{}/{}", numerator, denominator),
                    x_pos as i32,
                    *y as i32,
                    14,
                    theme.number,
                );
                *y += line_height;
            }
            Expr::Character(ch) => {
                let repr = match *ch {
                    ' ' => "#\\space".to_string(),
                    '\n' => "#\\newline".to_string(),
                    '\t' => "#\\tab".to_string(),
                    '\r' => "#\\return".to_string(),
                    c => format!("#\\{}", c),
                };
                d.draw_text(
                    &repr,
                    x_pos as i32,
                    *y as i32,
                    14,
                    theme.string,
                );
                *y += line_height;
            }
            Expr::Symbol(sym_data) => {
                let text = match sym_data {
                    SymbolData::Keyword(name) => format!(":{}", name),
                    SymbolData::Uninterned(name, id) => format!("#:{}#{}", name, id),
                    SymbolData::Interned(name) => name.clone(),
                };
                d.draw_text(
                    &text,
                    x_pos as i32,
                    *y as i32,
                    14,
                    theme.keyword,
                );
                *y += line_height;
            }
            Expr::String(s) => {
                let display = if s.len() > 50 {
                    format!("\"{}...\"", &s[..50])
                } else {
                    format!("\"{}\"", s)
                };
                d.draw_text(
                    &display,
                    x_pos as i32,
                    *y as i32,
                    14,
                    theme.string,
                );
                *y += line_height;
            }
            Expr::List(list) => {
                let node_id = format!("list_{}", *y as i32);
                let expanded = self.expanded_nodes.get(&node_id).copied().unwrap_or(true);

                // Draw expand/collapse indicator
                let indicator = if expanded { "▼" } else { "▶" };
                d.draw_text(
                    indicator,
                    (x_pos - 15.0) as i32,
                    *y as i32,
                    14,
                    theme.text,
                );

                if list.is_empty() {
                    d.draw_text(
                        "()",
                        x_pos as i32,
                        *y as i32,
                        14,
                        theme.text,
                    );
                    *y += line_height;
                } else {
                    d.draw_text(
                        &format!("List [{}]", list.len()),
                        x_pos as i32,
                        *y as i32,
                        14,
                        theme.text,
                    );
                    *y += line_height;

                    if expanded {
                        for (i, item) in list.iter().enumerate() {
                            if *y > bounds.y + bounds.height {
                                break;
                            }
                            d.draw_text(
                                &format!("[{}]:", i),
                                (x_pos + indent_width) as i32,
                                *y as i32,
                                12,
                                theme.text_dim,
                            );
                            *y += line_height;
                            self.draw_expr(d, item, x, y, indent + 2, theme, bounds);
                        }
                    }
                }
            }
            Expr::Vector(vec) => {
                let node_id = format!("vec_{}", *y as i32);
                let expanded = self.expanded_nodes.get(&node_id).copied().unwrap_or(true);

                let indicator = if expanded { "▼" } else { "▶" };
                d.draw_text(
                    indicator,
                    (x_pos - 15.0) as i32,
                    *y as i32,
                    14,
                    theme.text,
                );

                d.draw_text(
                    &format!("Vector [{}]", vec.len()),
                    x_pos as i32,
                    *y as i32,
                    14,
                    theme.text,
                );
                *y += line_height;

                if expanded {
                    for (i, item) in vec.iter().enumerate() {
                        if *y > bounds.y + bounds.height {
                            break;
                        }
                        d.draw_text(
                            &format!("[{}]:", i),
                            (x_pos + indent_width) as i32,
                            *y as i32,
                            12,
                            theme.text_dim,
                        );
                        *y += line_height;
                        self.draw_expr(d, item, x, y, indent + 2, theme, bounds);
                    }
                }
            }
            Expr::HashTable(h) => {
                let node_id = format!("hash_{}", *y as i32);
                let expanded = self.expanded_nodes.get(&node_id).copied().unwrap_or(false);

                let indicator = if expanded { "▼" } else { "▶" };
                d.draw_text(
                    indicator,
                    (x_pos - 15.0) as i32,
                    *y as i32,
                    14,
                    theme.text,
                );

                d.draw_text(
                    &format!("HashTable [{}]", h.len()),
                    x_pos as i32,
                    *y as i32,
                    14,
                    theme.text,
                );
                *y += line_height;

                if expanded {
                    for (key, value) in h.iter() {
                        if *y > bounds.y + bounds.height {
                            break;
                        }
                        d.draw_text(
                            &format!("{:?}:", key),
                            (x_pos + indent_width) as i32,
                            *y as i32,
                            12,
                            theme.text_dim,
                        );
                        *y += line_height;
                        self.draw_expr(d, value, x, y, indent + 2, theme, bounds);
                    }
                }
            }
            Expr::Cons(car, cdr) => {
                let node_id = format!("cons_{}", *y as i32);
                let expanded = self.expanded_nodes.get(&node_id).copied().unwrap_or(true);

                let indicator = if expanded { "▼" } else { "▶" };
                d.draw_text(
                    indicator,
                    (x_pos - 15.0) as i32,
                    *y as i32,
                    14,
                    theme.text,
                );

                d.draw_text(
                    "Cons",
                    x_pos as i32,
                    *y as i32,
                    14,
                    theme.text,
                );
                *y += line_height;

                if expanded {
                    d.draw_text(
                        "car:",
                        (x_pos + indent_width) as i32,
                        *y as i32,
                        12,
                        theme.text_dim,
                    );
                    *y += line_height;
                    self.draw_expr(d, car, x, y, indent + 2, theme, bounds);

                    d.draw_text(
                        "cdr:",
                        (x_pos + indent_width) as i32,
                        *y as i32,
                        12,
                        theme.text_dim,
                    );
                    *y += line_height;
                    self.draw_expr(d, cdr, x, y, indent + 2, theme, bounds);
                }
            }
        }
    }

    fn toggle_node(&mut self, y: i32) {
        let node_id = format!("node_{}", y);
        let expanded = self.expanded_nodes.get(&node_id).copied().unwrap_or(true);
        self.expanded_nodes.insert(node_id, !expanded);
    }
}

impl Pane for InspectorPane {
    fn id(&self) -> &str {
        &self.id
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn draw(&mut self, d: &mut RaylibDrawHandle, bounds: Rectangle, theme: &Theme) {
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
        d.draw_text(
            &self.title,
            (bounds.x + 5.0) as i32,
            (bounds.y + 5.0) as i32,
            16,
            theme.text,
        );

        // Draw content
        let content_y = bounds.y + title_height + 5.0;
        let content_height = bounds.height - title_height - 10.0;

        // Use scissor mode to clip content
        let mut scissor = d.begin_scissor_mode(
            bounds.x as i32,
            content_y as i32,
            bounds.width as i32,
            content_height as i32,
        );

        if let Some(ref value) = self.current_value {
            let mut y = content_y - (self.scroll_offset as f32);
            self.draw_expr(&mut scissor, value, bounds.x + 20.0, &mut y, 0, theme, &bounds);
        } else {
            scissor.draw_text(
                "No value to inspect",
                (bounds.x + 10.0) as i32,
                content_y as i32,
                14,
                theme.text_dim,
            );
            scissor.draw_text(
                "Select an expression to inspect",
                (bounds.x + 10.0) as i32,
                (content_y + 20.0) as i32,
                12,
                theme.text_dim,
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
            handled = true;
        }

        // Handle mouse clicks to expand/collapse nodes
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse_pos = rl.get_mouse_position();
            if mouse_pos.x >= bounds.x && mouse_pos.x <= bounds.x + bounds.width
                && mouse_pos.y >= bounds.y && mouse_pos.y <= bounds.y + bounds.height
            {
                let clicked_y = (mouse_pos.y - bounds.y + self.scroll_offset as f32) as i32;
                self.toggle_node(clicked_y);
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