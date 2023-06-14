use serde::Deserialize;
use std::{fs, path::Path};

use super::build_target::{TomlBundle, TomlDylib, TomlFile, TomlSubscene, TomlZip};

#[derive(Deserialize, Debug, Clone)]
pub struct TomlPKG {
    #[serde(rename = "bundle")]
    pub bundles: Option<Vec<TomlBundle>>,

    #[serde(rename = "subscene")]
    pub subscenes: Option<Vec<TomlSubscene>>,

    #[serde(rename = "file")]
    pub files: Option<Vec<TomlFile>>,

    #[serde(rename = "dylib")]
    pub dylibs: Option<Vec<TomlDylib>>,

    #[serde(rename = "zip")]
    pub zips: Option<Vec<TomlZip>>,
}

impl TomlPKG {
    pub fn get_deps(&self) -> Vec<String> {
        let Some(bundles) = &self.bundles else {
            return Vec::new();
        };

        let mut deps = Vec::new();
        for bundle in bundles {
            let Some(d) = bundle.dependencies.as_ref() else {
                continue;
            };
            deps.append(&mut d.to_owned());
        }
        deps
    }
}

pub(crate) fn parse(file: impl AsRef<Path>) -> Option<TomlPKG> {
    let Ok(content) = fs::read_to_string(file.as_ref()) else {
        eprintln!("read {} failed",file.as_ref().display());   
        return None;
    };

    let Ok(pkg) = toml::from_str(&content) else {
        eprintln!("parse {} failed",file.as_ref().display());   
        return None;
    };
    Some(pkg)
}

pub fn get_dep_patterns_from_file(file: impl AsRef<Path>) -> Option<Vec<String>> {
    match parse(file) {
        Some(pkg) => Some(pkg.get_deps()),
        None => None,
    }
}
