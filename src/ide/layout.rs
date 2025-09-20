use crate::ide::pane::PaneBounds;
use raylib::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum LayoutNode {
    Leaf(String), // Pane ID
    Split {
        direction: SplitDirection,
        ratio: f32,
        first: Box<LayoutNode>,
        second: Box<LayoutNode>,
    },
    Tabs {
        active: usize,
        panes: Vec<String>,
    },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SplitDirection {
    Horizontal,
    Vertical,
}

pub struct LayoutManager {
    root: LayoutNode,
    pane_bounds: HashMap<String, PaneBounds>,
    focused_pane: Option<String>,
}

impl LayoutManager {
    pub fn new(initial_pane: String) -> Self {
        Self {
            root: LayoutNode::Leaf(initial_pane),
            pane_bounds: HashMap::new(),
            focused_pane: None,
        }
    }

    pub fn create_default() -> Self {
        Self {
            root: LayoutNode::Split {
                direction: SplitDirection::Horizontal,
                ratio: 0.7,
                first: Box::new(LayoutNode::Split {
                    direction: SplitDirection::Vertical,
                    ratio: 0.7,
                    first: Box::new(LayoutNode::Leaf("editor".to_string())),
                    second: Box::new(LayoutNode::Leaf("repl".to_string())),
                }),
                second: Box::new(LayoutNode::Split {
                    direction: SplitDirection::Vertical,
                    ratio: 0.5,
                    first: Box::new(LayoutNode::Leaf("symbols".to_string())),
                    second: Box::new(LayoutNode::Leaf("inspector".to_string())),
                }),
            },
            pane_bounds: HashMap::new(),
            focused_pane: Some("editor".to_string()),
        }
    }

    pub fn calculate_bounds(&mut self, window_bounds: Rectangle) {
        self.pane_bounds.clear();
        self.calculate_node_bounds(&self.root.clone(), window_bounds);
    }

    fn calculate_node_bounds(&mut self, node: &LayoutNode, bounds: Rectangle) {
        match node {
            LayoutNode::Leaf(pane_id) => {
                self.pane_bounds.insert(
                    pane_id.clone(),
                    PaneBounds {
                        x: bounds.x,
                        y: bounds.y,
                        width: bounds.width,
                        height: bounds.height,
                    },
                );
            }
            LayoutNode::Split {
                direction,
                ratio,
                first,
                second,
            } => match direction {
                SplitDirection::Horizontal => {
                    let first_width = bounds.width * ratio;
                    self.calculate_node_bounds(
                        first,
                        Rectangle {
                            x: bounds.x,
                            y: bounds.y,
                            width: first_width,
                            height: bounds.height,
                        },
                    );
                    self.calculate_node_bounds(
                        second,
                        Rectangle {
                            x: bounds.x + first_width,
                            y: bounds.y,
                            width: bounds.width - first_width,
                            height: bounds.height,
                        },
                    );
                }
                SplitDirection::Vertical => {
                    let first_height = bounds.height * ratio;
                    self.calculate_node_bounds(
                        first,
                        Rectangle {
                            x: bounds.x,
                            y: bounds.y,
                            width: bounds.width,
                            height: first_height,
                        },
                    );
                    self.calculate_node_bounds(
                        second,
                        Rectangle {
                            x: bounds.x,
                            y: bounds.y + first_height,
                            width: bounds.width,
                            height: bounds.height - first_height,
                        },
                    );
                }
            },
            LayoutNode::Tabs { active, panes } => {
                if let Some(pane_id) = panes.get(*active) {
                    self.pane_bounds.insert(
                        pane_id.clone(),
                        PaneBounds {
                            x: bounds.x,
                            y: bounds.y,
                            width: bounds.width,
                            height: bounds.height,
                        },
                    );
                }
            }
        }
    }

    pub fn get_pane_bounds(&self, pane_id: &str) -> Option<&PaneBounds> {
        self.pane_bounds.get(pane_id)
    }

    pub fn get_all_pane_bounds(&self) -> &HashMap<String, PaneBounds> {
        &self.pane_bounds
    }

    pub fn focus_pane(&mut self, pane_id: String) {
        self.focused_pane = Some(pane_id);
    }

    pub fn get_focused_pane(&self) -> Option<&String> {
        self.focused_pane.as_ref()
    }

    pub fn split_current(&mut self, direction: SplitDirection, new_pane_id: String) {
        if let Some(focused) = self.focused_pane.clone() {
            self.root = self.split_node(self.root.clone(), &focused, direction, new_pane_id);
        }
    }

    fn split_node(
        &self,
        node: LayoutNode,
        target_id: &str,
        direction: SplitDirection,
        new_pane_id: String,
    ) -> LayoutNode {
        match node {
            LayoutNode::Leaf(ref pane_id) if pane_id == target_id => LayoutNode::Split {
                direction,
                ratio: 0.5,
                first: Box::new(node),
                second: Box::new(LayoutNode::Leaf(new_pane_id)),
            },
            LayoutNode::Split {
                direction: d,
                ratio: r,
                first,
                second,
            } => LayoutNode::Split {
                direction: d,
                ratio: r,
                first: Box::new(self.split_node(*first, target_id, direction, new_pane_id.clone())),
                second: Box::new(self.split_node(*second, target_id, direction, new_pane_id)),
            },
            _ => node,
        }
    }

    pub fn handle_click(&mut self, x: f32, y: f32) -> Option<String> {
        for (pane_id, bounds) in &self.pane_bounds {
            if x >= bounds.x
                && x <= bounds.x + bounds.width
                && y >= bounds.y
                && y <= bounds.y + bounds.height
            {
                self.focused_pane = Some(pane_id.clone());
                return Some(pane_id.clone());
            }
        }
        None
    }
}
