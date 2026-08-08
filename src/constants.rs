use crate::css::Color;

// window
pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

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

// default styles
pub const UA_STYLESHEET: &str = r#"
    head {
        display: none;
    }

    * {
        display: block;
        color: #000000;
    }

    p {
        margin-top: 16px;
        margin-bottom: 16px;
    }

    h1 {
        font-size: 32px;
        font-weight: bold;
        margin-top: 11px;
        margin-bottom: 11px;
    }

    h2 {
        font-size: 24px;
        font-weight: bold;
        margin-top: 13px;
        margin-bottom: 13px;
    }

    h3 {
        font-size: 19px;
        font-weight: bold;
        margin-top: 16px;
        margin-bottom: 16px;
    }

    h4 {
        font-size: 16px;
        font-weight: bold;
        margin-top: 21px;
        margin-bottom: 21px;
    }

    h5 {
        font-size: 13px;
        font-weight: bold;
        margin-top: 27px;
        margin-bottom: 27px;
    }

    h6 {
        font-size: 11px;
        font-weight: bold;
        margin-top: 37px;
        margin-bottom: 37px;
    }
"#;
