use std::collections::BTreeMap;
use std::fmt::Display;
use std::path::Path;

use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;

use funny_utils_rs::path::canonicalize_path;
use funny_utils_rs::path::norm_path;
use funny_utils_rs::path::trim_path2;

use crate::core::resolve_build_deps;
use crate::core::Assets;
use crate::core::BuildCollection;
use crate::core::Dependencies;
use crate::core::PKGTargetPaths;
use crate::BuildTarget;
use crate::TomlBundle;
use crate::TomlDylib;
use crate::TomlFile;
use crate::TomlPKG;
use crate::TomlSubscene;
use crate::TomlZip;

use crate::resolve_target_path;
use crate::scan_files_block_by_manifest;
use crate::scan_files_block_by_pkg_manifest;
use crate::to_bundle_path;
#[derive(Default)]
pub struct BuildMap {
    /// project path
    root_path: String,

    /// key: pkg path, value: target paths
    pkg_to_targets: BTreeMap<String, PKGTargetPaths>,

    /// key: addon path, value: target paths
    addon_to_targets: BTreeMap<String, PKGTargetPaths>,

    /// key: addon path, value: pkg paths
    addon_to_pkgs: BTreeMap<String, Vec<String>>,

    /// key: target path, value: bundle path
    bundle_paths: BTreeMap<String, String>,

    /// key: target path, value: TomlBundle target
    bundle_targets: BTreeMap<String, Box<dyn BuildTarget>>,

    /// key: target path, value: TomlSubscene target
    subscene_targets: BTreeMap<String, Box<dyn BuildTarget>>,

    /// key: target path, value: TomlFile target
    file_targets: BTreeMap<String, Box<dyn BuildTarget>>,

    /// key: target path, value: TomlDylib target
    dylib_targets: BTreeMap<String, Box<dyn BuildTarget>>,

    /// key: target path, value: TomlZip target
    zip_targets: BTreeMap<String, Box<dyn BuildTarget>>,

    /// key: target path, value: Assets
    bundle_assets: BTreeMap<String, Assets>,

    /// key: target path, value: Assets
    subscene_assets: BTreeMap<String, Assets>,

    /// key: target path, value: Assets
    file_assets: BTreeMap<String, Assets>,

    /// key: target path, value: Assets
    dylib_assets: BTreeMap<String, Assets>,

    /// key: target path, value: Assets
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

    pub fn get_target_types(&self, addon_path: impl AsRef<str>) -> Result<Vec<&str>> {
        let addon_path = norm_path(canonicalize_path(addon_path.as_ref())?);
        match self.addon_to_targets.get(&addon_path) {
            Some(pkg_to_targets) => Ok(pkg_to_targets.get_target_types()),
            None => Ok(Vec::new()),
        }
    }

    pub fn get_target_types_from_pkg(&self, pkg_path: impl AsRef<str>) -> Result<Vec<&str>> {
        let pkg_path = norm_path(canonicalize_path(pkg_path.as_ref())?);
        match self.pkg_to_targets.get(&pkg_path) {
            Some(pkg_to_targets) => Ok(pkg_to_targets.get_target_types()),
            None => Ok(Vec::new()),
        }
    }

    pub fn get_target_paths(
        &self,
        addon_path: impl AsRef<str>,
        target_type: impl AsRef<str>,
    ) -> Result<Vec<&String>> {
        let addon_path = norm_path(canonicalize_path(addon_path.as_ref())?);
        match self.addon_to_targets.get(&addon_path) {
            Some(pkg_to_targets) => Ok(pkg_to_targets.get_target_paths(target_type)),
            None => Ok(Vec::new()),
        }
    }

    pub fn get_target_paths_from_pkg(
        &self,
        pkg_path: impl AsRef<str>,
        target_type: impl AsRef<str>,
    ) -> Result<Vec<&String>> {
        let pkg_path = norm_path(canonicalize_path(pkg_path.as_ref())?);
        match self.pkg_to_targets.get(&pkg_path) {
            Some(pkg_to_targets) => Ok(pkg_to_targets.get_target_paths(target_type)),
            None => Ok(Vec::new()),
        }
    }

