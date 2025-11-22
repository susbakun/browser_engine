pub struct Stylesheet {
    pub rules: Vec<Rule>,
}

pub struct Rule {
    pub selectors: Vec<Selector>,
    pub declarations: Vec<Declartion>
}

pub enum Selector {
    Simple(SimpleSelector)
}

pub struct SimpleSelector {
    pub tag_name: Option<String>,
    pub id: Option<String>,
    pub class: Vec<String>
}

pub struct Declartion {
    pub name: String,
    pub value: Value
}

#[derive(Clone, PartialEq)]
pub enum Value {
    Keyword(String),
    Length(f32, Unit),
    ColorValue(Color)
}

#[derive(Clone, PartialEq)]
pub enum Unit {
    Px
}

#[derive(Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}


pub type Specificity = (usize, usize, usize);

impl Selector {
    pub fn specificity(&self) -> Specificity {
        // http://www.w3.org/TR/selectors/#specificity
        let Selector::Simple(simple) = self;
        let a = simple.id.iter().count();
        let b = simple.class.len();
        let c = simple.tag_name.iter().count();
        return (a, b, c);
    }
}

impl Value {
    pub fn to_px(&self) -> f32 {
        match *self {
            Value::Length(f, Unit::Px) => f,
            _ => 0.0
        }
    }
}

pub fn parse(source: String) -> Stylesheet {
    let mut parser = Parser {input: source, pos: 0};
    Stylesheet { rules: parser.parse_rules() }
}

struct Parser {
    input: String,
    pos: usize
}

impl Parser {
    fn parse_rules(&mut self)  -> Vec<Rule> {
        let mut rules = vec![];
        loop {
            self.consume_whitespace();
            if self.eof() {break};
            rules.push(self.parse_rule());
        }

        rules
    }

    fn parse_rule(&mut self) -> Rule {
        Rule { selectors: self.parse_selectors(), declarations: self.parse_declartions() }
    }

    fn parse_selectors(&mut self) -> Vec<Selector>{
        let mut selectors = vec![];
        loop {
            selectors.push(Selector::Simple(self.parse_simple_selectors()));
            self.consume_whitespace();
            match self.next_char(){
                ',' => {self.consume_char(); self.consume_whitespace();},
                '{' => break,
                c => panic!("Unexpected character: {c}")
            }
        }
        selectors.sort_by_key(|selector| selector.specificity());
        selectors
    }

    fn parse_simple_selectors(&mut self) -> SimpleSelector {
        let mut selector = SimpleSelector {id: None, tag_name: None, class: vec![]};
        while !self.eof() {
            match self.next_char() {
                '.' => {
                    self.consume_char();
                    selector.class.push(self.parse_identifier());
                }
                '#' => {
                    self.consume_char();
                    selector.id = Some(self.parse_identifier());
                },
                '*' => {
                    self.consume_char();
                }
                c if valid_identifier_char(c) => {
                    selector.tag_name = Some(self.parse_identifier());
                }
                _ => break
            }
        }

        selector
    }

    fn parse_declartions(&mut self) -> Vec<Declartion> {
        self.expect("{");
        let mut declrations = vec![];
        loop {
            self.consume_whitespace();
            if self.next_char() == '}' {
                self.consume_char();
                break;
            }
            declrations.push(self.parse_declration())
        }

        declrations
    }

    fn parse_declration(&mut self) -> Declartion {
        let name = self.parse_identifier();
        self.consume_whitespace();
        self.expect(":");
        self.consume_whitespace();
        let value = self.parse_value();
        self.expect(";");
        Declartion { name, value }
    }

    fn parse_value(&mut self) -> Value {
        match self.next_char() {
            '0'..'9' => self.parse_length(),
            '#' => self.parse_color(),
            _ => Value::Keyword(self.parse_identifier())
        }
    }
    
    fn parse_length(&mut self) -> Value {
        Value::Length(self.parse_float(), self.parse_unit())
    }

    fn parse_float(&mut self) -> f32 {
        self.consume_while(|c| matches!(c, '0'..'9' | '.')).parse().unwrap()
    }

    fn parse_unit(&mut self) -> Unit {
        self.consume_whitespace();
        match self.parse_identifier().to_ascii_lowercase().as_str() {
            "px" => Unit::Px,
            _ => panic!("unrecognized unit")
        }
    }

    fn parse_color(&mut self) -> Value {
        self.expect("#");
        Value::ColorValue(Color {
            r: self.parse_hex_pair(),
            g: self.parse_hex_pair(),
            b: self.parse_hex_pair(),
            a: 255
        })
    }

    fn parse_hex_pair(&mut self) -> u8 {
        let val = &self.input[self.pos..self.pos+2];
        self.pos += 2;
        u8::from_str_radix(val, 16).unwrap()
    }

    fn parse_identifier(&mut self) -> String {
        self.consume_while(valid_identifier_char)
    }

    fn next_char(&self) -> char {
        self.input[self.pos..].chars().next().unwrap()
    }


    fn expect(&mut self, s: &str){
        if self.input[self.pos..].starts_with(s){
            self.pos += s.len();
        }else{
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
        while !self.eof() && test(self.next_char()){
            result.push(self.consume_char())
        }

        result
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(char::is_whitespace);
    }
}

fn valid_identifier_char(c: char) -> bool {
    matches!(c, 'a'..'z' | 'A'..'Z' | '0'..'9' | '-' | '_')
}



#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn correct_css(){
        let source = fs::read_to_string("./test.css").unwrap();
        parse(source);
    }
}