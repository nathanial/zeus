use crate::ide::editor::EditorPane;
use crate::ide::inspector::InspectorPane;
use crate::ide::layout::LayoutManager;
use crate::ide::pane::Pane;
use crate::ide::repl_pane::ReplPane;
use crate::ide::symbol_browser::SymbolBrowserPane;
use crate::ide::theme::Theme;
use crate::interpreter::evaluator::Evaluator;
use std::collections::HashMap;

pub struct IdeState {
    pub layout_manager: LayoutManager,
    pub panes: HashMap<String, Box<dyn Pane>>,
    pub theme: Theme,
    pub shared_evaluator: Evaluator,
}

impl IdeState {
    pub fn new() -> Self {
        let shared_evaluator = Evaluator::new();

        let mut panes: HashMap<String, Box<dyn Pane>> = HashMap::new();

        // Create panes
        panes.insert(
            "editor".to_string(),
            Box::new(EditorPane::new("editor".to_string())),
        );

        panes.insert(
            "repl".to_string(),
            Box::new(ReplPane::new("repl".to_string(), shared_evaluator.clone())),
        );

        panes.insert(
            "symbols".to_string(),
            Box::new(SymbolBrowserPane::new("symbols".to_string())),
        );

        panes.insert(
            "inspector".to_string(),
            Box::new(InspectorPane::new("inspector".to_string())),
        );

        Self {
            layout_manager: LayoutManager::create_default(),
            panes,
            theme: Theme::dark(),
            shared_evaluator,
        }
    }

    pub fn get_pane(&self, id: &str) -> Option<&dyn Pane> {
        self.panes.get(id).map(|p| p.as_ref())
    }

    pub fn get_pane_mut(&mut self, id: &str) -> Option<&mut Box<dyn Pane>> {
        self.panes.get_mut(id)
    }

    pub fn focus_pane(&mut self, id: String) {
        // Blur current focused pane
        if let Some(current_id) = self.layout_manager.get_focused_pane() {
            if let Some(pane) = self.panes.get_mut(current_id) {
                pane.on_blur();
            }
        }

        // Focus new pane
        self.layout_manager.focus_pane(id.clone());
        if let Some(pane) = self.panes.get_mut(&id) {
            pane.on_focus();
        }
    }

    pub fn switch_theme(&mut self) {
        // Toggle between dark and light themes
        // In a real implementation, this would check the current theme
        // For now, we'll just use dark theme
        self.theme = Theme::dark();
    }

    pub fn update_symbol_browser(&mut self) {
        // Update the symbol browser with current environment
        if let Some(pane) = self.panes.get_mut("symbols") {
            if let Some(symbols_pane) = pane.as_any_mut().downcast_mut::<SymbolBrowserPane>() {
                symbols_pane.update_environment(self.shared_evaluator.get_environment().clone());
            }
        }
    }

    pub fn inspect_value(&mut self, expr: crate::interpreter::types::Expr) {
        if let Some(pane) = self.panes.get_mut("inspector") {
            if let Some(inspector) = pane.as_any_mut().downcast_mut::<InspectorPane>() {
                inspector.inspect(expr);
            }
        }
    }
}