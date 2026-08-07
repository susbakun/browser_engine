use crate::css::Color;

// fonts
pub const ROBOTO: &[u8] = include_bytes!("../resources/fonts/Roboto-Regular.ttf") as &[u8];
pub const DEFAULT_FONT_SIZE: f32 = 16.0;

// colors
pub const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
};
pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};

pub const SELF_CLOSING_TAGS: [&'static str; 3] = ["img", "br", "input"];
