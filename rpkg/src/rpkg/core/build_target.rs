use serde::Deserialize;
use std::path::Path;

use super::match_patterns;

const ASSET_PROTOCAL: &str = "asset";
const ASSET_PKG_EXTENSION: &str = "bundle";

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

pub trait BuildTarget {
    fn get_path(&self) -> &Option<String>;

    fn get_patterns(&self) -> &Option<Vec<String>>;

    fn is_pkg(&self) -> bool {
        true
    }

    // scan assets and return asset urls by build target
    // root_path is usually {unity-project-dir}/Assets
    //
    // examples
    // bundle or subscene target (from root_path)
    //   - asset_src: "{unity-project-dir}/Assets/foobar.prefab"
    //   - pkg_path: "assets.bundle"
    //   - url: "asset://assets.bundle/foobar.prefab"
    //
    // bundle or subscene target (not from root_path)
    //   - asset_src: "{unity-project-dir}/Assets/Arts/Character/Clips/Chr_player_actor/CLR_fall2idle.anim"
    //   - pkg_path: "arts/character/clips.bundle"
    //   - url: "asset://arts/character/clips.bundle/chr_player_actor/clr_fall2idle.anim"
    //
    // file target
    //   - asset_src: "{unity-project-dir}/Assets/Gameplay/Inputs/InputAction.txt"
    //   - pkg_path: None
    //   - url: "asset://gameplay/inputs/inputaction.txt"
    //
    // dylib target
    //   - asset_src: "{unity-project-dir}/Assets/Scripts/Core/Runtime/Framework.Core.Runtime.asmdef"
    //   - pkg_path: None
    //   - url: "asset://scripts/core/runtime/framework.core.runtime.dll"
    //
    // zip(todo)
    fn scan_assets(&self, root_path: impl AsRef<Path>, cur_path: impl AsRef<Path>) -> Vec<String> {
        let root_path = root_path.as_ref();
        let cur_path = cur_path.as_ref();

        let scan_path = match &self.get_path() {
            Some(p) => root_path.join(p),
            None => cur_path.to_path_buf(),
        };

        let pkg_path = match (self.is_pkg(), root_path == scan_path) {
            (true, true) => {
                let path = Path::new("assets");
                let path = path.with_extension(ASSET_PKG_EXTENSION);
                Some(path)
            }
            (true, false) => {
                let path = scan_path.strip_prefix(root_path).unwrap();
                let path = path.with_extension(ASSET_PKG_EXTENSION);
                Some(path)
            }
            _ => None,
        };

        let Some(patterns) = &self.get_patterns() else {
            return Vec::new();
        };

        let mut assets = Vec::new();
        let scan_path = scan_path.display().to_string().replace("\\", "/");
        let root_path = root_path.display().to_string().replace("\\", "/");

        for file in &match_patterns(&scan_path, patterns) {
            let asset_path = match pkg_path {
                Some(_) => file.strip_prefix(&scan_path).unwrap(),
                None => file.strip_prefix(&root_path).unwrap(),
            };

            let asset = self.build_asset_url(&pkg_path, asset_path);
            assets.push(asset);
        }

        assets
    }

    fn build_asset_url(
        &self,
        pkg_path: &Option<impl AsRef<Path>>,
        asset_path: impl AsRef<Path>,
    ) -> String {
        let url = {
            let asset_path = asset_path.as_ref().display().to_string().replace("\\", "/");
            let asset_path = asset_path.trim_matches('/');

            match pkg_path {
                Some(p) => {
                    let path = p.as_ref().display().to_string().replace("\\", "/");
                    format!(
                        "{}://{}/{}",
                        ASSET_PROTOCAL,
                        path.trim_matches('/'),
                        asset_path
                    )
                }
                None => format!("{}://{}", ASSET_PROTOCAL, asset_path),
            }
        };
        url.to_lowercase()
    }
}

impl BuildTarget for TomlBundle {
    fn get_path(&self) -> &Option<String> {
        &self.path
    }

    fn get_patterns(&self) -> &Option<Vec<String>> {
        &self.patterns
    }
}

impl BuildTarget for TomlSubscene {
    fn get_path(&self) -> &Option<String> {
        &self.path
    }

    fn get_patterns(&self) -> &Option<Vec<String>> {
        &self.patterns
    }
}

impl BuildTarget for TomlFile {
    fn get_path(&self) -> &Option<String> {
        &self.path
    }

    fn get_patterns(&self) -> &Option<Vec<String>> {
        &self.patterns
    }

    fn is_pkg(&self) -> bool {
        false
    }
}

impl BuildTarget for TomlZip {
    fn get_path(&self) -> &Option<String> {
        &self.path
    }

    fn get_patterns(&self) -> &Option<Vec<String>> {
        &self.patterns
    }
}

impl BuildTarget for TomlDylib {
    fn get_path(&self) -> &Option<String> {
        &self.path
    }

    fn get_patterns(&self) -> &Option<Vec<String>> {
        &self.patterns
    }

    fn is_pkg(&self) -> bool {
        false
    }

    fn build_asset_url(
        &self,
        _: &Option<impl AsRef<Path>>,
        asset_path: impl AsRef<Path>,
    ) -> String {
        let asset_path = Path::new(asset_path.as_ref())
            .with_extension("dll")
            .display()
            .to_string();

        let url = format!("{}://{}", ASSET_PROTOCAL, asset_path.trim_matches('/'));
        url.to_lowercase()
    }
}
