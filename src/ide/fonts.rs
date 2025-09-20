use crate::fonts::load_monospace_font;
use raylib::consts::TextureFilter;
use raylib::core::drawing::RaylibDraw;
use raylib::core::text::{RaylibFont, WeakFont};
use raylib::prelude::*;

pub const IDE_FONT_SPACING: f32 = 1.0;

pub struct IdeFonts {
    custom: Option<Font>,
    fallback: WeakFont,
    spacing: f32,
}

impl IdeFonts {
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread, atlas_base_size: f32) -> Self {
        let dpi_scale = rl.get_window_scale_dpi();
        let atlas_scale = dpi_scale.x.max(dpi_scale.y).max(1.0);
        let atlas_font_size = (atlas_base_size * atlas_scale).round() as i32;

        let custom = load_monospace_font(rl, thread, atlas_font_size);
        let fallback = rl.get_font_default();
        fallback
            .texture()
            .set_texture_filter(thread, TextureFilter::TEXTURE_FILTER_POINT);

        Self {
            custom,
            fallback,
            spacing: IDE_FONT_SPACING,
        }
    }

    pub fn measure_text(&self, text: &str, font_size: f32) -> Vector2 {
        match self.custom.as_ref() {
            Some(font) => font.measure_text(text, font_size, self.spacing),
            None => self.fallback.measure_text(text, font_size, self.spacing),
        }
    }

    pub fn draw_text<T: RaylibDraw>(
        &self,
        target: &mut T,
        text: &str,
        position: Vector2,
        font_size: f32,
        color: Color,
    ) {
        if let Some(font) = self.custom.as_ref() {
            target.draw_text_ex(font, text, position, font_size, self.spacing, color);
        } else {
            target.draw_text_ex(
                &self.fallback,
                text,
                position,
                font_size,
                self.spacing,
                color,
            );
        }
    }
}
