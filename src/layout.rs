use crate::constants::DEFAULT_FONT_SIZE;
use crate::css::Unit::Px;
use crate::dom::NodeType::{self, Element};
use crate::layout::BoxType::BlockNode;

use super::css::{Unit, Value};
use super::style::{Display, StyleNode};

#[derive(Default, Clone, Copy)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Default, Clone, Copy)]
pub struct Dimensions {
    pub content: Rect,
    padding: EdgeSizes,
    margin: EdgeSizes,
    pub border: EdgeSizes,
}

#[derive(Default, Clone, Copy)]
pub struct EdgeSizes {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

pub struct LayoutBox<'a> {
    pub dimension: Dimensions,
    pub box_type: BoxType<'a>,
    pub children: Vec<LayoutBox<'a>>,
}

pub enum BoxType<'a> {
    InlineNode(&'a StyleNode<'a>),
    BlockNode(&'a StyleNode<'a>),
    TextNode(&'a StyleNode<'a>, &'a String),
    AnonymousBlock,
}

pub fn layout_tree<'a>(
    style_node: &'a StyleNode<'a>,
    mut containing_block: Dimensions,
    font: &fontdue::Font,
) -> LayoutBox<'a> {
    containing_block.content.height = 0.0;

    let mut root = build_layout_tree(style_node);
    root.layout(containing_block, font);

    root
}

fn build_layout_tree<'a>(style_node: &'a StyleNode<'a>) -> LayoutBox<'a> {
    let mut root = LayoutBox::new(match style_node.display() {
        Display::Block => match &style_node.node.node_type {
            NodeType::Text(text) => BoxType::TextNode(style_node, text),
            _ => BlockNode(style_node),
        },
        Display::Inline => BoxType::InlineNode(style_node),
        Display::None => panic!("Root node has none display"),
    });

    // Do not need to explore the children for text boxes
    if matches!(root.box_type, BoxType::TextNode(..)) {
        return root;
    }

    for child in &style_node.children {
        match child.display() {
            Display::Block => root.children.push(build_layout_tree(child)),
            Display::Inline => root
                .get_inline_container()
                .children
                .push(build_layout_tree(child)),
            Display::None => {}
        }
    }

    root
}

impl<'a> LayoutBox<'a> {
    fn new(box_type: BoxType<'a>) -> Self {
        LayoutBox {
            dimension: Default::default(),
            box_type,
            children: vec![],
        }
    }

