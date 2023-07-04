use serde::Deserialize;
use std::path::Path;

use super::BuildTarget;
use super::ASSET_PROTOCAL;

#[derive(Deserialize, Debug, Clone)]
pub struct TomlDylib {
    pub path: Option<String>,
    pub patterns: Option<Vec<String>>,
    pub dependencies: Option<Vec<String>>,
}

impl BuildTarget for TomlDylib {
    fn get_path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    fn get_patterns(&self) -> Option<&Vec<String>> {
        self.patterns.as_ref()
    }

    fn get_deps(&self) -> Option<&Vec<String>> {
        self.dependencies.as_ref()
    }

    // fn build_asset_url(
    //     &self,
    //     _: &Option<impl AsRef<Path>>,
    //     asset_path: impl AsRef<Path>,
    // ) -> String {
    //     let asset_path = Path::new(asset_path.as_ref())
    //         .with_extension("dll")
    //         .display()
    //         .to_string();

    //     let url = format!("{}://{}", ASSET_PROTOCAL, asset_path.trim_matches('/'));
    //     url.to_lowercase()
    // }
}
