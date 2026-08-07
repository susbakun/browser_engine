use image::{ImageReader, imageops::FilterType::Triangle};

use crate::css::Color;

pub fn load_image(src: &String, img_width: f32, img_height: f32) -> Vec<u8> {
    let img = match ImageReader::open(src) {
        Ok(img) => img,
        Err(err) => {
            eprintln!("error opening the image: {err}");
            return vec![];
        }
    };

    match img.decode() {
        Ok(dec_img) => dec_img
            .resize_exact(img_width as u32, img_height as u32, Triangle)
            .to_rgba8()
            .to_vec(),
        Err(err) => {
            eprintln!("error decoding the image: {err}");
            return vec![];
        }
    }
}

pub fn get_components(colors: &Vec<u8>, pixel: usize) -> Color {
    let index = pixel * 4;

    let r = colors[index];
    let g = colors[index + 1];
    let b = colors[index + 2];
    let a = colors[index + 3];

    Color { r, g, b, a }
}
