use std::collections::HashMap;

use crate::{constants::SELF_CLOSING_TAGS, dom};

struct Parser {
    pos: usize,
    input: String,
}

impl Parser {
    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }

    fn starts_with(&self, s: &str) -> bool {
        self.input[self.pos..].starts_with(s)
    }

    fn expect(&mut self, s: &str) {
        if self.input[self.pos..].starts_with(s) {
            self.pos += s.len();
        } else {
            println!("{:?}", self.input.get(self.pos..));
            panic!("Expected {:?} at byte {} but it was not found", s, self.pos)
        }
    }

    fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn consume_char(&mut self) -> char {
        let c = self.next_char();
        self.pos += c.len_utf8();
        c
    }

    fn consume_while(&mut self, test: impl Fn(char) -> bool) -> String {
        let mut result = String::new();
        while !self.eof() && test(self.next_char()) {
            result.push(self.consume_char())
        }

        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }

    fn parse_name(&mut self) -> String {
        self.consume_while(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '0'..='9'))
    }

    fn parse_attr(&mut self) -> (String, String) {
        let name = self.parse_name();
        self.expect("=");
        let value = self.parse_attr_value();

        (name, value)
    }

    fn parse_attr_value(&mut self) -> String {
        let open_quote = self.consume_char();
        assert!(open_quote == '"' || open_quote == '\'');
        let value = self.consume_while(|c| c != open_quote);
        let close_quote = self.consume_char();
        assert!(close_quote == open_quote);
        value
    }

    fn parse_attributes(&mut self, closing_char: char) -> dom::AttrType {
        let mut attributes = HashMap::new();
        loop {
            self.consume_whitespace();
            if self.next_char() == closing_char {
                break;
            }
            let (name, value) = self.parse_attr();
            attributes.insert(name, value);
        }
        attributes
    }

    fn parse_text(&mut self) -> dom::Node {
        dom::text(self.consume_while(|c| c != '<'))
    }

    fn parse_element(&mut self) -> dom::Node {
        self.expect("<");

        let tag_name = self.parse_name();
        let attrs;
        let children;

        // handling self-closing tags seperately
        if SELF_CLOSING_TAGS.contains(&tag_name.as_str()) {
            attrs = self.parse_attributes('/');
            self.expect("/");
            self.expect(">");
            children = vec![];
        } else {
            attrs = self.parse_attributes('>');
            self.expect(">");
            children = self.parse_nodes();
            self.expect("</");
            self.expect(&tag_name);
            self.expect(">");
        }

        dom::element(tag_name, attrs, children)
    }

    fn parse_node(&mut self) -> dom::Node {
        if self.starts_with("<") {
            self.parse_element()
        } else {
            self.parse_text()
        }
    }

    fn parse_nodes(&mut self) -> Vec<dom::Node> {
        let mut nodes = Vec::new();
        loop {
            self.consume_whitespace();
            if self.eof() || self.starts_with("</") {
                break;
            }

            nodes.push(self.parse_node());
        }

        nodes
    }
}

pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser {
        pos: 0,
        input: source,
    }
    .parse_nodes();

    if nodes.len() == 1 {
        return nodes.remove(0);
    } else {
        dom::element("html".to_string(), HashMap::new(), nodes)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn parse_correct_html() {
        let html_code = fs::read_to_string("./test.html").expect("Couldn't read the file");
        parse(html_code);
    }
}
