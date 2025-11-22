use crate::css::Value;

use super::css::Color;
use super::layout::{LayoutBox, Rect, BoxType};

type DisplayList = Vec<DisplayCommand>;

enum DisplayCommand {
    SolidColor(Color, Rect)
}

fn build_display_list(layout_root: &LayoutBox) -> DisplayList {
    let mut list = Vec::new();
    render_layout_box(&mut list, layout_root);
    list
}

fn render_layout_box(list: &mut DisplayList, layout_box: &LayoutBox) {
    render_background(list, layout_box);
    render_borders(list ,layout_box);

    for child in layout_box.children.iter(){
        render_layout_box(list, &child);
    }
}

fn render_background(list: &mut DisplayList, layout_box: &LayoutBox) {
    let color = get_color("background-color", layout_box)
        .or_else(|| get_color("background", layout_box));
    
    if let Some(color) = color {
        list.push(DisplayCommand::SolidColor(color, layout_box.dimension.border_box()));
    }
}

fn render_borders(list: &mut DisplayList, layout_box: &LayoutBox) {
    let color = match get_color("border-color", layout_box){
        Some(color) => color,
        None => return
    };

    let d = &layout_box.dimension;
    let border_box = d.border_box();

    // left
    list.push(DisplayCommand::SolidColor(color, Rect {
        x: border_box.x,
        y: border_box.y,
        width: d.border.left,
        height: border_box.height
    }));

    // right
    list.push(DisplayCommand::SolidColor(color, Rect {
        x: border_box.x + border_box.width - d.border.right,
        y: border_box.y,
        width: d.border.right,
        height: border_box.height
    }));

    // top
    list.push(DisplayCommand::SolidColor(color, Rect {
        x: border_box.x,
        y: border_box.y,
        width: border_box.width,
        height: d.border.top
    }));

    // bottom
    list.push(DisplayCommand::SolidColor(color, Rect {
        x: border_box.x,
        y: border_box.y + border_box.height - d.border.bottom,
        width: border_box.width,
        height: d.border.bottom
    }));
}

fn get_color(name: &str, layout_box: &LayoutBox) -> Option<Color> {
    match layout_box.box_type{
        BoxType::BlockNode(style) | BoxType::InlineNode(style) => match style.value(name){
            Some(Value::ColorValue(color)) => Some(color),
            _ => None
        }
        BoxType::AnonymousBlock => None
    }
}


pub struct Canvas {
    pub pixels: Vec<Color>,
    pub width: usize,
    pub height: usize
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let white: Color = Color{r: 255, g: 255, b: 255, a: 255};
        Canvas { 
            pixels: vec![white; width * height], 
            width, 
            height
        }
    }

    fn paint_item(&mut self, item: &DisplayCommand) {
        match item {
            & DisplayCommand::SolidColor(color, rect) => {
                let x0 = rect.x.clamp(0.0, self.width as f32) as usize;
                let y0 = rect.y.clamp(0.0, self.height as f32) as usize;
                let x1 = (rect.x + rect.width).clamp(0.0, self.width as f32) as usize;
                let y1 = (rect.y + rect.height).clamp(0.0, self.height as f32) as usize;


                for y in y0..y1{
                    for x in x0..x1 {
                        self.pixels[x + y * self.width] = color;
                    }
                }
            }
        }
    }
}

pub fn paint(layout_root: &LayoutBox, bounds: Rect) -> Canvas {
    let display_list = build_display_list(layout_root);
    let mut canvas = Canvas::new(bounds.width as usize, bounds.height as usize);

    for item in display_list {
        canvas.paint_item(&item);
    }

    canvas
}