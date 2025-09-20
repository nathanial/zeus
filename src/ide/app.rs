use crate::ide::editor::EditorPane;
use crate::ide::file_tree::FileTreePane;
use crate::ide::fonts::IdeFonts;
use crate::ide::ide_state::IdeState;
use raylib::prelude::*;

const WINDOW_WIDTH: i32 = 1200;
const WINDOW_HEIGHT: i32 = 800;
const MIN_WINDOW_WIDTH: i32 = 800;
const MIN_WINDOW_HEIGHT: i32 = 600;
const IDE_ATLAS_BASE_SIZE: f32 = 16.0;

pub struct IdeApp {
    rl: RaylibHandle,
    thread: RaylibThread,
    state: IdeState,
    fonts: IdeFonts,
}

impl IdeApp {
    pub fn new() -> Self {
        // Request high-DPI support
        unsafe {
            raylib::ffi::SetConfigFlags(raylib::consts::ConfigFlags::FLAG_WINDOW_HIGHDPI as u32);
        }

        let (mut rl, thread) = raylib::init()
            .size(WINDOW_WIDTH, WINDOW_HEIGHT)
            .title("Zeus LISP IDE - Phase 1")
            .resizable()
            .build();

        let fonts = IdeFonts::load(&mut rl, &thread, IDE_ATLAS_BASE_SIZE);

        let state = IdeState::new();

        let mut app = Self {
            rl,
            thread,
            state,
            fonts,
        };

        // Set minimum window size
        unsafe {
            raylib::ffi::SetWindowMinSize(MIN_WINDOW_WIDTH, MIN_WINDOW_HEIGHT);
        }

        app.rl.set_target_fps(60);
        app
    }

    pub fn run(&mut self) {
        // Initial layout calculation
        let window_bounds = Rectangle {
            x: 0.0,
            y: 0.0,
            width: self.rl.get_screen_width() as f32,
            height: self.rl.get_screen_height() as f32,
        };
        self.state.layout_manager.calculate_bounds(window_bounds);

        // Focus the editor by default
        self.state.focus_pane("editor".to_string());

        while !self.rl.window_should_close() {
            self.handle_input();
            self.update();
            self.draw();
        }
    }

    fn handle_input(&mut self) {
        // Handle global keyboard shortcuts
        let is_ctrl_or_cmd = self.rl.is_key_down(KeyboardKey::KEY_LEFT_CONTROL)
            || self.rl.is_key_down(KeyboardKey::KEY_LEFT_SUPER);

        if is_ctrl_or_cmd {
            // Ctrl/Cmd+1: Focus file tree
            if self.rl.is_key_pressed(KeyboardKey::KEY_ONE) {
                self.state.focus_pane("file_tree".to_string());
            }
            // Ctrl/Cmd+2: Focus editor
            else if self.rl.is_key_pressed(KeyboardKey::KEY_TWO) {
                self.state.focus_pane("editor".to_string());
            }
            // Ctrl/Cmd+3: Focus REPL
            else if self.rl.is_key_pressed(KeyboardKey::KEY_THREE) {
                self.state.focus_pane("repl".to_string());
            }
            // Ctrl/Cmd+4: Focus symbols
            else if self.rl.is_key_pressed(KeyboardKey::KEY_FOUR) {
                self.state.focus_pane("symbols".to_string());
            }
            // Ctrl/Cmd+5: Focus inspector
            else if self.rl.is_key_pressed(KeyboardKey::KEY_FIVE) {
                self.state.focus_pane("inspector".to_string());
            }
            // Ctrl/Cmd+U: Update symbol browser
            else if self.rl.is_key_pressed(KeyboardKey::KEY_U) {
                self.state.update_symbol_browser();
            }
        }

        // Handle mouse clicks to focus panes
        if self
            .rl
            .is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT)
        {
            let mouse_pos = self.rl.get_mouse_position();
            if let Some(pane_id) = self
                .state
                .layout_manager
                .handle_click(mouse_pos.x, mouse_pos.y)
            {
                self.state.focus_pane(pane_id);
            }
        }

