#![allow(clippy::all, clippy::pedantic)]
extern crate getopts;

use std::{fs::{self, File}, io::BufWriter};
use image::ImageBuffer;

mod dom;
mod html;
mod css;
mod style;
mod layout;
mod painting;

fn main() {
    let mut opts = getopts::Options::new();
    opts.optopt("h", "html", "HTML document", "FILENAME");
    opts.optopt("c", "css", "CSS stylesheet", "FILENAME");
    opts.optopt("o", "output", "Output file", "FILENAME");
    opts.optopt("f", "format", "Output file format", "png");

    let matches = opts.parse(std::env::args().skip(1))
        .expect("Couldn't parse the args");
    let str_arg = |flag: &str, default: &str| -> String {
        matches.opt_str(flag).unwrap_or(default.to_string())
    };

    let html = read_source(str_arg("h", "./test.html"));
    let css = read_source(str_arg("c", "./test.css"));

    let mut viewport: layout::Dimensions = Default::default();
    viewport.content.width = 800.0;
    viewport.content.height = 600.0;

    let root_node = html::parse(html);
    let stylesheet = css::parse(css);
    let style_root = style::style_tree(&root_node, &stylesheet);
    let layout_root = layout::layout_tree(&style_root, viewport);

    let file_name = str_arg("o", "./output.png");
    let mut file = BufWriter::new(File::create(&file_name)
        .expect("Failed to create a bufwriter"));

    
    let canvas = painting::paint(&layout_root, viewport.content);
    let (w, h) = (canvas.width as u32, canvas.height as u32);
    let img = ImageBuffer::from_fn(w, h, |x, y| {
        let color = canvas.pixels[(x + y * w) as usize];
        image::Rgba([color.r, color.g, color.b, color.a])
    });
    let result = image::DynamicImage::ImageRgba8(img).write_to(&mut file, image::ImageFormat::Png);
    if result.is_ok(){
        println!("Saved the output to {file_name}");
    }else {
        println!("Failed saving the output to {file_name}");
    }



}


fn read_source(file_name: String) -> String {
    fs::read_to_string(file_name)
        .expect("Couldn't read the source")
}