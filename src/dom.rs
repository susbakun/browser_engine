use std::collections::{HashMap, HashSet};

pub type AttrType = HashMap<String, String>;

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

impl Node {
    fn get_tag_name(&self) -> Option<&str> {
        match &self.node_type {
            NodeType::Element(ed) => Some(ed.tag_name.as_str()),
            NodeType::Text(_) => None,
        }
    }
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData),
}

#[derive(Debug)]
pub struct ElementData {
    pub tag_name: String,
    pub attrs: AttrType,
}

pub fn text(data: String) -> Node {
    Node {
        children: vec![],
        node_type: NodeType::Text(data),
    }
}

pub fn element(tag_name: String, attrs: AttrType, children: Vec<Node>) -> Node {
    Node {
        children,
        node_type: NodeType::Element(ElementData { tag_name, attrs }),
    }
}

impl ElementData {
    pub fn get_attr(&self, attribute: &str) -> Option<&String> {
        self.attrs.get(attribute)
    }

    pub fn id(&self) -> Option<&String> {
        self.get_attr("id")
    }

    pub fn classes(&self) -> HashSet<&str> {
        match self.get_attr("class") {
            Some(classlist) => classlist.split(" ").collect(),
            None => HashSet::new(),
        }
    }
}
