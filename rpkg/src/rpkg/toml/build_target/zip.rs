use serde::Deserialize;

use crate::BuildTarget;

#[derive(Deserialize, Debug, Clone)]
pub struct TomlZip {
    pub path: Option<String>,
    pub patterns: Option<Vec<String>>,
    pub dependencies: Option<Vec<String>>,
}

impl BuildTarget for TomlZip {
    fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    fn get_patterns(&self) -> Option<&Vec<String>> {
        self.patterns.as_ref()
    }

    fn get_deps(&self) -> Option<&Vec<String>> {
        self.dependencies.as_ref()
    }

    fn is_pkg(&self) -> bool {
        true
    }
}