    pub fn seek_build_collection(&self, select_path: impl AsRef<str>) -> Result<BuildCollection> {
        let select_path: String = norm_path(canonicalize_path(select_path.as_ref()).unwrap());
        let path = Path::new(&select_path);

        let mut build_collection = BuildCollection::new();

        // path is addon
        if path.is_dir() {
            if self.is_addon(&select_path) {
                for path in self.get_inner_addon_paths(&select_path) {
                    build_collection.add_addon_path(path.to_string());
                }

                return Ok(build_collection);
            } else {
                let mut pkg_files = None;

                // get inner addon
                if self.has_inner_addon(&select_path) {
                    for path in self.get_inner_addon_paths(&select_path) {
                        build_collection.add_addon_path(path.to_string());
                    }

                    pkg_files = Some(scan_files_block_by_manifest(&select_path, &["**/.pkg"]));

                    // no pkg
                    if matches!(pkg_files, Some(ref files) if files.is_empty()) {
                        return Ok(build_collection);
                    }
                }

                // get outer addon
                if self.has_outer_addon(&select_path) {
                    if let Some(outer_addon_path) = self.get_outer_addon_path(&select_path) {
                        build_collection.add_addon_path(outer_addon_path);
                    }

                    return Ok(build_collection);
                }

                if pkg_files.is_none() {
                    pkg_files = Some(scan_files_block_by_manifest(&select_path, &["**/.pkg"]));
                }

                if let Some(files) = pkg_files {
                    build_collection.set_pkg_path(files);
                }

                return Ok(build_collection);
            }
        } else {
            let directory = norm_path(path.parent().unwrap());

            // path is addon
            if self.is_addon(&directory) {
                if let Some(addon_path) = self.get_addon_path(&directory) {
                    build_collection.add_addon_path(addon_path);
                }

                return Ok(build_collection);
            }

            let is_root_manifest = select_path == format!("{}/manifest.toml", &self.root_path);

            // path is root manifest.toml
            if is_root_manifest {
                for path in self.get_inner_addon_paths(&directory) {
                    build_collection.add_addon_path(path.to_string());
                }

                return Ok(build_collection);
            }

            // get outer addon
            if self.has_outer_addon(&directory) {
                if let Some(outer_addon_path) = self.get_outer_addon_path(&directory) {
                    build_collection.add_addon_path(outer_addon_path);
                }

                return Ok(build_collection);
            }

            let pkg_path = Path::new(&directory).join(".pkg");

            if pkg_path.exists() {
                build_collection.add_pkg_path(norm_path(pkg_path));
            }
        }

        Ok(build_collection)
    }

    pub fn get_addon_pkg_paths(&self, addon_path: impl AsRef<str>) -> Result<Vec<String>> {
        let addon_path: String = norm_path(canonicalize_path(addon_path.as_ref())?);
        match self.addon_to_pkgs.get(&addon_path) {
            Some(addon_to_pkgs) => Ok(addon_to_pkgs.to_owned()),
            None => Ok(Vec::new()),
        }
    }

    pub fn is_addon(&self, path: impl AsRef<str>) -> bool {
        let path: String = norm_path(canonicalize_path(path.as_ref()).unwrap());

        for addon_path in self.addon_to_targets.keys() {
            if path.eq(addon_path) {
                return true;
            }
        }

        return false;
    }

    pub fn get_addon_path(&self, path: impl AsRef<str>) -> Option<String> {
        let path: String = norm_path(canonicalize_path(path.as_ref()).unwrap());

        for addon_path in self.addon_to_targets.keys() {
            if path.eq(addon_path) {
                let addon = &addon_path[self.root_path.len() + 1..];
                return Some(addon.to_string());
            }
        }

        return None;
    }

    pub fn has_inner_addon(&self, path: impl AsRef<str>) -> bool {
        let path: String = norm_path(canonicalize_path(path.as_ref()).unwrap());

        for addon_path in self.addon_to_targets.keys() {
            if addon_path.starts_with(&path) {
                return true;
            }
        }

        return false;
    }

    pub fn get_inner_addon_paths(&self, path: impl AsRef<str>) -> Vec<&str> {
        let path: String = norm_path(canonicalize_path(path.as_ref()).unwrap());
        let mut addon_paths = Vec::new();

        for addon_path in self.addon_to_targets.keys() {
            if addon_path == &self.root_path {
                addon_paths.push("./");
            } else if addon_path.starts_with(&path) {
                let addon = &addon_path[self.root_path.len() + 1..];
                addon_paths.push(addon);
            }
        }

        return addon_paths;
    }

