use super::css::Value;
use super::css::{Parser as CssParser, Rule, Selector, SimpleSelector, Specificity, Stylesheet};
use super::dom::{ElementData, Node, NodeType};
use std::collections::HashMap;

type PropertyMap = HashMap<String, Value>;

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

pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a Stylesheet) -> StyleNode<'a> {
    StyleNode {
        node: root,
        specified_values: match &root.node_type {
            NodeType::Element(element) => specified_values(&element, stylesheet),
            NodeType::Text(_) => HashMap::new(),
        },
        children: root
            .children
            .iter()
            .map(|child| style_tree(child, stylesheet))
            .collect(),
    }
}
