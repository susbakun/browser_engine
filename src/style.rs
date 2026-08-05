use crate::constants::WHITE;

use super::css::Value;
use super::css::{Parser as CssParser, Rule, Selector, SimpleSelector, Specificity, Stylesheet};
use super::dom::{ElementData, Node, NodeType};
use std::collections::HashMap;

type PropertyMap = HashMap<String, Value>;

#[derive(Clone)]
pub struct StyleNode<'a> {
    pub node: &'a Node,
    specified_values: PropertyMap,
    pub children: Vec<StyleNode<'a>>,
}

pub enum Display {
    Inline,
    Block,
    None,
}

impl<'a> StyleNode<'a> {
    pub fn value(&self, name: &str) -> Option<Value> {
        self.specified_values.get(name).map(|value| value.clone())
    }

    pub fn lookup(&self, name: &str, fallback_name: &str, default: &Value) -> Value {
        self.value(name)
            .unwrap_or_else(|| self.value(fallback_name).unwrap_or_else(|| default.clone()))
    }

    pub fn display(&self) -> Display {
        match self.value("display") {
            Some(Value::Keyword(s)) => match s.as_str() {
                "block" => Display::Block,
                "none" => Display::None,
                _ => Display::Inline,
            },
            _ => Display::Inline,
        }
    }

    pub fn apply_inherit_styles(&mut self) {
        let self_clone = self.clone();
        let inherit_keyword = String::from("inherit");

        for child in self.children.iter_mut() {
            for (key, value) in child.specified_values.iter_mut() {
                match value {
                    Value::Keyword(kw) if *kw == inherit_keyword => match key.as_str() {
                        "display" => *value = Value::Keyword("block".to_string()),
                        "color" => *value = self_clone.value("color").unwrap_or(WHITE.into()),
                        "background-color" => {
                            *value =
                                self_clone.lookup("background_color", "background", &WHITE.into())
                        }
                        _ => continue,
                    },
                    _ => continue,
                };
            }

            child.apply_inherit_styles();
        }
    }
}

fn matches(element: &ElementData, selector: &Selector) -> bool {
    match selector {
        Selector::Simple(s) => matches_simple_selector(element, s),
    }
}

fn matches_simple_selector(element: &ElementData, selector: &SimpleSelector) -> bool {
    if selector
        .tag_name
        .iter()
        .any(|name| *name != element.tag_name)
    {
        return false;
    }

    if selector.id.iter().any(|id| Some(id) != element.id()) {
        return false;
    }

    if selector
        .class
        .iter()
        .any(|class_name| !element.classes().contains(class_name.as_str()))
    {
        return false;
    }

    return true;
}

type MatchedRule<'a> = (Specificity, &'a Rule);

fn match_rule<'a>(element: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
    rule.selectors
        .iter()
        .find(|selector| matches(element, selector))
        .map(|selector| (selector.specificity(), rule))
}

fn matching_rules<'a>(element: &ElementData, stylesheet: &'a Stylesheet) -> Vec<MatchedRule<'a>> {
    stylesheet
        .rules
        .iter()
        .filter_map(|rule| match_rule(element, rule))
        .collect()
}

fn add_inline_styles(element: &ElementData, values: &mut HashMap<String, Value>) {
    if element.attrs.contains_key("style") {
        let styles = element
            .attrs
            .get("style")
            .expect("The element does have a style attribute");
        let declrations = CssParser::parse_inline_style(styles.to_string());

        declrations.iter().for_each(|dec| {
            values.insert(dec.name.clone(), dec.value.clone());
        });
    }
}

fn specified_values(element: &ElementData, stylesheet: &Stylesheet) -> PropertyMap {
    let mut values = HashMap::new();
    let mut rules = matching_rules(element, stylesheet);

    rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));
    for (_, rule) in rules {
        for decrlation in rule.declarations.iter() {
            values.insert(decrlation.name.clone(), decrlation.value.clone());
        }
    }

    add_inline_styles(element, &mut values);

    values
}

fn text_node_values() -> PropertyMap {
    let mut values = HashMap::new();

    let styles = "display: inherit; color: inherit;";
    let declrations = CssParser::parse_inline_style(styles.to_string());

    declrations.iter().for_each(|dec| {
        values.insert(dec.name.clone(), dec.value.clone());
    });

    values
}

pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyleNode<'a> {
    let mut sn = StyleNode {
        node: root,
        specified_values: match &root.node_type {
            NodeType::Element(element) => specified_values(&element, stylesheet),
            NodeType::Text(_) => text_node_values(),
        },
        children: root
            .children
            .iter()
            .map(|child| style_tree(child, stylesheet))
            .collect(),
    };

    sn.apply_inherit_styles();

    sn
}