        // Handle Tab key to cycle through panes
        if self.rl.is_key_pressed(KeyboardKey::KEY_TAB) {
            let pane_ids: Vec<String> = self.state.panes.keys().cloned().collect();
            if let Some(current_id) = self.state.layout_manager.get_focused_pane() {
                if let Some(current_index) = pane_ids.iter().position(|id| id == current_id) {
                    let next_index = (current_index + 1) % pane_ids.len();
                    self.state.focus_pane(pane_ids[next_index].clone());
                }
            }
        }

        // Let the focused pane handle input
        // We need to clone the focused_id to avoid borrowing issues
        let focused_info = self.state.layout_manager.get_focused_pane().and_then(|id| {
            self.state
                .layout_manager
                .get_pane_bounds(id)
                .map(|bounds| (id.clone(), bounds.clone()))
        });

        if let Some((focused_id, bounds)) = focused_info {
            if let Some(pane) = self.state.panes.get_mut(&focused_id) {
                pane.handle_input(&mut self.rl, bounds.to_rectangle());
            }
        }
    }

    fn update(&mut self) {
        // Update layout if window was resized
        if self.rl.is_window_resized() {
            let window_bounds = Rectangle {
                x: 0.0,
                y: 0.0,
                width: self.rl.get_screen_width() as f32,
                height: self.rl.get_screen_height() as f32,
            };
            self.state.layout_manager.calculate_bounds(window_bounds);
        }

        // Check if file tree has a file to open
        if let Some(pane) = self.state.panes.get_mut("file_tree") {
            if let Some(file_tree) = pane.as_any_mut().downcast_mut::<FileTreePane>() {
                if let Some(path) = file_tree.take_file_to_open() {
                    // Load the file into the editor
                    if let Some(editor_pane) = self.state.panes.get_mut("editor") {
                        if let Some(editor) = editor_pane.as_any_mut().downcast_mut::<EditorPane>()
                        {
                            editor.load_file_from_path(path);
                            // Focus the editor after loading a file
                            self.state.focus_pane("editor".to_string());
                        }
                    }
                }
            }
        }
    }

    fn draw(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);

        // Clear background
        d.clear_background(self.state.theme.background);

        // Draw all visible panes
        for (pane_id, bounds) in self.state.layout_manager.get_all_pane_bounds() {
            if let Some(pane) = self.state.panes.get_mut(pane_id) {
                pane.draw(
                    &mut d,
                    bounds.to_rectangle(),
                    &self.state.theme,
                    &self.fonts,
                );
            }
        }

        // Draw status bar
        let status_height = 25.0;
        let screen_width = d.get_screen_width() as f32;
        let screen_height = d.get_screen_height() as f32;
        let status_y = screen_height - status_height;

        d.draw_rectangle(
            0,
            status_y as i32,
            screen_width as i32,
            status_height as i32,
            self.state.theme.panel,
        );

        // Draw status text
        let status_text = if let Some(focused_id) = self.state.layout_manager.get_focused_pane() {
            format!("Focus: {} | Ctrl+1-5: Switch Panes | Tab: Cycle | Ctrl+E: Evaluate | F5: Refresh Files",
                    focused_id)
        } else {
            "Zeus LISP IDE - Phase 1".to_string()
        };

        self.fonts.draw_text(
            &mut d,
            &status_text,
            Vector2::new(10.0, status_y + 5.0),
            14.0,
            self.state.theme.text,
        );

        // Draw FPS in top-right corner (for debugging)
        let fps_text = format!("FPS: {}", d.get_fps());
        self.fonts.draw_text(
            &mut d,
            &fps_text,
            Vector2::new(screen_width - 80.0, 5.0),
            12.0,
            self.state.theme.text_dim,
        );
    }
}
