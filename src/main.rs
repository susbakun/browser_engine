#![allow(clippy::all, clippy::pedantic, dead_code)]

use anyhow::Result;

use crate::painting::get_canvas;

mod cli;
mod css;
mod dom;
mod html;
mod layout;
mod painting;
mod style;
mod window;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;

fn main() -> Result<()> {
    let (html, css) = cli::parse_args();

    let canvas = get_canvas(html, css);

    window::create_window(&canvas)
}
