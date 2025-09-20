use crate::ide::fonts::IdeFonts;
use crate::ide::pane::Pane;
use crate::ide::theme::Theme;
use crate::interpreter::environment::Environment;
use raylib::prelude::*;
use std::any::Any;

#[derive(Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub value: String,
}

#[derive(Clone, Debug)]
pub enum SymbolType {
    Function,
    Macro,
    Variable,
    Constant,
}

pub struct SymbolBrowserPane {
    id: String,
    title: String,
    symbols: Vec<Symbol>,
    filtered_symbols: Vec<Symbol>,
    search_query: String,
    selected_index: usize,
    scroll_offset: i32,
    has_focus: bool,
    environment: Option<Environment>,
}

impl SymbolBrowserPane {
    pub fn new(id: String) -> Self {
        Self {
            id,
            title: "Symbols".to_string(),
            symbols: Vec::new(),
            filtered_symbols: Vec::new(),
            search_query: String::new(),
            selected_index: 0,
            scroll_offset: 0,
            has_focus: false,
            environment: None,
        }
    }

    pub fn update_environment(&mut self, env: Environment) {
        self.environment = Some(env.clone());
        self.refresh_symbols();
    }

    fn refresh_symbols(&mut self) {
        self.symbols.clear();

        if let Some(ref env) = self.environment {
            // Get all symbols from environment
            let bindings = env.get_all_bindings();

            for (name, value) in bindings {
                let symbol_type = self.determine_symbol_type(&value);
                let value_str = self.format_value(&value);

                self.symbols.push(Symbol {
                    name: name.clone(),
                    symbol_type,
                    value: value_str,
                });
            }
        }

        // Add some built-in symbols for demonstration
        self.symbols.push(Symbol {
            name: "+".to_string(),
            symbol_type: SymbolType::Function,
            value: "Built-in arithmetic function".to_string(),
        });
        self.symbols.push(Symbol {
            name: "-".to_string(),
            symbol_type: SymbolType::Function,
            value: "Built-in arithmetic function".to_string(),
        });
        self.symbols.push(Symbol {
            name: "*".to_string(),
            symbol_type: SymbolType::Function,
            value: "Built-in arithmetic function".to_string(),
        });
        self.symbols.push(Symbol {
            name: "/".to_string(),
            symbol_type: SymbolType::Function,
            value: "Built-in arithmetic function".to_string(),
        });
        self.symbols.push(Symbol {
            name: "car".to_string(),
            symbol_type: SymbolType::Function,
            value: "Get first element of list".to_string(),
        });
        self.symbols.push(Symbol {
            name: "cdr".to_string(),
            symbol_type: SymbolType::Function,
            value: "Get rest of list".to_string(),
        });
        self.symbols.push(Symbol {
            name: "cons".to_string(),
            symbol_type: SymbolType::Function,
            value: "Construct list".to_string(),
        });
        self.symbols.push(Symbol {
            name: "list".to_string(),
            symbol_type: SymbolType::Function,
            value: "Create list from arguments".to_string(),
        });
        self.symbols.push(Symbol {
            name: "lambda".to_string(),
            symbol_type: SymbolType::Macro,
            value: "Create anonymous function".to_string(),
        });
        self.symbols.push(Symbol {
            name: "defun".to_string(),
            symbol_type: SymbolType::Macro,
            value: "Define function".to_string(),
        });
        self.symbols.push(Symbol {
            name: "let".to_string(),
            symbol_type: SymbolType::Macro,
            value: "Local bindings".to_string(),
        });
        self.symbols.push(Symbol {
            name: "if".to_string(),
            symbol_type: SymbolType::Macro,
            value: "Conditional expression".to_string(),
        });

        // Sort symbols by name
        self.symbols.sort_by(|a, b| a.name.cmp(&b.name));

        self.filter_symbols();
    }

    fn determine_symbol_type(&self, value: &crate::interpreter::types::Expr) -> SymbolType {
        use crate::interpreter::types::Expr;

        match value {
            Expr::List(list) if !list.is_empty() => {
                if let Expr::Symbol(sym) = &list[0] {
                    if sym.as_str() == "lambda" {
                        return SymbolType::Function;
                    }
                }
                SymbolType::Variable
            }
            _ => SymbolType::Variable,
        }
    }

    fn format_value(&self, value: &crate::interpreter::types::Expr) -> String {
        use crate::interpreter::types::Expr;

        match value {
            Expr::Integer(n) => format!("{}", n),
            Expr::Float(f) => format!("{}", f),
            Expr::String(s) => format!("\"{}\"", s),
            Expr::List(list) if !list.is_empty() => {
                if let Expr::Symbol(sym) = &list[0] {
                    if sym.as_str() == "lambda" {
                        return "Function".to_string();
                    }
                }
                "List".to_string()
            }
            Expr::List(_) => "()".to_string(),
            Expr::Symbol(s) => s.as_str().to_string(),
            _ => "...".to_string(),
        }
    }

