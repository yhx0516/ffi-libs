use std::collections::BTreeMap;
use std::fmt::Display;
use std::path::Path;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

use rutils::path::canonicalize_path;
use rutils::path::norm_path;
use rutils::path::norm_path_extreme;

use crate::core::resolve_build_deps;
use crate::core::Assets;
use crate::core::Dependencies;
use crate::core::TargetPaths;
use crate::BuildTarget;
use crate::TomlPKG;

use crate::build_target_url;
use crate::resolve_target_path;
use crate::scan_files_block_pkg_manifest;

#[derive(Default)]
pub struct BuildMap {
    /// project path
    root_path: String,

    /// key: pkg path, value: target paths
    target_paths: BTreeMap<String, TargetPaths>,

    /// key: mount path, value: target paths
    addon_target_paths: BTreeMap<String, TargetPaths>,

    /// key: bundle path, value: bundle url
    bundle_urls: BTreeMap<String, String>,

    /// key: bundle path, value: TomlBundle target
    bundle_targets: BTreeMap<String, Box<dyn BuildTarget>>,

    /// key: subscene path, value: TomlSubscene target
    subscene_targets: BTreeMap<String, Box<dyn BuildTarget>>,

    /// key: file path, value: TomlFile target
    file_targets: BTreeMap<String, Box<dyn BuildTarget>>,

    /// key: dylib path, value: TomlDylib target
    dylib_targets: BTreeMap<String, Box<dyn BuildTarget>>,

    /// key: zip path, value: TomlZip target
    zip_targets: BTreeMap<String, Box<dyn BuildTarget>>,

    /// key: bundle path, value: Assets
    bundle_assets: BTreeMap<String, Assets>,

    /// key: subscene path, value: Assets
    subscene_assets: BTreeMap<String, Assets>,

    /// key: file path, value: Assets
    file_assets: BTreeMap<String, Assets>,

    /// key: dylib path, value: Assets
    dylib_assets: BTreeMap<String, Assets>,

    /// key: zip path, value: Assets
    zip_assets: BTreeMap<String, Assets>,
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

    pub fn get_bundle_paths(&self, addon_path: impl AsRef<str>) -> Result<Vec<&String>> {
        let addon_path = norm_path(canonicalize_path(addon_path.as_ref())?);
        match self.addon_target_paths.get(&addon_path) {
            Some(target_paths) => Ok(target_paths.get_bundles()),
            None => Ok(Vec::new()),
        }
    }

    pub fn get_subscene_paths(&self, addon_path: impl AsRef<str>) -> Result<Vec<&String>> {
        let addon_path = norm_path(canonicalize_path(addon_path.as_ref())?);
        match self.addon_target_paths.get(&addon_path) {
            Some(target_paths) => Ok(target_paths.get_subscenes()),
            None => Ok(Vec::new()),
        }
    }

    pub fn get_file_paths(&self, addon_path: impl AsRef<str>) -> Result<Vec<&String>> {
        let addon_path = norm_path(canonicalize_path(addon_path.as_ref())?);
        match self.addon_target_paths.get(&addon_path) {
            Some(target_paths) => Ok(target_paths.get_files()),
            None => Ok(Vec::new()),
        }
    }

    pub fn get_dylib_paths(&self, addon_path: impl AsRef<str>) -> Result<Vec<&String>> {
        let addon_path = norm_path(canonicalize_path(addon_path.as_ref())?);
        match self.addon_target_paths.get(&addon_path) {
            Some(target_paths) => Ok(target_paths.get_dylibs()),
            None => Ok(Vec::new()),
        }
    }

    pub fn get_zip_paths(&self, addon_path: impl AsRef<str>) -> Result<Vec<&String>> {
        let addon_path = norm_path(canonicalize_path(addon_path.as_ref())?);
        match self.addon_target_paths.get(&addon_path) {
            Some(target_paths) => Ok(target_paths.get_zips()),
            None => Ok(Vec::new()),
        }
    }