    fn get_style_node(&self) -> &StyleNode<'_> {
        match self.box_type {
            BoxType::BlockNode(node) | BoxType::InlineNode(node) | BoxType::TextNode(node, _) => {
                node
            }
            BoxType::AnonymousBlock => panic!("Anonymous block box has no style node"),
        }
    }

    fn get_inline_container(&mut self) -> &mut Self {
        match self.box_type {
            BoxType::InlineNode(_) | BoxType::AnonymousBlock => self,
            BoxType::BlockNode(_) => {
                match self.children.last() {
                    Some(&LayoutBox {
                        box_type: BoxType::AnonymousBlock,
                        ..
                    }) => {}
                    _ => self.children.push(LayoutBox::new(BoxType::AnonymousBlock)),
                }
                self.children.last_mut().unwrap()
            }
            _ => unreachable!(),
        }
    }

    fn layout(&mut self, containing_block: Dimensions, font: &fontdue::Font) {
        match self.box_type {
            BoxType::BlockNode(_) => self.layout_block(containing_block, font),
            BoxType::TextNode(_, ..) => self.layout_text(containing_block, font),
            BoxType::InlineNode(_) | BoxType::AnonymousBlock => {} //TODO:,
        }
    }

    fn layout_block(&mut self, containing_block: Dimensions, font: &fontdue::Font) {
        self.calculate_block_width(containing_block);
        self.calculate_block_position(containing_block);
        self.layout_block_children(font);
        self.calculate_block_height();
    }

    fn calculate_block_width(&mut self, containing_block: Dimensions) {
        let style = self.get_style_node();

        let auto = Value::Keyword("auto".to_string());
        let mut width = style.value("width").unwrap_or_else(|| {
            // handling the case where the width attribute is set
            // usually for img tags
            let width_attr = match self.box_type {
                BlockNode(style_node) => match &style_node.node.node_type {
                    Element(element) => element.get_attr("width"),
                    _ => None,
                },
                _ => None,
            };
            if let Some(width) = width_attr {
                let w = width.parse::<usize>().unwrap();
                Value::Length(w as f32, Px)
            } else {
                auto.clone()
            }
        });

        let zero = Value::Length(0.0, Unit::Px);

        let mut margin_left = style.lookup("margin-left", "margin", &zero);
        let mut margin_right = style.lookup("margin-right", "margin", &zero);

        let border_left = style.lookup("border-left-width", "border-width", &zero);
        let border_right = style.lookup("border-right-width", "border-width", &zero);

        let padding_left = style.lookup("padding-left", "padding", &zero);
        let padding_right = style.lookup("padding-right", "padding", &zero);

        let total = sum([
            &margin_left,
            &margin_right,
            &border_left,
            &border_right,
            &padding_left,
            &padding_right,
            &width,
        ]
        .iter()
        .map(|v| v.to_px()));

        if width != auto && total > containing_block.content.width {
            if margin_left == auto {
                margin_left = Value::Length(0.0, Unit::Px);
            }
            if margin_right == auto {
                margin_right = Value::Length(0.0, Unit::Px);
            }
        }

        let underflow = containing_block.content.width - total;

        match (width == auto, margin_left == auto, margin_right == auto) {
            (false, false, false) => {
                margin_right = Value::Length(margin_right.to_px() + underflow, Unit::Px)
            }
            (false, false, true) => margin_right = Value::Length(underflow, Unit::Px),
            (false, true, false) => margin_left = Value::Length(underflow, Unit::Px),
            (true, _, _) => {
                if margin_left == auto {
                    margin_left = Value::Length(0.0, Unit::Px);
                }

                if margin_right == auto {
                    margin_right = Value::Length(0.0, Unit::Px);
                }

                if underflow >= 0.0 {
                    width = Value::Length(underflow, Unit::Px);
                } else {
                    width = Value::Length(0.0, Unit::Px);
                    margin_right = Value::Length(margin_right.to_px() + underflow, Unit::Px);
                }
            }
            (false, true, true) => {
                margin_left = Value::Length(underflow / 2.0, Unit::Px);
                margin_right = Value::Length(underflow / 2.0, Unit::Px);
            }
        }

        let d = &mut self.dimension;

        d.content.width = width.to_px();

        d.border.left = border_left.to_px();
        d.border.right = border_right.to_px();

        d.padding.left = padding_left.to_px();
        d.padding.right = padding_right.to_px();

        d.margin.left = margin_left.to_px();
        d.margin.right = margin_right.to_px();
    }

    fn calculate_block_position(&mut self, containing_block: Dimensions) {
        let style = self.get_style_node();

        let zero = Value::Length(0.0, Unit::Px);

        let margin_top = style.lookup("margin-top", "margin", &zero).to_px();
        let margin_bottom = style.lookup("margin-bottom", "margin", &zero).to_px();

        let border_top = style
            .lookup("border-top-width", "border-width", &zero)
            .to_px();
        let border_bottom = style
            .lookup("border-bottom-width", "border-width", &zero)
            .to_px();

        let padding_top = style.lookup("padding-top", "padding", &zero).to_px();
        let padding_bottom = style.lookup("padding-bottom", "padding", &zero).to_px();

        let d = &mut self.dimension;
        d.border.top = border_top;
        d.border.bottom = border_bottom;

        d.margin.top = margin_top;
        d.margin.bottom = margin_bottom;

        d.padding.top = padding_top;
        d.padding.bottom = padding_bottom;

        d.content.x = containing_block.content.x + d.border.left + d.margin.left + d.padding.left;

        d.content.y = containing_block.content.y
            + containing_block.content.height
            + d.border.top
            + d.padding.top
            + d.margin.top;
    }

    fn layout_block_children(&mut self, font: &fontdue::Font) {
        for child in &mut self.children {
            child.layout(self.dimension, font);
            self.dimension.content.height += child.dimension.margin_box().height;
        }
    }

    fn calculate_block_height(&mut self) {
        if let Some(Value::Length(h, Unit::Px)) = self.get_style_node().value("height") {
            self.dimension.content.height = h;
        } else {
            // handling the case where the height attribute is set
            // usually for img tags
            let height_attr = match self.box_type {
                BlockNode(style_node) => match &style_node.node.node_type {
                    Element(element) => element.get_attr("height"),
                    _ => None,
                },
                _ => None,
            };
            if let Some(height) = height_attr {
                let h = height.parse::<usize>().unwrap();
                self.dimension.content.height = h as f32;
            }
        }
    }

    fn layout_text(&mut self, containing_block: Dimensions, font: &fontdue::Font) {
        self.calculate_text_span(font);
        // same as block version
        self.calculate_block_position(containing_block);
    }

    fn calculate_text_span(&mut self, font: &fontdue::Font) {
        let style = self.get_style_node();

        let text = match self.box_type {
            BoxType::TextNode(_, text) => text,
            _ => unreachable!(),
        };

        let font_size = match style.value("font-size") {
            Some(Value::Length(s, Unit::Px)) => s,
            _ => DEFAULT_FONT_SIZE,
        };

        let zero = Value::Length(0.0, Unit::Px);

        let width = text
            .chars()
            .map(|c| font.metrics(c, font_size).advance_width)
            .sum::<f32>();

        let height = font
            .vertical_line_metrics(font_size)
            .map(|m| m.new_line_size)
            .unwrap_or(font_size);

        let margin_left = style.lookup("margin-left", "margin", &zero).to_px();
        let margin_right = style.lookup("margin-right", "margin", &zero).to_px();

        let border_left = style
            .lookup("border-left-width", "border-width", &zero)
            .to_px();
        let border_right = style
            .lookup("border-right-width", "border-width", &zero)
            .to_px();

        let padding_left = style.lookup("padding-left", "padding", &zero).to_px();
        let padding_right = style.lookup("padding-right", "padding", &zero).to_px();

        let d = &mut self.dimension;

        d.margin.left = margin_left;
        d.margin.right = margin_right;

        d.border.left = border_left;
        d.border.right = border_right;

        d.padding.left = padding_left;
        d.padding.right = padding_right;

        d.content.width = width;
        d.content.height = height;
    }
}

impl Dimensions {
    pub fn padding_box(&self) -> Rect {
        self.content.expanded_by(&self.padding)
    }

    pub fn border_box(&self) -> Rect {
        self.padding_box().expanded_by(&self.border)
    }

    pub fn margin_box(&self) -> Rect {
        self.border_box().expanded_by(&self.margin)
    }
}

impl Rect {
    fn expanded_by(&self, edge: &EdgeSizes) -> Rect {
        Rect {
            x: self.x - edge.left,
            y: self.y - edge.top,
            width: self.width + edge.left + edge.right,
            height: self.height + edge.top + edge.bottom,
        }
    }
}

fn sum<I>(iter: I) -> f32
where
    I: Iterator<Item = f32>,
{
    iter.sum()
}