    fn filter_symbols(&mut self) {
        if self.search_query.is_empty() {
            self.filtered_symbols = self.symbols.clone();
        } else {
            let query = self.search_query.to_lowercase();
            self.filtered_symbols = self
                .symbols
                .iter()
                .filter(|s| s.name.to_lowercase().contains(&query))
                .cloned()
                .collect();
        }
    }

    fn move_selection_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.ensure_selection_visible();
        }
    }

    fn move_selection_down(&mut self) {
        if self.selected_index < self.filtered_symbols.len().saturating_sub(1) {
            self.selected_index += 1;
            self.ensure_selection_visible();
        }
    }

    fn ensure_selection_visible(&mut self) {
        let line_height = 18;
        let selected_y = self.selected_index as i32 * line_height;

        if selected_y < self.scroll_offset {
            self.scroll_offset = selected_y;
        } else if selected_y > self.scroll_offset + 200 {
            self.scroll_offset = selected_y - 200;
        }
    }

    pub fn get_selected_symbol(&self) -> Option<&Symbol> {
        self.filtered_symbols.get(self.selected_index)
    }
}

impl Pane for SymbolBrowserPane {
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
            &format!("{} ({})", self.title, self.filtered_symbols.len()),
            Vector2::new(bounds.x + 5.0, bounds.y + 5.0),
            16.0,
            theme.text,
        );

        // Draw search box
        let search_y = bounds.y + title_height;
        let search_height = 25.0;
        d.draw_rectangle(
            bounds.x as i32,
            search_y as i32,
            bounds.width as i32,
            search_height as i32,
            theme.panel,
        );

        let search_text = if self.search_query.is_empty() {
            "Search symbols...".to_string()
        } else {
            self.search_query.clone()
        };
        let search_color = if self.search_query.is_empty() {
            theme.text_dim
        } else {
            theme.text
        };
        fonts.draw_text(
            d,
            &search_text,
            Vector2::new(bounds.x + 5.0, search_y + 5.0),
            14.0,
            search_color,
        );

        // Calculate content area
        let content_y = search_y + search_height + 5.0;
        let content_height = bounds.height - title_height - search_height - 10.0;
        let line_height = 18.0;

        // Draw symbols with scrolling
        let mut y = content_y - (self.scroll_offset as f32);

        // Use scissor mode to clip content
        let mut scissor = d.begin_scissor_mode(
            bounds.x as i32,
            content_y as i32,
            bounds.width as i32,
            content_height as i32,
        );

        for (index, symbol) in self.filtered_symbols.iter().enumerate() {
            if y >= content_y - line_height && y < content_y + content_height {
                // Draw selection highlight
                if index == self.selected_index {
                    scissor.draw_rectangle(
                        bounds.x as i32,
                        y as i32,
                        bounds.width as i32,
                        line_height as i32,
                        theme.selection,
                    );
                }

                // Draw symbol type icon
                let (icon, color) = match symbol.symbol_type {
                    SymbolType::Function => ("ƒ", theme.function),
                    SymbolType::Macro => ("M", theme.macro_color),
                    SymbolType::Variable => ("v", theme.text),
                    SymbolType::Constant => ("c", theme.keyword),
                };

                fonts.draw_text(
                    &mut scissor,
                    icon,
                    Vector2::new(bounds.x + 5.0, y),
                    14.0,
                    color,
                );

                // Draw symbol name
                fonts.draw_text(
                    &mut scissor,
                    &symbol.name,
                    Vector2::new(bounds.x + 25.0, y),
                    14.0,
                    theme.text,
                );

                // Draw value preview (truncated)
                let value_preview = if symbol.value.len() > 20 {
                    format!("{}...", &symbol.value[..20])
                } else {
                    symbol.value.clone()
                };

                if bounds.width > 150.0 {
                    fonts.draw_text(
                        &mut scissor,
                        &value_preview,
                        Vector2::new(bounds.x + bounds.width - 100.0, y),
                        12.0,
                        theme.text_dim,
                    );
                }
            }
            y += line_height;
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

            let total_lines = self.filtered_symbols.len() as i32;
            let visible_lines = ((bounds.height - 55.0) / 18.0) as i32;
            let max_scroll = ((total_lines - visible_lines) * 18).max(0);
            self.scroll_offset = self.scroll_offset.min(max_scroll);
            handled = true;
        }

        // Handle keyboard input
        if let Some(key) = rl.get_key_pressed() {
            match key {
                KeyboardKey::KEY_UP => {
                    self.move_selection_up();
                    handled = true;
                }
                KeyboardKey::KEY_DOWN => {
                    self.move_selection_down();
                    handled = true;
                }
                KeyboardKey::KEY_BACKSPACE => {
                    if !self.search_query.is_empty() {
                        self.search_query.pop();
                        self.filter_symbols();
                        self.selected_index = 0;
                        handled = true;
                    }
                }
                _ => {}
            }
        }

        // Handle text input for search
        if let Some(char) = rl.get_char_pressed() {
            if char.is_ascii() && !char.is_control() {
                self.search_query.push(char);
                self.filter_symbols();
                self.selected_index = 0;
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