    pub fn get_bundle_paths_from_pkg(&self, pkg_path: impl AsRef<str>) -> Result<Vec<&String>> {
        let pkg_path = norm_path(canonicalize_path(pkg_path.as_ref())?);
        match self.target_paths.get(&pkg_path) {
            Some(target_paths) => Ok(target_paths.get_bundles()),
            None => Ok(Vec::new()),
        }
    }

    pub fn get_subscene_paths_from_pkg(&self, pkg_path: impl AsRef<str>) -> Result<Vec<&String>> {
        let pkg_path = norm_path(canonicalize_path(pkg_path.as_ref())?);
        match self.target_paths.get(&pkg_path) {
            Some(target_paths) => Ok(target_paths.get_subscenes()),
            None => Ok(Vec::new()),
        }
    }

    pub fn get_file_paths_from_pkg(&self, pkg_path: impl AsRef<str>) -> Result<Vec<&String>> {
        let pkg_path = norm_path(canonicalize_path(pkg_path.as_ref())?);
        match self.target_paths.get(&pkg_path) {
            Some(target_paths) => Ok(target_paths.get_files()),
            None => Ok(Vec::new()),
        }
    }

    pub fn get_dylib_paths_from_pkg(&self, pkg_path: impl AsRef<str>) -> Result<Vec<&String>> {
        let pkg_path = norm_path(canonicalize_path(pkg_path.as_ref())?);
        match self.target_paths.get(&pkg_path) {
            Some(target_paths) => Ok(target_paths.get_dylibs()),
            None => Ok(Vec::new()),
        }
    }

    pub fn get_zip_paths_from_pkg(&self, pkg_path: impl AsRef<str>) -> Result<Vec<&String>> {
        let pkg_path = norm_path(canonicalize_path(pkg_path.as_ref())?);
        match self.target_paths.get(&pkg_path) {
            Some(target_paths) => Ok(target_paths.get_zips()),
            None => Ok(Vec::new()),
        }
    }

    pub fn get_bundle_assets(&self, target_path: impl AsRef<str>) -> Vec<&String> {
        match self.bundle_assets.get(target_path.as_ref()) {
            Some(assets) => assets.get_paths(),
            None => Vec::new(),
        }
    }

    pub fn get_subscene_assets(&self, target_path: impl AsRef<str>) -> Vec<&String> {
        match self.subscene_assets.get(target_path.as_ref()) {
            Some(assets) => assets.get_paths(),
            None => Vec::new(),
        }
    }

    pub fn get_file_assets(&self, target_path: impl AsRef<str>) -> Vec<&String> {
        match self.file_assets.get(target_path.as_ref()) {
            Some(assets) => assets.get_paths(),
            None => Vec::new(),
        }
    }

    pub fn get_dylib_assets(&self, target_path: impl AsRef<str>) -> Vec<&String> {
        match self.dylib_assets.get(target_path.as_ref()) {
            Some(assets) => assets.get_paths(),
            None => Vec::new(),
        }
    }

    pub fn get_zip_assets(&self, target_path: impl AsRef<str>) -> Vec<&String> {
        match self.zip_assets.get(target_path.as_ref()) {
            Some(assets) => assets.get_paths(),
            None => Vec::new(),
        }
    }

    pub fn get_asset_urls(&self, addon_path: impl AsRef<str>) -> Result<Vec<&String>> {
        let addon_path = addon_path.as_ref();
        let mut asset_urls = Vec::new();

        for target_path in self.get_bundle_paths(addon_path)? {
            if let Some(assets) = self.bundle_assets.get(target_path) {
                asset_urls.append(&mut assets.get_urls());
            }
        }

        for target_path in self.get_subscene_paths(addon_path)? {
            if let Some(assets) = self.subscene_assets.get(target_path) {
                asset_urls.append(&mut assets.get_urls());
            }
        }

        for target_path in self.get_file_paths(addon_path)? {
            if let Some(assets) = self.file_assets.get(target_path) {
                asset_urls.append(&mut assets.get_urls());
            }
        }

        for target_path in self.get_dylib_paths(addon_path)? {
            if let Some(assets) = self.dylib_assets.get(target_path) {
                asset_urls.append(&mut assets.get_urls());
            }
        }

        for target_path in self.get_zip_paths(addon_path)? {
            if let Some(assets) = self.zip_assets.get(target_path) {
                asset_urls.append(&mut assets.get_urls());
            }
        }

        Ok(asset_urls)
    }

