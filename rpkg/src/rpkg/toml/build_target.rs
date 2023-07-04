use std::{path::Path, string};

use crate::core::scan_files;

mod bundle;
mod dylib;
mod file;
mod subscene;
mod zip;

pub use bundle::TomlBundle;
pub use dylib::TomlDylib;
pub use file::TomlFile;
use rutils::{canonicalize_path, norm_path};
pub use subscene::TomlSubscene;
pub use zip::TomlZip;

pub(crate) const ASSET_PROTOCAL: &str = "asset";
pub(crate) const ASSET_PKG_EXTENSION: &str = "bundle";

pub trait BuildTarget {
    fn get_path(&self) -> Option<&String>;

    fn get_patterns(&self) -> Option<&Vec<String>>;

    fn get_deps(&self) -> Option<&Vec<String>>;

    fn is_pkg(&self) -> bool {
        true
    }

    fn display(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("path = {:?}, ", self.get_path()));
        output.push_str(&format!("patterns = {:?}, ", self.get_patterns()));
        output.push_str(&format!("dependencies = {:?}, ", self.get_deps()));
        output
    }
}

pub fn resolve_target_path(
    root_path: impl AsRef<Path>,
    cur_path: impl AsRef<Path>,
    target_path: Option<&String>,
) -> anyhow::Result<String> {
    let root_path = canonicalize_path(root_path.as_ref())?;
    let cur_path = cur_path.as_ref();

    let path = match target_path {
        Some(ref p) => canonicalize_path(cur_path.join(p))?,
        None => canonicalize_path(cur_path)?,
    };

    let res = norm_path(path.strip_prefix(root_path)?);
    Ok(res)
}

pub fn resolve_dep_path(
    root_path: impl AsRef<Path>,
    cur_path: impl AsRef<Path>,
    deps: Option<&Vec<String>>,
) -> anyhow::Result<Vec<String>> {
    let Some(deps) = deps else {
        return Ok(Vec::new());
    };

    let root_path = canonicalize_path(root_path.as_ref())?;
    let cur_path = cur_path.as_ref();

    let mut res = Vec::new();
    for dep_path in deps {
        let new_path = norm_dep_path(&root_path, cur_path, dep_path)?;
        res.push(new_path);
    }
    Ok(res)
}

fn norm_dep_path(
    root_path: impl AsRef<Path>,
    cur_path: impl AsRef<Path>,
    dep_path: impl AsRef<Path>,
) -> anyhow::Result<String> {
    let root_path = root_path.as_ref();
    let cur_path = cur_path.as_ref();
    let dep_path = dep_path.as_ref();

    if dep_path.starts_with("/") {
        return Ok(norm_path(dep_path));
    }

    let path = canonicalize_path(root_path.join(cur_path).join(dep_path))?;
    Ok(norm_path(path.strip_prefix(root_path)?))
}

pub trait IntoBuildTarget {
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
    // fn scan_assets(&self, root_path: impl AsRef<Path>, cur_path: impl AsRef<Path>) -> Vec<String> {
    //     let root_path = root_path.as_ref();
    //     let cur_path = cur_path.as_ref();

    //     let scan_path = match &self.get_path() {
    //         Some(p) => root_path.join(p),
    //         None => cur_path.to_path_buf(),
    //     };

    //     let pkg_path = match (self.is_pkg(), root_path == scan_path) {
    //         (true, true) => {
    //             let path = Path::new("assets");
    //             let path = path.with_extension(ASSET_PKG_EXTENSION);
    //             Some(path)
    //         }
    //         (true, false) => {
    //             let path = scan_path.strip_prefix(root_path).unwrap();
    //             let path = path.with_extension(ASSET_PKG_EXTENSION);
    //             Some(path)
    //         }
    //         _ => None,
    //     };

    //     let Some(patterns) = &self.get_patterns() else {
    //         return Vec::new();
    //     };

    //     let mut assets = Vec::new();
    //     let scan_path = scan_path.display().to_string().replace("\\", "/");
    //     let root_path = root_path.display().to_string().replace("\\", "/");

    //     for file in &scan_files(&scan_path, patterns) {
    //         let asset_path = match pkg_path {
    //             Some(_) => file.strip_prefix(&scan_path).unwrap(),
    //             None => file.strip_prefix(&root_path).unwrap(),
    //         };

    //         let asset = self.build_asset_url(&pkg_path, asset_path);
    //         assets.push(asset);
    //     }

    //     assets
    // }

    // fn build_asset_url(
    //     &self,
    //     pkg_path: &Option<impl AsRef<Path>>,
    //     asset_path: impl AsRef<Path>,
    // ) -> String {
    //     let url = {
    //         let asset_path = asset_path.as_ref().display().to_string().replace("\\", "/");
    //         let asset_path = asset_path.trim_matches('/');

    //         match pkg_path {
    //             Some(p) => {
    //                 let path = p.as_ref().display().to_string().replace("\\", "/");
    //                 format!(
    //                     "{}://{}/{}",
    //                     ASSET_PROTOCAL,
    //                     path.trim_matches('/'),
    //                     asset_path
    //                 )
    //             }
    //             None => format!("{}://{}", ASSET_PROTOCAL, asset_path),
    //         }
    //     };
    //     url.to_lowercase()
    // }
}
