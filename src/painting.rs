use super::{HEIGHT, WIDTH};

use crate::constants::{BLACK, ROBOTO};
use crate::css::Unit::Px;
use crate::css::{self, Value};
use crate::dom::NodeType::Element;
use crate::image::{get_components, load_image};
use crate::{html, layout, style};

use super::css::Color;
use super::layout::{BoxType, LayoutBox, Rect};

type DisplayList = Vec<DisplayCommand>;

enum DisplayCommand {
    SolidColor(Color, Rect),
    Text(String, Rect, Color, f32),
    Image(Rect, Vec<u8>),
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
    render_image(list, layout_box);

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
    let font_size = match layout_box.box_type {
        BoxType::TextNode(style, _) => style
            .value("font-size")
            .unwrap_or(Value::Length(16.0, Px))
            .to_px(),
        _ => return,
    };

    list.push(DisplayCommand::Text(
        text.clone(),
        layout_box.dimension.border_box(),
        color,
        font_size,
    ));
}

fn render_image(list: &mut DisplayList, layout_box: &LayoutBox) {
    let element = match layout_box.box_type {
        BoxType::BlockNode(style) => match &style.node.node_type {
            Element(element) => element,
            _ => return,
        },
        _ => return,
    };

    let src = match element.get_attr("src") {
        Some(src) => src,
        None => return,
    };

    // getting the properties needed for image rendering
    let img_width = layout_box.dimension.border_box().width;
    let img_height = layout_box.dimension.border_box().height;

    let colors = load_image(src, img_width, img_height);

    list.push(DisplayCommand::Image(
        layout_box.dimension.border_box(),
        colors,
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
                let line_metrics = font.horizontal_line_metrics(*size).unwrap();
                let baseline_y = rect.y + line_metrics.ascent;

                let mut pen_x = rect.x;
                for ch in text.chars() {
                    let (metrics, bitmap) = font.rasterize(ch, *size);

                    let glyph_top = baseline_y - metrics.ymin as f32 - metrics.height as f32;

                    for y in 0..metrics.height {
                        for x in 0..metrics.width {
                            let coverage = bitmap[y * metrics.width + x];
                            if coverage == 0 {
                                continue;
                            }

                            let px = pen_x as usize + metrics.xmin as usize + x;
                            let py = glyph_top as usize + y;
                            let bg = self.pixels[px + py * self.width];
                            let blended = color.get_blended_color_with_coverage(bg, coverage);
                            self.pixels[px + py * self.width] = blended;
                        }
                    }
                    pen_x += metrics.advance_width;
                }
            }
            &DisplayCommand::Image(rect, colors) => {
                let x0 = rect.x.clamp(0.0, self.width as f32) as usize;
                let y0 = rect.y.clamp(0.0, self.height as f32) as usize;
                let x1 = (rect.x + rect.width).clamp(0.0, self.width as f32) as usize;
                let y1 = (rect.y + rect.height).clamp(0.0, self.height as f32) as usize;

                for y in y0..y1 {
                    for x in x0..x1 {
                        let background_color = self.pixels[x + y * self.width];

                        let pixel = (x - x0) + (y - y0) * rect.width as usize;
                        let color = get_components(colors, pixel);

                        let result = color.get_blended_color(background_color);

                        self.pixels[x + y * self.width] = result;
                    }
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
    let mut style_root = style::style_tree(&root_node, &stylesheet);

    let layout_root = layout::layout_tree(&mut style_root, viewport, &font);

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
