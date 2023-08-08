use anyhow::Result;
use funny_utils_rs::path::canonicalize_path;
use funny_utils_rs::path::norm_path;
use funny_utils_rs::path::trim_path2;
use std::path::Path;

mod bundle;
mod dylib;
mod file;
mod subscene;
mod zip;

pub use bundle::TomlBundle;
pub use dylib::TomlDylib;
pub use file::TomlFile;
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

    fn build_asset_url(&self, addon_path: &str, target_path: &str, asset_path: &str) -> String {
        // NOTE: 出现要 build 的资源不在指定的 addon_path 下，则直接返回空字符串
        let Some(pkg_path) =  target_path.strip_prefix(addon_path) else {
            println!("addon path {} is not the prefix of {}",addon_path,target_path);
            return String::new();
        };

        let pkg_path = match (self.is_pkg(), addon_path == target_path) {
            (true, true) => {
                let path = Path::new("assets").to_path_buf();
                Some(path)
            }
            (true, false) => {
                let path = Path::new(&trim_path2(pkg_path)).to_path_buf();
                Some(path)
            }
            _ => None,
        };

        let rel_asset_path = match pkg_path {
            Some(_) => asset_path.strip_prefix(target_path).unwrap(),
            None => asset_path.strip_prefix(addon_path).unwrap(),
        };

        let url = match pkg_path {
            Some(p) => {
                format!(
                    "{}://{}/{}",
                    ASSET_PROTOCAL,
                    trim_path2(p),
                    trim_path2(rel_asset_path)
                )
            }
            None => format!("{}://{}", ASSET_PROTOCAL, trim_path2(rel_asset_path)),
        };
        url.to_lowercase()
    }
}

pub fn resolve_target_path(
    root_path: impl AsRef<Path>,
    cur_path: impl AsRef<Path>,
    target_path: Option<&String>,
) -> Result<String> {
    let root_path = root_path.as_ref();
    let cur_path = cur_path.as_ref();

    let path = match target_path {
        Some(ref p) => canonicalize_path(cur_path.join(p))?,
        None => canonicalize_path(cur_path)?,
    };

    let res = trim_path2(path.strip_prefix(root_path)?)
        .trim_start_matches('/')
        .to_string();
    Ok(res)
}

pub fn to_bundle_path(
    root_path: impl AsRef<Path>,
    addon_path: impl AsRef<Path>,
    target_path: impl AsRef<Path>,
) -> Result<String> {
    let root_path = root_path.as_ref();
    let addon_path = addon_path.as_ref();
    let target_path = root_path.join(target_path.as_ref());

    let rel_path = match addon_path == target_path {
        true => String::from("assets"),
        false => trim_path2(target_path.strip_prefix(addon_path)?),
    };

    let url = format!("{}.{}", rel_path, ASSET_PKG_EXTENSION);
    Ok(url.to_lowercase())
}

pub fn resolve_dep_path(
    root_path: impl AsRef<Path>,
    cur_path: impl AsRef<Path>,
    deps: Option<&Vec<String>>,
) -> Result<Vec<String>> {
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
) -> Result<String> {
    let root_path = root_path.as_ref();
    let cur_path = cur_path.as_ref();
    let dep_path = dep_path.as_ref();

    if dep_path.starts_with("/") {
        return Ok(trim_path2(dep_path));
    }

    let path = match norm_path(dep_path).split_once('*') {
        Some((rhs, lhs)) => {
            canonicalize_path(root_path.join(cur_path).join(rhs))?.join(format!("*{}", lhs))
        }
        None => canonicalize_path(root_path.join(cur_path).join(&dep_path))?,
    };

    Ok(trim_path2(path.strip_prefix(root_path)?))
}
