use super::{HEIGHT, WIDTH};

use crate::constants::{BLACK, ROBOTO};
use crate::css::{self, Value};
use crate::{html, layout, style};

use super::css::Color;
use super::layout::{BoxType, LayoutBox, Rect};

type DisplayList = Vec<DisplayCommand>;

enum DisplayCommand {
    SolidColor(Color, Rect),
    Text(String, Rect, Color, f32),
}

fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
    let mut list = Vec::new();
    render_layout_box(&mut list, layout_root);
    list
}

fn render_layout_box(list: &mut DisplayList, layout_box: &LayoutBox) {
    render_background(list, layout_box);
    render_borders(list, layout_box);

    redner_text(list, layout_box);

    for child in layout_box.children.iter() {
        render_layout_box(list, &child);
    }
}

fn render_background(list: &mut DisplayList, layout_box: &LayoutBox) {
    let color =
        get_color("background-color", layout_box).or_else(|| get_color("background", layout_box));

    if let Some(color) = color {
        list.push(DisplayCommand::SolidColor(
            color,
            layout_box.dimension.border_box(),
        ));
    }
}

fn render_borders(list: &mut DisplayList, layout_box: &LayoutBox) {
    let color = match get_color("border-color", layout_box) {
        Some(color) => color,
        None => return,
    };

    let d = &layout_box.dimension;
    let border_box = d.border_box();

    // left
    list.push(DisplayCommand::SolidColor(
        color,
        Rect {
            x: border_box.x,
            y: border_box.y,
            width: d.border.left,
            height: border_box.height,
        },
    ));

    // right
    list.push(DisplayCommand::SolidColor(
        color,
        Rect {
            x: border_box.x + border_box.width - d.border.right,
            y: border_box.y,
            width: d.border.right,
            height: border_box.height,
        },
    ));

    // top
    list.push(DisplayCommand::SolidColor(
        color,
        Rect {
            x: border_box.x,
            y: border_box.y,
            width: border_box.width,
            height: d.border.top,
        },
    ));

    // bottom
    list.push(DisplayCommand::SolidColor(
        color,
        Rect {
            x: border_box.x,
            y: border_box.y + border_box.height - d.border.bottom,
            width: border_box.width,
            height: d.border.bottom,
        },
    ));
}

fn redner_text(list: &mut DisplayList, layout_box: &LayoutBox) {
    let color = get_color("color", layout_box);

    
    let text = match layout_box.box_type {
        BoxType::TextNode(_, text) => text,
        _ => return,
    };
    
    let color = color.unwrap_or(BLACK);

    list.push(DisplayCommand::Text(
        text.clone(),
        layout_box.dimension.border_box(),
        color,
        // TODO: font size should be changed here
        16.0,
    ));
}

fn get_color(name: &str, layout_box: &LayoutBox) -> Option<Color> {
    match layout_box.box_type {
        BoxType::BlockNode(style) | BoxType::InlineNode(style) | BoxType::TextNode(style, _) => {
            match style.value(name) {
                Some(Value::ColorValue(color)) => Some(color),
                _ => None,
            }
        }
        BoxType::AnonymousBlock => None,
    }
}

pub struct Canvas {
    pub pixels: Vec<Color>,
    pub width: usize,
    pub height: usize,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let white: Color = Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        };
        Canvas {
            pixels: vec![white; width * height],
            width,
            height,
        }
    }

    fn paint_item(&mut self, item: &DisplayCommand, font: &fontdue::Font) {
        match &item {
            &DisplayCommand::SolidColor(foreground_color, rect) => {
                let x0 = rect.x.clamp(0.0, self.width as f32) as usize;
                let y0 = rect.y.clamp(0.0, self.height as f32) as usize;
                let x1 = (rect.x + rect.width).clamp(0.0, self.width as f32) as usize;
                let y1 = (rect.y + rect.height).clamp(0.0, self.height as f32) as usize;

                for y in y0..y1 {
                    for x in x0..x1 {
                        let background_color = self.pixels[x + y * self.width];

                        let result = foreground_color.get_blended_color(background_color);

                        self.pixels[x + y * self.width] = result;
                    }
                }
            }
            &DisplayCommand::Text(text, rect, color, size) => {
                let mut pen_x = rect.x;
                for ch in text.chars() {
                    let (metrics, bitmap) = font.rasterize(ch, *size);
                    for y in 0..metrics.height {
                        for x in 0..metrics.width {
                            let coverage = bitmap[y * metrics.width + x];
                            if coverage == 0 {
                                continue;
                            }

                            let px = pen_x as usize + metrics.xmin as usize + x;
                            let py = (rect.y + metrics.ymin as f32) as usize + y;
                            let bg = self.pixels[px + py * self.width];
                            let blended = color.get_blended_color(bg);
                            self.pixels[px + py * self.width] = blended;
                        }
                    }
                    pen_x += metrics.advance_width;
                }
            }
        }
    }
}

pub fn get_canvas(html: String, css: String) -> Canvas {
    let mut viewport: layout::Dimensions = Default::default();
    viewport.content.width = WIDTH as f32;
    viewport.content.height = HEIGHT as f32;

    let font = fontdue::Font::from_bytes(ROBOTO, fontdue::FontSettings::default()).unwrap();

    let root_node = html::parse(html);
    let stylesheet = css::parse(css);
    let style_root = style::style_tree(&root_node, &stylesheet);
    let layout_root = layout::layout_tree(&style_root, viewport, &font);

    paint(&layout_root, viewport.content, &font)
}

fn paint(layout_root: &LayoutBox, bounds: Rect, font: &fontdue::Font) -> Canvas {
    let display_list = build_display_list(layout_root);
    let mut canvas = Canvas::new(bounds.width as usize, bounds.height as usize);

    for item in display_list {
        canvas.paint_item(&item, &font);
    }

    canvas
}
