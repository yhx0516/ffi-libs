use std::collections::BTreeMap;
use std::fmt::Display;
use std::path::Path;

use anyhow::anyhow;
use anyhow::Result;

use rutils::path::canonicalize_path;
use rutils::path::norm_path;
use rutils::path::norm_path_extreme;

use crate::core::resolve_build_deps;
use crate::core::Assets;
use crate::core::Dependencies;
use crate::pkg;
use crate::BuildTarget;

use crate::build_target_url;
use crate::resolve_target_path;
use crate::scan_files_block_pkg_manifest;

#[derive(Default)]
pub struct BuildMap {
    /// project path
    root_path: String,

    /// bundle-url map
    bundle_urls: BTreeMap<String, String>,

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
    pub fn new(root_path: impl AsRef<Path>) -> Result<BuildMap> {
        let mut build_map = BuildMap::default();
        build_map.root_path = norm_path(canonicalize_path(root_path.as_ref())?);
        Ok(build_map)
    }

    pub fn get_root_path(&self) -> &String {
        &self.root_path
    }

    pub fn get_bundle_paths(&self) -> Vec<&String> {
        self.bundles.iter().map(|(key, _)| key).collect()
    }

    pub fn get_subscene_paths(&self) -> Vec<&String> {
        self.subscenes.iter().map(|(key, _)| key).collect()
    }

    pub fn get_file_paths(&self) -> Vec<&String> {
        self.files.iter().map(|(key, _)| key).collect()
    }

    pub fn get_dylib_paths(&self) -> Vec<&String> {
        self.dylibs.iter().map(|(key, _)| key).collect()
    }

    pub fn get_zip_paths(&self) -> Vec<&String> {
        self.zips.iter().map(|(key, _)| key).collect()
    }

