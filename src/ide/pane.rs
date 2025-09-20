use raylib::prelude::*;
use std::any::Any;

pub trait Pane {
    fn id(&self) -> &str;
    fn title(&self) -> &str;
    fn draw(&mut self, d: &mut RaylibDrawHandle, bounds: Rectangle, theme: &crate::ide::theme::Theme);
    fn handle_input(&mut self, rl: &mut RaylibHandle, bounds: Rectangle) -> bool;
    fn is_focusable(&self) -> bool { true }
    fn on_focus(&mut self) {}
    fn on_blur(&mut self) {}
    fn preferred_size(&self) -> (Option<f32>, Option<f32>) { (None, None) }
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

#[derive(Debug, Clone)]
pub struct PaneBounds {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl PaneBounds {
    pub fn to_rectangle(&self) -> Rectangle {
        Rectangle {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}