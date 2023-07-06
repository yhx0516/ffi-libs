use anyhow::anyhow;
use serde::Deserialize;
use std::{fs, path::Path};

use super::build_target::*;

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

pub fn parse(file: impl AsRef<Path>) -> anyhow::Result<TomlPKG> {
    let Ok(content) = fs::read_to_string(file.as_ref()) else {
        return Err(anyhow!("read {} failed",file.as_ref().display()));
    };

    let Ok(pkg) = toml::from_str(&content) else {
        return Err(anyhow!("parse {} failed",file.as_ref().display()));
    };

    Ok(pkg)
}
