use std::fmt;
use std::fs;
use std::io::BufReader;
use std::path::Path;

use anyhow::anyhow;
use anyhow::Result;

use xml::reader::{EventReader, XmlEvent};

use crate::core::element::collect_tree_text;
use crate::Element;

#[derive(Default)]
pub struct Document {
    pub version: String,
    pub encoding: String,
    pub children: Vec<Element>,
}

impl Document {
    pub fn new() -> Document {
        Default::default()
    }

    pub fn parse_file(path: impl AsRef<Path>) -> Result<Document> {
        let Ok(content) = fs::read_to_string(path.as_ref()) else {
            return Err(anyhow!("read {} failed",path.as_ref().display()));
        };

        Self::parse(content)
    }

    pub fn parse(content: impl AsRef<str>) -> Result<Document> {
        let buf_reader = BufReader::new(content.as_ref().as_bytes());
        let mut event_reader = EventReader::new(buf_reader);

        let mut doc = Document::new();
        let mut stack = Vec::new();

        loop {
            let event = event_reader.next().map_err(|e| anyhow!(e))?;

            match event {
                XmlEvent::StartDocument {
                    version, encoding, ..
                } => {
                    doc.version = version.to_string();
                    doc.encoding = encoding;
                }

                XmlEvent::StartElement {
                    name, attributes, ..
                } => {
                    let mut element: Element = name.into();

                    for attr in attributes {
                        element.insert_attribute(attr.name.local_name, attr.value);
                    }

                    stack.push(element);
                }

                XmlEvent::Characters(text) => match stack.last_mut() {
                    Some(element) => element.set_text(text),
                    None => return Err(anyhow!("element stack is invalid at xml text")),
                },

                XmlEvent::EndElement { .. } => {
                    let element = match stack.pop() {
                        Some(element) => element,
                        None => return Err(anyhow!("element stack is invalid at xml element")),
                    };

                    match stack.last_mut() {
                        Some(parent) => parent.push_child(element),
                        None => doc.children.push(element),
                    };
                }

                XmlEvent::EndDocument => break,

                _ => {}
            };
        }

        Ok(doc)
    }
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut data = String::new();
        data.push_str(&format!("version:{}\n", &self.version));
        data.push_str(&format!("encoding:{}\n", &self.encoding));

        data.push_str("element tree:\n");
        let element_count = self.children.len();
        for (index, element) in self.children.iter().enumerate() {
            let is_last = index == element_count - 1;
            let mut collected_text = String::new();

            collect_tree_text(element, String::new(), is_last, &mut collected_text);
            data.push_str(&collected_text);
        }

        write!(f, "{data}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_parse_test() {
        let doc = match Document::parse_file("../tests/example.xml") {
            Ok(doc) => doc,
            Err(e) => panic!("{}", e.to_string()),
        };

        println!("{}", doc);
    }
}