    pub fn has_outer_addon(&self, path: impl AsRef<str>) -> bool {
        let path: String = norm_path(canonicalize_path(path.as_ref()).unwrap());

        for addon_path in self.addon_to_targets.keys() {
            if path.len() == addon_path.len() {
                return false;
            }

            if path.starts_with(addon_path) {
                return true;
            }
        }

        return false;
    }

    pub fn get_outer_addon_path(&self, path: impl AsRef<str>) -> Option<String> {
        let path: String = norm_path(canonicalize_path(path.as_ref()).unwrap());
        let mut outer_addon_path = String::new();

        for addon_path in self.addon_to_targets.keys() {
            if path.len() == addon_path.len() {
                continue;
            }

            if addon_path != &self.root_path && path.starts_with(addon_path) {
                let addon = &addon_path[self.root_path.len() + 1..];

                if outer_addon_path.len() < addon.len() {
                    outer_addon_path = addon.to_string();
                }
            }
        }

        return Some(outer_addon_path);
    }

    pub fn get_build_target(
        &self,
        target_path: impl AsRef<str>,
        target_type: impl AsRef<str>,
    ) -> Result<&Box<dyn BuildTarget>> {
        let target_path = target_path.as_ref();
        let target_type = target_type.as_ref();

        let res = match target_type {
            TomlBundle::TYPE_NAME => self.bundle_targets.get(target_path),
            TomlSubscene::TYPE_NAME => self.subscene_targets.get(target_path),
            TomlFile::TYPE_NAME => self.file_targets.get(target_path),
            TomlDylib::TYPE_NAME => self.dylib_targets.get(target_path),
            TomlZip::TYPE_NAME => self.zip_targets.get(target_path),
            _ => None,
        };

        match res {
            Some(target) => Ok(target),
            None => Err(anyhow!("not found {} target {}", target_type, target_path)),
        }
    }

    pub fn get_target_assets(
        &self,
        target_path: impl AsRef<str>,
        target_type: impl AsRef<str>,
    ) -> Vec<&String> {
        let target_path = target_path.as_ref();
        let target_type = target_type.as_ref();

        let assets = match target_type {
            TomlBundle::TYPE_NAME => self.bundle_assets.get(target_path),
            TomlSubscene::TYPE_NAME => self.subscene_assets.get(target_path),
            TomlFile::TYPE_NAME => self.file_assets.get(target_path),
            TomlDylib::TYPE_NAME => self.dylib_assets.get(target_path),
            TomlZip::TYPE_NAME => self.zip_assets.get(target_path),
            _ => None,
        };

        match assets {
            Some(assets) => assets.get_paths(),
            None => Vec::new(),
        }
    }

    pub fn get_target_asset_urls(
        &self,
        target_path: impl AsRef<str>,
        target_type: impl AsRef<str>,
    ) -> Vec<&String> {
        let target_path = target_path.as_ref();
        let target_type = target_type.as_ref();

        let assets = match target_type {
            TomlBundle::TYPE_NAME => self.bundle_assets.get(target_path),
            TomlSubscene::TYPE_NAME => self.subscene_assets.get(target_path),
            TomlFile::TYPE_NAME => self.file_assets.get(target_path),
            TomlDylib::TYPE_NAME => self.dylib_assets.get(target_path),
            TomlZip::TYPE_NAME => self.zip_assets.get(target_path),
            _ => None,
        };

        match assets {
            Some(assets) => assets.get_urls(),
            None => Vec::new(),
        }
    }