    // insert build-target from pkgs
    pub fn insert(
        &mut self,
        mount_path: impl AsRef<str>,
        pkg_paths: Vec<impl AsRef<Path>>,
    ) -> Result<()> {
        let root_path = self.get_root_path().clone();
        let mount_path = canonicalize_path(mount_path.as_ref())?;

        for pkg_path in pkg_paths {
            let pkg_path = pkg_path.as_ref();
            let cur_path = pkg_path.parent().unwrap();
            let pkg = pkg::parse(&pkg_path)?;

            for target in pkg.bundles.unwrap_or(Vec::new()) {
                let target_path = resolve_target_path(&root_path, cur_path, target.path.as_ref())?;
                self.bundles
                    .entry(target_path.clone())
                    .or_insert(Box::new(target));

                // update bundle_url map
                {
                    let target_url = build_target_url(&root_path, &mount_path, &target_path)?;
                    let target_path = target_path.to_lowercase();
                    self.bundle_urls.entry(target_path).or_insert(target_url);
                }
            }

            for target in pkg.subscenes.unwrap_or(Vec::new()) {
                let target_path = resolve_target_path(&root_path, cur_path, target.path.as_ref())?;
                self.subscenes
                    .entry(target_path.clone())
                    .or_insert(Box::new(target));

                // update bundle_url map
                {
                    let target_url = build_target_url(&root_path, &mount_path, &target_path)?;
                    let target_path = target_path.to_lowercase();
                    self.bundle_urls.entry(target_path).or_insert(target_url);
                }
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

    pub fn find_bundle_url(&self, bundle_path: impl AsRef<str>) -> Result<String> {
        let bundle_path = bundle_path.as_ref().to_lowercase();
        match self.bundle_urls.get(&bundle_path) {
            Some(v) => Ok(v.to_owned()),
            None => Err(anyhow!("not found {} in map", bundle_path)),
        }
    }

    pub fn resolve_bundle_deps(&self, target_path: impl AsRef<str>) -> Result<Dependencies> {
        let root_path = self.get_root_path();
        resolve_build_deps(root_path, &norm_path(target_path.as_ref()), &self.bundles)
    }

    pub fn resolve_subscene_deps(&self, target_path: impl AsRef<str>) -> Result<Dependencies> {
        let root_path = self.get_root_path();
        resolve_build_deps(root_path, &norm_path(target_path.as_ref()), &self.subscenes)
    }

    pub fn resolve_dylib_deps(&self, target_path: impl AsRef<str>) -> Result<Dependencies> {
        let root_path = self.get_root_path();
        resolve_build_deps(root_path, &norm_path(target_path.as_ref()), &self.dylibs)
    }

    pub fn resolve_file_deps(&self, target_path: impl AsRef<str>) -> Result<Dependencies> {
        let root_path = self.get_root_path();
        resolve_build_deps(root_path, &norm_path(target_path.as_ref()), &self.files)
    }

    pub fn resolve_zip_deps(&self, target_path: impl AsRef<str>) -> Result<Dependencies> {
        let root_path = self.get_root_path();
        resolve_build_deps(root_path, &norm_path(target_path.as_ref()), &self.zips)
    }

    pub fn scan_bundle_assets(
        &self,
        mount_path: impl AsRef<str>,
        target_path: impl AsRef<str>,
    ) -> Result<Assets> {
        let root_path = self.get_root_path();
        inner_scan_assets(root_path, mount_path, target_path, &self.bundles)
    }

    pub fn scan_subscene_assets(
        &self,
        mount_path: impl AsRef<str>,
        target_path: impl AsRef<str>,
    ) -> Result<Assets> {
        let root_path = self.get_root_path();
        inner_scan_assets(root_path, mount_path, target_path, &self.subscenes)
    }

    pub fn scan_dylib_assets(
        &self,
        mount_path: impl AsRef<str>,
        target_path: impl AsRef<str>,
    ) -> Result<Assets> {
        let root_path = self.get_root_path();
        inner_scan_assets(root_path, mount_path, target_path, &self.dylibs)
    }

    pub fn scan_file_assets(
        &self,
        mount_path: impl AsRef<str>,
        target_path: impl AsRef<str>,
    ) -> Result<Assets> {
        let root_path = self.get_root_path();
        inner_scan_assets(root_path, mount_path, target_path, &self.files)
    }

    pub fn scan_zip_assets(
        &self,
        mount_path: impl AsRef<str>,
        target_path: impl AsRef<str>,
    ) -> Result<Assets> {
        let root_path = self.get_root_path();
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

        assets.push_asset(norm_path_extreme(rel_path), url)
    }

    Ok(assets)
}

impl Display for BuildMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        let url_to_str = |target_map: &BTreeMap<String, String>| {
            let mut output = String::new();
            for (key, val) in target_map {
                output.push_str(&format!("    {}  {}\n", key, val));
            }
            output
        };

        let target_to_str = |target_map: &BTreeMap<String, Box<dyn BuildTarget>>| {
            let mut output = String::new();
            for (key, val) in target_map {
                output.push_str(&format!("    {}  {}\n", key, val.display()));
            }
            output
        };

        output.push_str(&format!("  root_path:\n    {:?}\n", self.root_path));
        output.push_str(&format!(
            "  bundle_urls:\n{}",
            url_to_str(&self.bundle_urls)
        ));
        output.push_str(&format!("  bundles:\n{}", target_to_str(&self.bundles)));
        output.push_str(&format!("  subscenes:\n{}", target_to_str(&self.subscenes)));
        output.push_str(&format!("  dylibs:\n{}", target_to_str(&self.dylibs)));
        output.push_str(&format!("  files:\n{}", target_to_str(&self.files)));
        output.push_str(&format!("  zips:\n{}", target_to_str(&self.zips)));
        f.write_str(&output)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::BuildMap;
    use crate::scan_files;

    #[test]
    fn circular_dep_test() {
        let root_path = r"../tests/pkg-dependencies/";
        let asset_path = "../tests/pkg-dependencies/CircularDep";
        let patterns = ["**/.pkg"];
        let pkgs = scan_files(asset_path, &patterns);

        let mut build_map = BuildMap::new(root_path).unwrap();

        build_map.insert(asset_path, pkgs).unwrap();

        let target_path = "CircularDep/A";
        let deps = build_map.resolve_bundle_deps(target_path).unwrap();
        assert_eq!(deps.is_circular, true);
    }
}
