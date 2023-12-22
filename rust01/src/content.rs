use std::any::type_name;
use std::fmt::Display;
use std::fs;
use std::str::FromStr;

use html_parser::{Dom, Element, Node};

use crate::text::Text;

#[derive(Debug)]
pub struct Content {
    pub texts: Vec<Text>,
}

impl Content {
    pub(crate) fn new(path: &String) -> Content {
        let mut content = Content { texts: vec![] };
        let html = fs::read_to_string(path).unwrap();
        let dom = Dom::parse(&html).expect("Invalid input.");
        dom.children
            .iter()
            .for_each(|node| content.parse_node(node, &Text::new()));
        content
    }
    fn parse_node(&mut self, node: &Node, parent_text: &Text) {
        let mut text = parent_text.clone();
        match node {
            Node::Text(value) => {
                text.content = value.to_string();
                self.texts.push(text)
            }
            Node::Element(element) => {
                self.parse_element(element, &mut text);
                element
                    .children
                    .iter()
                    .for_each(|node| self.parse_node(node, &text))
            }
            Node::Comment(_) => {}
        }
    }
    fn parse_element(&self, element: &Element, text: &mut Text) {
        Content::parse_attribute(element, &mut text.font_size, "size");
        Content::parse_attribute(element, &mut text.font_color, "color");
        Content::parse_attribute(element, &mut text.background_color, "background");
    }

    fn parse_attribute<U: FromStr>(element: &Element, u: &mut U, key: &str)
    where
        U::Err: Display,
    {
        let option = element.attributes.get(key).and_then(|option| {
            option.as_ref().map(|value| {
                value.parse().unwrap_or_else(|e| {
                    panic!(
                        "Invalid value {}. Expect {} Error: {}",
                        value,
                        type_name::<U>(),
                        e
                    )
                })
            })
        });
        if let Some(value) = option {
            *u = value;
        }
    }
}
