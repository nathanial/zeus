use raylib::consts::TextureFilter;
use raylib::prelude::*;
use std::path::Path;

const FONT_PATH_CANDIDATES: [&str; 5] = [
    "/System/Applications/Utilities/Terminal.app/Contents/Resources/Fonts/SF-Mono-Regular.otf",
    "/System/Library/Fonts/SF-Mono-Regular.otf",
    "/System/Library/Fonts/SFMono-Regular.otf",
    "/Library/Fonts/Menlo.ttc",
    "/System/Library/Fonts/Menlo.ttc",
];

pub fn load_monospace_font(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    base_size: i32,
) -> Option<Font> {
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