    // insert
    pub fn insert(
        &mut self,
        addon_path: impl AsRef<str>,
        pkg_paths: Vec<impl AsRef<Path>>,
    ) -> Result<()> {
        let root_path = self.get_root_path().clone();
        let addon_path = canonicalize_path(addon_path.as_ref())?;
        let mut addon_target_paths = TargetPaths::new();

        for pkg_path in pkg_paths {
            let pkg_path = pkg_path.as_ref();
            let cur_path = pkg_path.parent().unwrap();
            let toml_pkg = TomlPKG::parse(&pkg_path)?;
            let mut target_paths = TargetPaths::new();

            // bundle
            for target in toml_pkg.bundles.unwrap_or(Vec::new()) {
                let target_path = resolve_target_path(&root_path, cur_path, target.path.as_ref())?;
                target_paths.push_bundle(&target_path)?;

                // update bundle_targets map
                self.bundle_targets
                    .entry(target_path.clone())
                    .or_insert(Box::new(target));

                // update bundle_url map
                {
                    let target_url = build_target_url(&root_path, &addon_path, &target_path)?;
                    let target_path = target_path.to_lowercase();
                    self.bundle_urls.entry(target_path).or_insert(target_url);
                }

                // update bundle_assets map
                let assets = self.scan_bundle_assets(norm_path(&addon_path), &target_path)?;
                self.bundle_assets.entry(target_path).or_insert(assets);
            }

            // subscene
            for target in toml_pkg.subscenes.unwrap_or(Vec::new()) {
                let target_path = resolve_target_path(&root_path, cur_path, target.path.as_ref())?;
                target_paths.push_subscene(&target_path)?;

                // update subscene_targets map
                self.subscene_targets
                    .entry(target_path.clone())
                    .or_insert(Box::new(target));

                // update bundle_url map
                {
                    let target_url = build_target_url(&root_path, &addon_path, &target_path)?;
                    let target_path = target_path.to_lowercase();
                    self.bundle_urls.entry(target_path).or_insert(target_url);
                }

                // update subscene_assets map
                let assets = self.scan_subscene_assets(norm_path(&addon_path), &target_path)?;
                self.subscene_assets.entry(target_path).or_insert(assets);
            }

            // file
            for target in toml_pkg.files.unwrap_or(Vec::new()) {
                let target_path = resolve_target_path(&root_path, cur_path, target.path.as_ref())?;
                target_paths.push_file(&target_path)?;

                // update file_targets map
                self.file_targets
                    .entry(target_path.clone())
                    .or_insert(Box::new(target));

                // update file_assets map
                let assets = self.scan_file_assets(norm_path(&addon_path), &target_path)?;
                self.file_assets.entry(target_path).or_insert(assets);
            }

            // dylib
            for target in toml_pkg.dylibs.unwrap_or(Vec::new()) {
                let target_path = resolve_target_path(&root_path, cur_path, target.path.as_ref())?;
                target_paths.push_dylib(&target_path)?;

                // update dylib_targets map
                self.dylib_targets
                    .entry(target_path.clone())
                    .or_insert(Box::new(target));

                // update dylib_assets map
                let assets = self.scan_dylib_assets(norm_path(&addon_path), &target_path)?;
                self.dylib_assets.entry(target_path).or_insert(assets);
            }

            // zip
            for target in toml_pkg.zips.unwrap_or(Vec::new()) {
                let target_path = resolve_target_path(&root_path, cur_path, target.path.as_ref())?;
                target_paths.push_zip(&target_path)?;

                // update zip_targets map
                self.zip_targets
                    .entry(target_path.clone())
                    .or_insert(Box::new(target));

                // update dylib_assets map
                let assets = self.scan_zip_assets(norm_path(&addon_path), &target_path)?;
                self.zip_assets.entry(target_path).or_insert(assets);
            }

            // append addon_target_paths
            addon_target_paths.append(&target_paths);

            // update pkgs map
            self.target_paths.insert(norm_path(pkg_path), target_paths);
        }

        // update addon_target_paths
        self.addon_target_paths
            .insert(norm_path(addon_path), addon_target_paths);

        Ok(())
    }

