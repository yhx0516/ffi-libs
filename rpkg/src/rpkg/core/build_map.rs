use std::collections::BTreeMap;
use std::fmt::Display;
use std::path::Path;

use anyhow::anyhow;
use anyhow::Result;

use rutils::canonicalize_path;
use rutils::norm_path;

use crate::core::Assets;
use crate::core::Dependencies;
use crate::pkg;
use crate::BuildTarget;

use crate::core::resolve_build_deps;
use crate::resolve_target_path;
use crate::scan_files_block_pkg_manifest;

#[derive(Default)]
pub struct BuildMap {
    /// project path
    root_path: Option<String>,

    /// bundle map
    bundles: BTreeMap<String, Box<dyn BuildTarget>>,

    /// subscene map
    subscenes: BTreeMap<String, Box<dyn BuildTarget>>,

    /// file map
    files: BTreeMap<String, Box<dyn BuildTarget>>,

    /// dlib map
    dylibs: BTreeMap<String, Box<dyn BuildTarget>>,

    /// zip map
    zips: BTreeMap<String, Box<dyn BuildTarget>>,
}

impl BuildMap {
    pub fn new() -> BuildMap {
        BuildMap::default()
    }

    pub fn init(
        &mut self,
        root_path: impl AsRef<Path>,
        pkg_paths: Vec<impl AsRef<Path>>,
    ) -> Result<()> {
        let root_path = root_path.as_ref();
        self.root_path = Some(norm_path(canonicalize_path(&root_path)?));

        for pkg_path in pkg_paths {
            let pkg_path = pkg_path.as_ref();
            let cur_path = pkg_path.parent().unwrap();
            let pkg = pkg::parse(&pkg_path)?;

            for target in pkg.bundles.unwrap_or(Vec::new()) {
                let target_path = resolve_target_path(&root_path, cur_path, target.path.as_ref())?;
                self.bundles.entry(target_path).or_insert(Box::new(target));
            }

            for target in pkg.subscenes.unwrap_or(Vec::new()) {
                let target_path = resolve_target_path(&root_path, cur_path, target.path.as_ref())?;
                self.subscenes
                    .entry(target_path)
                    .or_insert(Box::new(target));
            }

            for target in pkg.files.unwrap_or(Vec::new()) {
                let target_path = resolve_target_path(&root_path, cur_path, target.path.as_ref())?;
                self.files.entry(target_path).or_insert(Box::new(target));
            }

            for target in pkg.dylibs.unwrap_or(Vec::new()) {
                let target_path = resolve_target_path(&root_path, cur_path, target.path.as_ref())?;
                self.dylibs.entry(target_path).or_insert(Box::new(target));
            }

            for target in pkg.zips.unwrap_or(Vec::new()) {
                let target_path = resolve_target_path(&root_path, cur_path, target.path.as_ref())?;
                self.zips.entry(target_path).or_insert(Box::new(target));
            }
        }

        Ok(())
    }

    pub fn get_root_path(&self) -> Result<&String> {
        self.root_path
            .as_ref()
            .map_or(Err(anyhow!("not found root path in build map")), |v| Ok(v))
    }

    pub fn get_root_path_mut(&mut self) -> Result<&mut String> {
        self.root_path
            .as_mut()
            .map_or(Err(anyhow!("not found root path in build map")), |v| Ok(v))
    }

    pub fn resolve_bundle_deps(&self, target_path: impl AsRef<str>) -> Result<Dependencies> {
        let root_path = self.get_root_path()?;
        resolve_build_deps(root_path, &norm_path(target_path.as_ref()), &self.bundles)
    }

    pub fn resolve_subscene_deps(&self, target_path: impl AsRef<str>) -> Result<Dependencies> {
        let root_path = self.get_root_path()?;
        resolve_build_deps(root_path, &norm_path(target_path.as_ref()), &self.subscenes)
    }

    pub fn resolve_dylib_deps(&self, target_path: impl AsRef<str>) -> Result<Dependencies> {
        let root_path = self.get_root_path()?;
        resolve_build_deps(root_path, &norm_path(target_path.as_ref()), &self.dylibs)
    }

    pub fn resolve_file_deps(&self, target_path: impl AsRef<str>) -> Result<Dependencies> {
        let root_path = self.get_root_path()?;
        resolve_build_deps(root_path, &norm_path(target_path.as_ref()), &self.files)
    }

    pub fn resolve_zip_deps(&self, target_path: impl AsRef<str>) -> Result<Dependencies> {
        let root_path = self.get_root_path()?;
        resolve_build_deps(root_path, &norm_path(target_path.as_ref()), &self.zips)
    }

    pub fn scan_bundle_assets(
        &self,
        mount_path: impl AsRef<str>,
        target_path: impl AsRef<str>,
    ) -> Result<Assets> {
        let root_path = self.get_root_path()?;
        inner_scan_assets(root_path, mount_path, target_path, &self.bundles)
    }