    // contain:
    //   - bundle assets
    //   - subscene assets
    //   - file assets
    //   - dylib assets
    //   - zip assets
    pub fn get_asset_urls(&self, addon_path: impl AsRef<str>) -> Result<Vec<&String>> {
        let addon_path = addon_path.as_ref();
        let mut asset_urls = Vec::new();

        for target_type in self.get_target_types(addon_path)? {
            for target_path in self.get_target_paths(addon_path, target_type)? {
                let mut urls = self.get_target_asset_urls(target_path, target_type);
                asset_urls.append(&mut urls);
            }
        }

        asset_urls.sort();
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
        let mut addon_to_targets = PKGTargetPaths::new();

        // update addon_to_pkgs map
        self.addon_to_pkgs.insert(
            norm_path(&addon_path),
            pkg_paths.iter().map(|s| norm_path(s)).collect(),
        );

        for pkg_path in pkg_paths {
            let pkg_path = pkg_path.as_ref();
            let cur_path = pkg_path.parent().unwrap();
            let toml_pkg = TomlPKG::parse(&pkg_path)?;
            let mut pkg_to_targets = PKGTargetPaths::new();

            // bundle
            for target in toml_pkg.bundles.unwrap_or(Vec::new()) {
                let target_path = resolve_target_path(&root_path, cur_path, target.path.as_ref())?;
                pkg_to_targets.push_bundle(&target_path)?;

                // update bundle_targets map
                self.bundle_targets
                    .entry(target_path.clone())
                    .or_insert(Box::new(target));

                // update bundle_url map
                {
                    let target_url = to_bundle_path(&root_path, &addon_path, &target_path)?;
                    let target_path = target_path.to_lowercase();
                    self.bundle_paths.entry(target_path).or_insert(target_url);
                }

                // update bundle_assets map
                let target_type = TomlBundle::TYPE_NAME;
                let assets = self.scan_assets(norm_path(&addon_path), &target_path, target_type)?;
                self.bundle_assets.entry(target_path).or_insert(assets);
            }

            // subscene
            for target in toml_pkg.subscenes.unwrap_or(Vec::new()) {
                let target_path = resolve_target_path(&root_path, cur_path, target.path.as_ref())?;
                pkg_to_targets.push_subscene(&target_path)?;

                // update subscene_targets map
                self.subscene_targets
                    .entry(target_path.clone())
                    .or_insert(Box::new(target));

                // update bundle_url map
                {
                    let target_url = to_bundle_path(&root_path, &addon_path, &target_path)?;
                    let target_path = target_path.to_lowercase();
                    self.bundle_paths.entry(target_path).or_insert(target_url);
                }

                // update subscene_assets map
                let target_type = TomlSubscene::TYPE_NAME;
                let assets = self.scan_assets(norm_path(&addon_path), &target_path, target_type)?;
                self.subscene_assets.entry(target_path).or_insert(assets);
            }

            // file
            for target in toml_pkg.files.unwrap_or(Vec::new()) {
                let target_path = resolve_target_path(&root_path, cur_path, target.path.as_ref())?;
                pkg_to_targets.push_file(&target_path)?;

                // update file_targets map
                self.file_targets
                    .entry(target_path.clone())
                    .or_insert(Box::new(target));

                // update file_assets map
                let target_type = TomlFile::TYPE_NAME;
                let assets = self.scan_assets(norm_path(&addon_path), &target_path, target_type)?;
                self.file_assets.entry(target_path).or_insert(assets);
            }

            // dylib
            for target in toml_pkg.dylibs.unwrap_or(Vec::new()) {
                let target_path = resolve_target_path(&root_path, cur_path, target.path.as_ref())?;
                pkg_to_targets.push_dylib(&target_path)?;

                // update dylib_targets map
                self.dylib_targets
                    .entry(target_path.clone())
                    .or_insert(Box::new(target));

                // update dylib_assets map
                let target_type = TomlDylib::TYPE_NAME;
                let assets = self.scan_assets(norm_path(&addon_path), &target_path, target_type)?;
                self.dylib_assets.entry(target_path).or_insert(assets);
            }

            // zip
            for target in toml_pkg.zips.unwrap_or(Vec::new()) {
                let target_path = resolve_target_path(&root_path, cur_path, target.path.as_ref())?;
                pkg_to_targets.push_zip(&target_path)?;

                // update zip_targets map
                self.zip_targets
                    .entry(target_path.clone())
                    .or_insert(Box::new(target));

                // update dylib_assets map
                let target_type = TomlZip::TYPE_NAME;
                let assets = self.scan_assets(norm_path(&addon_path), &target_path, target_type)?;
                self.zip_assets.entry(target_path).or_insert(assets);
            }

            // append addon_to_targets
            addon_to_targets.append(&pkg_to_targets);

            // update pkgs map
            self.pkg_to_targets
                .insert(norm_path(pkg_path), pkg_to_targets);
        }

        // update addon_to_targets
        self.addon_to_targets
            .insert(norm_path(addon_path), addon_to_targets);

        Ok(())
    }

