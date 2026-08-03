use crate::css::Color;

// fonts
pub const ROBOTO: &[u8] = include_bytes!("../resources/Roboto-Regular.ttf") as &[u8];

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
