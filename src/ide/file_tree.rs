use crate::ide::fonts::IdeFonts;
use crate::ide::pane::Pane;
use crate::ide::theme::Theme;
use raylib::prelude::*;
use std::any::Any;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct FileNode {
    pub path: PathBuf,
    pub name: String,
    pub is_directory: bool,
    pub is_expanded: bool,
    pub children: Vec<FileNode>,
}

impl FileNode {
    fn from_path(path: &Path) -> Option<Self> {
        let name = path.file_name()?.to_str()?.to_string();
        let is_directory = path.is_dir();

        let mut node = FileNode {
            path: path.to_path_buf(),
            name,
            is_directory,
            is_expanded: false,
            children: Vec::new(),
        };

        if is_directory {
            node.load_children();
        }

        Some(node)
    }

    fn load_children(&mut self) {
        if !self.is_directory {
            return;
        }

        self.children.clear();

        if let Ok(entries) = fs::read_dir(&self.path) {
            let mut children: Vec<FileNode> = entries
                .filter_map(|entry| entry.ok().and_then(|e| FileNode::from_path(&e.path())))
                .filter(|node| {
                    // Filter out hidden files and common build directories
                    !node.name.starts_with('.')
                        && node.name != "target"
                        && node.name != "node_modules"
                })
                .collect();

            // Sort directories first, then files, alphabetically within each group
            children.sort_by(|a, b| match (a.is_directory, b.is_directory) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            });

            self.children = children;
        }
    }

    fn toggle_expanded(&mut self) {
        if self.is_directory {
            self.is_expanded = !self.is_expanded;
            if self.is_expanded && self.children.is_empty() {
                self.load_children();
            }
        }
    }
}

pub struct FileTreePane {
    id: String,
    title: String,
    root_path: PathBuf,
    root_node: FileNode,
    selected_path: Option<PathBuf>,
    scroll_offset: i32,
    has_focus: bool,
    file_to_open: Option<PathBuf>,
    pending_toggle: Option<PathBuf>,
}

impl FileTreePane {
    pub fn new(id: String) -> Self {
        let root_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let mut root_node = FileNode {
            path: root_path.clone(),
            name: root_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("Project")
                .to_string(),
            is_directory: true,
            is_expanded: true,
            children: Vec::new(),
        };
        root_node.load_children();

        Self {
            id,
            title: "Files".to_string(),
            root_path,
            root_node,
            selected_path: None,
            scroll_offset: 0,
            has_focus: false,
            file_to_open: None,
            pending_toggle: None,
        }
    }

    pub fn take_file_to_open(&mut self) -> Option<PathBuf> {
        self.file_to_open.take()
    }

    fn toggle_node_at_path(path: &PathBuf, node: &mut FileNode) -> bool {
        if node.path == *path {
            node.toggle_expanded();
            return true;
        }
        if node.is_expanded {
            for child in &mut node.children {
                if Self::toggle_node_at_path(path, child) {
                    return true;
                }
            }
        }
        false
    }

    fn draw_node(
        &self,
        node: &FileNode,
        d: &mut RaylibDrawHandle,
        x: f32,
        y: &mut f32,
        indent: i32,
        theme: &Theme,
        fonts: &IdeFonts,
        bounds: &Rectangle,
        clicked_path: &mut Option<PathBuf>,
        toggle_path: &mut Option<PathBuf>,
    ) {
        let indent_width = 16.0;
        let line_height = 20.0;
        let x_pos = x + (indent as f32 * indent_width);

        // Skip if outside visible area
        if *y < bounds.y || *y > bounds.y + bounds.height {
            *y += line_height;
            if node.is_expanded {
                for child in &node.children {
                    self.draw_node(
                        child,
                        d,
                        x,
                        y,
                        indent + 1,
                        theme,
                        fonts,
                        bounds,
                        clicked_path,
                        toggle_path,
                    );
                }
            }
            return;
        }

        // Draw selection highlight
        if let Some(ref selected) = self.selected_path {
            if selected == &node.path {
                d.draw_rectangle(
                    bounds.x as i32,
                    (*y - 2.0) as i32,
                    bounds.width as i32,
                    line_height as i32,
                    theme.selection,
                );
            }
        }

        let mouse_pos = d.get_mouse_position();
        let is_hovered = mouse_pos.x >= bounds.x
            && mouse_pos.x <= bounds.x + bounds.width
            && mouse_pos.y >= *y - 2.0
            && mouse_pos.y <= *y + line_height - 2.0;

        // Handle mouse clicks
        if is_hovered && d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if node.is_directory {
                *toggle_path = Some(node.path.clone());
            } else {
                *clicked_path = Some(node.path.clone());
            }
        }