    pub fn find_bundle_path(&self, bundle_path: impl AsRef<str>) -> Result<String> {
        let bundle_path = bundle_path.as_ref().to_lowercase();
        match self.bundle_paths.get(&bundle_path) {
            Some(v) => Ok(v.to_owned()),
            None => Err(anyhow!("not found url {} in map", bundle_path)),
        }
    }

    pub fn resolve_target_deps(
        &self,
        target_path: impl AsRef<str>,
        target_type: impl AsRef<str>,
    ) -> Result<Dependencies> {
        let root_path = self.get_root_path();
        let target_path = norm_path(target_path.as_ref());
        let target_type = target_type.as_ref();

        let target_map = match target_type {
            TomlBundle::TYPE_NAME => &self.bundle_targets,
            TomlSubscene::TYPE_NAME => &self.subscene_targets,
            TomlFile::TYPE_NAME => &self.file_targets,
            TomlDylib::TYPE_NAME => &self.dylib_targets,
            TomlZip::TYPE_NAME => &self.zip_targets,
            _ => return Err(anyhow!("not fount {} type", target_type)),
        };

        resolve_build_deps(root_path, &target_path, target_map)
            .with_context(|| format!("type: {}", target_type))
    }

    fn scan_assets(
        &self,
        addon_path: impl AsRef<str>,
        target_path: impl AsRef<str>,
        target_type: impl AsRef<str>,
    ) -> Result<Assets> {
        let root_path = self.get_root_path();
        let target_type = target_type.as_ref();

        let target_map = match target_type {
            TomlBundle::TYPE_NAME => &self.bundle_targets,
            TomlSubscene::TYPE_NAME => &self.subscene_targets,
            TomlFile::TYPE_NAME => &self.file_targets,
            TomlDylib::TYPE_NAME => &self.dylib_targets,
            TomlZip::TYPE_NAME => &self.zip_targets,
            _ => return Err(anyhow!("not fount {} type", target_type)),
        };

        inner_scan_assets(root_path, addon_path, target_path, target_map)
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

    for item in scan_files_block_by_pkg_manifest(path, patterns) {
        let Some(rel_path) = item.strip_prefix(root_path) else {
            return Err(anyhow!("{} is not the prefix of {}",root_path, &target_path));
        };

        let target_path = format!("{}/{}", root_path, target_path);
        let url = target.build_asset_url(&addon_path, &target_path, &item);

        assets.push_asset(trim_path2(rel_path), url)
    }

    Ok(assets)
}

impl Display for BuildMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        let pkgs_to_str = |target_map: &BTreeMap<String, PKGTargetPaths>| {
            let mut output = String::new();
            for (key, val) in target_map {
                output.push_str(&format!("  {}:\n", key));
                output.push_str(&format!("{}\n", val.to_string()));
            }
            output
        };

        let addon_to_strs = |target_map: &BTreeMap<String, Vec<String>>| {
            let mut output = String::new();
            for (key, val) in target_map {
                output.push_str(&format!("  {}:\n", key));

                for v in val {
                    output.push_str(&format!("    {}\n", v.to_string()));
                }
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
        output.push_str(&pkgs_to_str(&self.pkg_to_targets));

        output.push_str("addon_targets:\n    ");
        output.push_str(&pkgs_to_str(&self.addon_to_targets));

        output.push_str("addon_pkgs:\n    ");
        output.push_str(&addon_to_strs(&self.addon_to_pkgs));

        output.push_str("bundle_urls:\n");
        output.push_str(&urls_to_str(&self.bundle_paths));

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
    use funny_utils_rs::scan::scan_files;

    use crate::core::BuildMap;
    use crate::TomlBundle;

    #[test]
    fn circular_dep_test() {
        let root_path = "../tests/pkg-dependencies/";
        let asset_path = "../tests/pkg-dependencies/CircularDep";
        let patterns = ["**/.pkg"];
        let pkgs = scan_files(asset_path, &patterns);

        let mut build_map = BuildMap::new(root_path).unwrap();
        build_map.insert(asset_path, pkgs).unwrap();

        let target_path = "CircularDep/A";
        let target_type = TomlBundle::TYPE_NAME;
        let deps = build_map
            .resolve_target_deps(target_path, target_type)
            .unwrap();
        assert_eq!(deps.is_circular, true);
    }
}