    pub fn find_bundle_url(&self, bundle_path: impl AsRef<str>) -> Result<String> {
        let bundle_path = bundle_path.as_ref().to_lowercase();
        match self.bundle_urls.get(&bundle_path) {
            Some(v) => Ok(v.to_owned()),
            None => Err(anyhow!("not found url {} in map", bundle_path)),
        }
    }

    pub fn resolve_bundle_deps(&self, target_path: impl AsRef<str>) -> Result<Dependencies> {
        let root_path = self.get_root_path();
        resolve_build_deps(
            root_path,
            &norm_path(target_path.as_ref()),
            &self.bundle_targets,
        )
        .with_context(|| "type: bundle")
    }

    pub fn resolve_subscene_deps(&self, target_path: impl AsRef<str>) -> Result<Dependencies> {
        let root_path = self.get_root_path();
        resolve_build_deps(
            root_path,
            &norm_path(target_path.as_ref()),
            &self.subscene_targets,
        )
        .with_context(|| "type: subscene")
    }

    pub fn resolve_dylib_deps(&self, target_path: impl AsRef<str>) -> Result<Dependencies> {
        let root_path = self.get_root_path();
        resolve_build_deps(
            root_path,
            &norm_path(target_path.as_ref()),
            &self.dylib_targets,
        )
        .with_context(|| "type: dylib")
    }

    pub fn resolve_file_deps(&self, target_path: impl AsRef<str>) -> Result<Dependencies> {
        let root_path = self.get_root_path();
        resolve_build_deps(
            root_path,
            &norm_path(target_path.as_ref()),
            &self.file_targets,
        )
        .with_context(|| "type: file")
    }

    pub fn resolve_zip_deps(&self, target_path: impl AsRef<str>) -> Result<Dependencies> {
        let root_path = self.get_root_path();
        resolve_build_deps(
            root_path,
            &norm_path(target_path.as_ref()),
            &self.zip_targets,
        )
        .with_context(|| "type: zip")
    }

    pub fn scan_bundle_assets(
        &self,
        addon_path: impl AsRef<str>,
        target_path: impl AsRef<str>,
    ) -> Result<Assets> {
        let root_path = self.get_root_path();
        inner_scan_assets(root_path, addon_path, target_path, &self.bundle_targets)
    }

    pub fn scan_subscene_assets(
        &self,
        addon_path: impl AsRef<str>,
        target_path: impl AsRef<str>,
    ) -> Result<Assets> {
        let root_path = self.get_root_path();
        inner_scan_assets(root_path, addon_path, target_path, &self.subscene_targets)
    }

    pub fn scan_dylib_assets(
        &self,
        addon_path: impl AsRef<str>,
        target_path: impl AsRef<str>,
    ) -> Result<Assets> {
        let root_path = self.get_root_path();
        inner_scan_assets(root_path, addon_path, target_path, &self.dylib_targets)
    }

    pub fn scan_file_assets(
        &self,
        addon_path: impl AsRef<str>,
        target_path: impl AsRef<str>,
    ) -> Result<Assets> {
        let root_path = self.get_root_path();
        inner_scan_assets(root_path, addon_path, target_path, &self.file_targets)
    }

    pub fn scan_zip_assets(
        &self,
        addon_path: impl AsRef<str>,
        target_path: impl AsRef<str>,
    ) -> Result<Assets> {
        let root_path = self.get_root_path();
        inner_scan_assets(root_path, addon_path, target_path, &self.zip_targets)
    }
}

