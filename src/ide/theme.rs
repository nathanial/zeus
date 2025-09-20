use raylib::prelude::*;

#[derive(Clone)]
pub struct Theme {
    pub background: Color,
    pub surface: Color,
    pub panel: Color,
    pub border: Color,
    pub text: Color,
    pub text_dim: Color,
    pub text_highlight: Color,
    pub selection: Color,
    pub cursor: Color,
    pub success: Color,
    pub error: Color,
    pub warning: Color,
    pub info: Color,
    pub keyword: Color,
    pub string: Color,
    pub number: Color,
    pub comment: Color,
    pub function: Color,
    pub macro_color: Color,
    pub special_form: Color,
    pub paren_match: Color,
    pub paren_mismatch: Color,
    pub focus_indicator: Color,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            background: Color::new(25, 25, 35, 255),
            surface: Color::new(35, 35, 45, 255),
            panel: Color::new(30, 30, 40, 255),
            border: Color::new(60, 60, 70, 255),
            text: Color::new(220, 220, 220, 255),
            text_dim: Color::new(150, 150, 150, 255),
            text_highlight: Color::new(255, 255, 255, 255),
            selection: Color::new(50, 100, 150, 128),
            cursor: Color::new(255, 255, 255, 255),
            success: Color::new(100, 255, 100, 255),
            error: Color::new(255, 100, 100, 255),
            warning: Color::new(255, 200, 100, 255),
            info: Color::new(100, 200, 255, 255),
            keyword: Color::new(200, 100, 255, 255),
            string: Color::new(100, 255, 150, 255),
            number: Color::new(255, 200, 100, 255),
            comment: Color::new(100, 100, 100, 255),
            function: Color::new(100, 150, 255, 255),
            macro_color: Color::new(255, 150, 100, 255),
            special_form: Color::new(255, 100, 200, 255),
            paren_match: Color::new(100, 255, 100, 255),
            paren_mismatch: Color::new(255, 100, 100, 255),
            focus_indicator: Color::new(100, 150, 255, 255),
        }
    }

    pub fn light() -> Self {
        Self {
            background: Color::new(245, 245, 245, 255),
            surface: Color::new(255, 255, 255, 255),
            panel: Color::new(250, 250, 250, 255),
            border: Color::new(200, 200, 200, 255),
            text: Color::new(30, 30, 30, 255),
            text_dim: Color::new(100, 100, 100, 255),
            text_highlight: Color::new(0, 0, 0, 255),
            selection: Color::new(100, 150, 200, 128),
            cursor: Color::new(0, 0, 0, 255),
            success: Color::new(0, 150, 0, 255),
            error: Color::new(200, 0, 0, 255),
            warning: Color::new(200, 100, 0, 255),
            info: Color::new(0, 100, 200, 255),
            keyword: Color::new(150, 0, 200, 255),
            string: Color::new(0, 150, 50, 255),
            number: Color::new(200, 100, 0, 255),
            comment: Color::new(150, 150, 150, 255),
            function: Color::new(0, 50, 200, 255),
            macro_color: Color::new(200, 50, 0, 255),
            special_form: Color::new(200, 0, 100, 255),
            paren_match: Color::new(0, 200, 0, 255),
            paren_mismatch: Color::new(255, 0, 0, 255),
            focus_indicator: Color::new(0, 100, 200, 255),
        }
    }
}