        // Draw hover highlight
        if is_hovered && self.selected_path.as_ref() != Some(&node.path) {
            d.draw_rectangle(
                bounds.x as i32,
                (*y - 2.0) as i32,
                bounds.width as i32,
                line_height as i32,
                Color::new(255, 255, 255, 20),
            );
        }

        // Draw expand/collapse indicator for directories
        if node.is_directory {
            let indicator = if node.is_expanded { "▼" } else { "▶" };
            fonts.draw_text(
                d,
                indicator,
                Vector2::new(x_pos - 12.0, *y),
                12.0,
                theme.text_dim,
            );
        }

        // Draw icon
        let (icon, color) = if node.is_directory {
            if node.is_expanded {
                ("📂", theme.keyword)
            } else {
                ("📁", theme.keyword)
            }
        } else {
            // File type icons based on extension
            let icon = match node.path.extension().and_then(|e| e.to_str()) {
                Some("rs") => "🦀",
                Some("lisp") | Some("lsp") | Some("cl") => "🔮",
                Some("md") => "📝",
                Some("toml") | Some("yaml") | Some("yml") | Some("json") => "⚙️",
                Some("txt") => "📄",
                _ => "📄",
            };
            (icon, theme.text)
        };

        fonts.draw_text(d, icon, Vector2::new(x_pos, *y), 14.0, color);

        // Draw name
        let name_color = if node.is_directory {
            theme.keyword
        } else {
            theme.text
        };

        fonts.draw_text(
            d,
            &node.name,
            Vector2::new(x_pos + 20.0, *y),
            14.0,
            name_color,
        );

        *y += line_height;

        // Draw children if expanded
        if node.is_expanded {
            for child in &node.children {
                self.draw_node(
                    child,
                    d,
                    x,
                    y,
                    indent + 1,
                    theme,
                    fonts,
                    bounds,
                    clicked_path,
                    toggle_path,
                );
            }
        }
    }

    fn count_visible_nodes(&self, node: &FileNode) -> i32 {
        let mut count = 1;
        if node.is_expanded {
            for child in &node.children {
                count += self.count_visible_nodes(child);
            }
        }
        count
    }
}

impl Pane for FileTreePane {
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

        // Draw file tree content
        let content_y = bounds.y + title_height + 5.0;
        let content_height = bounds.height - title_height - 10.0;

        // Use scissor mode to clip content
        let mut scissor = d.begin_scissor_mode(
            bounds.x as i32,
            content_y as i32,
            bounds.width as i32,
            content_height as i32,
        );

        let mut y = content_y - (self.scroll_offset as f32);
        let mut clicked_path = None;
        let mut toggle_path = None;

        // Draw the file tree
        self.draw_node(
            &self.root_node,
            &mut scissor,
            bounds.x + 5.0,
            &mut y,
            0,
            theme,
            fonts,
            &Rectangle {
                x: bounds.x,
                y: content_y,
                width: bounds.width,
                height: content_height,
            },
            &mut clicked_path,
            &mut toggle_path,
        );

        // Handle any clicks after drawing
        if let Some(path) = clicked_path {
            self.selected_path = Some(path.clone());
            self.file_to_open = Some(path);
        }

        // Store toggle for later processing
        if let Some(path) = toggle_path {
            self.pending_toggle = Some(path);
        }

        // Process pending toggle
        if let Some(path) = self.pending_toggle.take() {
            Self::toggle_node_at_path(&path, &mut self.root_node);
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

            // Calculate max scroll based on total nodes
            let total_nodes = self.count_visible_nodes(&self.root_node);
            let visible_lines = ((bounds.height - 30.0) / 20.0) as i32;
            let max_scroll = ((total_nodes - visible_lines) * 20).max(0);
            self.scroll_offset = self.scroll_offset.min(max_scroll);

            handled = true;
        }

        // Handle refresh with F5
        if rl.is_key_pressed(KeyboardKey::KEY_F5) {
            self.root_node.load_children();
            handled = true;
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
