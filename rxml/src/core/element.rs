use std::collections::BTreeMap;

pub struct Element {
    name: String,
    text: String,
    attributes: BTreeMap<String, String>,
    children: Vec<Element>,
}

impl Element {
    pub fn new(name: String) -> Element {
        Self {
            name,
            text: String::new(),
            attributes: BTreeMap::new(),
            children: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_text(&self) -> &String {
        &self.text
    }

    pub fn get_attribute_keys(&self) -> Vec<String> {
        self.attributes.keys().cloned().collect()
    }

    pub fn get_attribute_value(&self, key: &str) -> Option<&String> {
        self.attributes.get(key)
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    pub fn insert_attribute(&mut self, key: String, val: String) {
        self.attributes.insert(key, val);
    }

    pub fn push_child(&mut self, val: Element) {
        self.children.push(val);
    }

    pub fn get_children_len(&self) -> usize {
        self.children.len()
    }

    pub fn get_child(&self, index: usize) -> Option<&Element> {
        self.children.get(index)
    }

    pub fn get_child_mut(&mut self, index: usize) -> Option<&mut Element> {
        self.children.get_mut(index)
    }
}

pub fn collect_tree_text(
    element: &Element,
    prefix: String,
    is_last: bool,
    collected_text: &mut String,
) {
    let tree_text = match is_last {
        true => format!("{}└── {}\n", prefix, &element.name),
        false => format!("{}├── {}\n", prefix, &element.name),
    };
    collected_text.push_str(&tree_text);

    let count = element.children.len();
    for (index, element) in element.children.iter().enumerate() {
        let is_last_child = index == count - 1;
        let new_prefix = match (is_last, is_last_child) {
            (true, true) => format!("{prefix}   "),
            (false, true) => format!("{prefix}│  "),
            (true, false) => format!("{prefix}   "),
            (false, false) => format!("{prefix}│  "),
        };

        collect_tree_text(element, new_prefix, is_last_child, collected_text);
    }
}
