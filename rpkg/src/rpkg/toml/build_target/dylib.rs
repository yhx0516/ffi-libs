use rutils::norm_path;
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

    fn is_pkg(&self) -> bool {
        false
    }

    fn build_asset_url(&self, mount_path: &str, _: &str, asset_path: &str) -> String {
        let asset_path = Path::new(asset_path).with_extension("dll");
        let rel_asset_path = asset_path.strip_prefix(mount_path).unwrap();

        let url = format!("{}://{}", ASSET_PROTOCAL, norm_path(rel_asset_path));
        url.to_lowercase()
    }
}
