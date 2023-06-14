use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct TomlBundle {
    pub path: Option<String>,
    pub patterns: Option<Vec<String>>,
    pub dependencies: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TomlSubscene {
    pub path: Option<String>,
    pub patterns: Option<Vec<String>>,
    pub dependencies: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TomlFile {
    pub path: Option<String>,
    pub patterns: Option<Vec<String>>,
    pub dependencies: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TomlDylib {
    pub path: Option<String>,
    pub patterns: Option<Vec<String>>,
    pub dependencies: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct TomlZip {
    pub path: Option<String>,
    pub patterns: Option<Vec<String>>,
    pub dependencies: Option<Vec<String>>,
}
