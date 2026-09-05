#![allow(clippy::all, clippy::pedantic, dead_code)]

use anyhow::Result;

mod cli;
mod constants;
mod css;
mod dom;
mod html;
mod image;
mod layout;
mod painting;
mod style;
mod watch;
mod window;

fn main() -> Result<()> {
    let (html_path, css_path) = cli::parse_args();

    window::create_window(html_path.into(), css_path.into())
}
