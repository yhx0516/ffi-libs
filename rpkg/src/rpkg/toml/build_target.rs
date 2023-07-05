use std::path::Path;

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

    fn is_pkg(&self) -> bool;

    fn display(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!("path = {:?}, ", self.get_path()));
        output.push_str(&format!("patterns = {:?}, ", self.get_patterns()));
        output.push_str(&format!("dependencies = {:?}, ", self.get_deps()));
        output
    }

    fn build_asset_url(&self, mount_path: &str, target_path: &str, asset_path: &str) -> String {
        let pkg_path = match (self.is_pkg(), mount_path == target_path) {
            (true, true) => {
                let path = Path::new("assets").with_extension(ASSET_PKG_EXTENSION);
                Some(path)
            }
            (true, false) => {
                let path = target_path.strip_prefix(mount_path).unwrap();
                let path = Path::new(path).with_extension(ASSET_PKG_EXTENSION);
                Some(path)
            }
            _ => None,
        };

        let rel_asset_path = match pkg_path {
            Some(_) => asset_path.strip_prefix(target_path).unwrap(),
            None => asset_path.strip_prefix(mount_path).unwrap(),
        };

        let url = match pkg_path {
            Some(p) => {
                format!(
                    "{}://{}/{}",
                    ASSET_PROTOCAL,
                    norm_path(p),
                    norm_path(rel_asset_path)
                )
            }
            None => format!("{}://{}", ASSET_PROTOCAL, asset_path),
        };
        url.to_lowercase()
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