fn inner_scan_assets(
    root_path: impl AsRef<str>,
    addon_path: impl AsRef<str>,
    target_path: impl AsRef<str>,
    target_map: &BTreeMap<String, Box<dyn BuildTarget>>,
) -> Result<Assets> {
    let root_path = root_path.as_ref();
    let addon_path = norm_path(canonicalize_path(addon_path.as_ref())?);
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
        let url = target.build_asset_url(&addon_path, &target_path, &item);

        assets.push_asset(norm_path_extreme(rel_path), url)
    }

    Ok(assets)
}

impl Display for BuildMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        let pkgs_to_str = |target_map: &BTreeMap<String, TargetPaths>| {
            let mut output = String::new();
            for (key, val) in target_map {
                output.push_str(&format!("  {}:\n", key));
                output.push_str(&format!("{}\n", val.to_string()));
            }
            output
        };

        let urls_to_str = |target_map: &BTreeMap<String, String>| {
            let mut output = String::new();
            for (key, val) in target_map {
                output.push_str(&format!("  {}  {}\n", key, val));
            }
            output
        };

        let targets_to_str = |target_map: &BTreeMap<String, Box<dyn BuildTarget>>| {
            let mut output = String::new();
            for (key, val) in target_map {
                output.push_str(&format!("  {}  {}\n", key, val.display()));
            }
            output
        };

        let assets_to_str = |target_map: &BTreeMap<String, Assets>| {
            let mut output = String::new();
            for (key, val) in target_map {
                output.push_str(&format!("  {}:\n", key));
                for v in val.get_paths() {
                    output.push_str(&format!("    {}\n", v.to_string()));
                }
            }
            output
        };

        output.push_str("root_path:\n    ");
        output.push_str(&self.root_path);
        output.push_str("\n");

        output.push_str("pkgs:\n    ");
        output.push_str(&pkgs_to_str(&self.target_paths));

        output.push_str("mount_pkgs:\n    ");
        output.push_str(&pkgs_to_str(&self.addon_target_paths));

        output.push_str("bundle_urls:\n");
        output.push_str(&urls_to_str(&self.bundle_urls));

        output.push_str("bundle_targets:\n");
        output.push_str(&targets_to_str(&self.bundle_targets));

        output.push_str("subscenes_targets:\n");
        output.push_str(&targets_to_str(&self.subscene_targets));

        output.push_str("files_targets:\n");
        output.push_str(&targets_to_str(&self.file_targets));

        output.push_str("dylibs_targets:\n");
        output.push_str(&targets_to_str(&self.dylib_targets));

        output.push_str("zips_targets:\n");
        output.push_str(&targets_to_str(&self.zip_targets));

        output.push_str("bundle_assets:\n");
        output.push_str(&assets_to_str(&self.bundle_assets));

        output.push_str("subscenes_assets:\n");
        output.push_str(&assets_to_str(&self.subscene_assets));

        output.push_str("dylibs_assets:\n");
        output.push_str(&assets_to_str(&self.dylib_assets));

        output.push_str("files_assets:\n");
        output.push_str(&assets_to_str(&self.file_assets));

        output.push_str("zips_assets:\n");
        output.push_str(&assets_to_str(&self.zip_assets));

        f.write_str(&output)
    }
}

#[cfg(test)]
mod tests {
    use crate::core::BuildMap;
    use crate::scan_files;

    #[test]
    fn circular_dep_test() {
        let root_path = "../tests/pkg-dependencies/";
        let asset_path = "../tests/pkg-dependencies/CircularDep";
        let patterns = ["**/.pkg"];
        let pkgs = scan_files(asset_path, &patterns);

        let mut build_map = BuildMap::new(root_path).unwrap();

        build_map.insert(asset_path, pkgs).unwrap();

        let target_path = "CircularDep/A";
        let deps = build_map.resolve_bundle_deps(target_path).err().unwrap();
        let e = format!("{}, {}", deps.root_cause().to_string(), deps.to_string());
        println!("{e}");
        // assert_eq!(deps.is_circular, true);
    }
}