    pub fn scan_subscene_assets(
        &self,
        mount_path: impl AsRef<str>,
        target_path: impl AsRef<str>,
    ) -> Result<Assets> {
        let root_path = self.get_root_path()?;
        inner_scan_assets(root_path, mount_path, target_path, &self.subscenes)
    }

    pub fn scan_dylib_assets(
        &self,
        mount_path: impl AsRef<str>,
        target_path: impl AsRef<str>,
    ) -> Result<Assets> {
        let root_path = self.get_root_path()?;
        inner_scan_assets(root_path, mount_path, target_path, &self.dylibs)
    }

    pub fn scan_file_assets(
        &self,
        mount_path: impl AsRef<str>,
        target_path: impl AsRef<str>,
    ) -> Result<Assets> {
        let root_path = self.get_root_path()?;
        inner_scan_assets(root_path, mount_path, target_path, &self.files)
    }

    pub fn scan_zip_assets(
        &self,
        mount_path: impl AsRef<str>,
        target_path: impl AsRef<str>,
    ) -> Result<Assets> {
        let root_path = self.get_root_path()?;
        inner_scan_assets(root_path, mount_path, target_path, &self.zips)
    }
}

fn inner_scan_assets(
    root_path: impl AsRef<str>,
    mount_path: impl AsRef<str>,
    target_path: impl AsRef<str>,
    target_map: &BTreeMap<String, Box<dyn BuildTarget>>,
) -> Result<Assets> {
    let root_path = root_path.as_ref();
    let mount_path = norm_path(canonicalize_path(mount_path.as_ref())?);
    let target_path = norm_path(target_path.as_ref());

    let Some(target) = target_map.get(&target_path) else {
        return Err(anyhow!("not found target: {}",&target_path));
    };

    let Some(patterns) = target.get_patterns() else {
        return Err(anyhow!("not found patters in {}",&target_path));
    };

    let path = Path::new(root_path).join(&target_path);
    let mut assets = Assets::new();

    for item in scan_files_block_pkg_manifest(path, patterns) {
        let Some(rel_path) = item.strip_prefix(root_path) else {
            return Err(anyhow!("{} is not the prefix of {}",root_path, &target_path));
        };

        let target_path = format!("{}/{}", root_path, target_path);
        let url = target.build_asset_url(&mount_path, &target_path, &item);

        assets.push_asset(rel_path.to_string(), url)
    }

    Ok(assets)
}

impl Display for BuildMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        let to_string = |target_map: &BTreeMap<String, Box<dyn BuildTarget>>| {
            let mut output = String::new();
            for (key, val) in target_map {
                output.push_str(&format!("  {}  {}\r\n", key, val.display()));
            }
            output
        };

        output.push_str(&format!("root_path:\n  {:?}\n", self.root_path));
        output.push_str(&format!("bundles:\n{}", to_string(&self.bundles)));
        output.push_str(&format!("subscenes:\n{}", to_string(&self.subscenes)));
        output.push_str(&format!("dylibs:\n{}", to_string(&self.dylibs)));
        output.push_str(&format!("files:\n{}", to_string(&self.files)));
        output.push_str(&format!("zips:\n{}", to_string(&self.zips)));
        f.write_str(&output)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::core::BuildMap;
    use crate::scan_files;
    use crate::scan_files_block_manifest;

    #[test]
    fn build_map_test() {
        let asset_path = "../tests/pkg-dependencies/BuildAssets";
        let patterns = ["**/.pkg"];
        let pkgs = scan_files(asset_path, &patterns);
        println!("total pkgs:");
        for item in &pkgs {
            println!("  {}", item);
        }
        println!();

        let mut build_map = BuildMap::new();
        let root_path = r"../tests/pkg-dependencies/";
        if let Err(e) = build_map.init(root_path, pkgs) {
            panic!("{}", e.to_string());
        }
        println!("{}\n", build_map.to_string());

        let members = ["./", "addon1", "addon2"];
        println!("addons and pkgs:");
        for member in members {
            let addon_path = Path::new(asset_path).join(member);
            let addon_pkgs = scan_files_block_manifest(addon_path, &patterns);
            println!("  addon \"{}\" pkgs:", member);
            for item in &addon_pkgs {
                println!("    {}", item);
            }
        }
        println!();

        let mount_path = "../tests/pkg-dependencies/BuildAssets/addon1";
        let target_path = "BuildAssets/addon1/Prefab";

        let to_build = match build_map.resolve_bundle_deps(target_path) {
            Err(e) => panic!("{}", e.to_string()),
            Ok(r) => r,
        };

        println!("to_build:");
        for target in &to_build.build_targets {
            println!("  {} assets:", target);
            let assets = match build_map.scan_bundle_assets(mount_path, target) {
                Ok(r) => r,
                Err(e) => panic!("{}", e.to_string()),
            };
            println!("{}", assets);
        }
    }
}
