use anyhow::anyhow;
use anyhow::Result;
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Debug, Default)]
pub struct TargetPaths {
    bundles: HashSet<String>,
    subscenes: HashSet<String>,
    files: HashSet<String>,
    dylibs: HashSet<String>,
    zips: HashSet<String>,
}

impl TargetPaths {
    pub fn new() -> Self {
        TargetPaths::default()
    }

    pub fn get_bundles(&self) -> Vec<&String> {
        self.bundles.iter().map(|v| v).collect()
    }

    pub fn get_subscenes(&self) -> Vec<&String> {
        self.subscenes.iter().map(|v| v).collect()
    }

    pub fn get_files(&self) -> Vec<&String> {
        self.files.iter().map(|v| v).collect()
    }

    pub fn get_dylibs(&self) -> Vec<&String> {
        self.dylibs.iter().map(|v| v).collect()
    }

    pub fn get_zips(&self) -> Vec<&String> {
        self.zips.iter().map(|v| v).collect()
    }

    pub fn push_bundle(&mut self, path: &String) -> Result<()> {
        match self.bundles.insert(path.to_owned()) {
            true => Ok(()),
            false => Err(anyhow!("{} already exist", path)),
        }
    }

    pub fn push_subscene(&mut self, path: &String) -> Result<()> {
        match self.subscenes.insert(path.to_owned()) {
            true => Ok(()),
            false => Err(anyhow!("{} already exist", path)),
        }
    }

    pub fn push_file(&mut self, path: &String) -> Result<()> {
        match self.files.insert(path.to_owned()) {
            true => Ok(()),
            false => Err(anyhow!("{} already exist", path)),
        }
    }

    pub fn push_dylib(&mut self, path: &String) -> Result<()> {
        match self.dylibs.insert(path.to_owned()) {
            true => Ok(()),
            false => Err(anyhow!("{} already exist", path)),
        }
    }

    pub fn push_zip(&mut self, path: &String) -> Result<()> {
        match self.zips.insert(path.to_owned()) {
            true => Ok(()),
            false => Err(anyhow!("{} already exist", path)),
        }
    }

    pub fn append(&mut self, pkg_targets: &TargetPaths) {
        for target in &pkg_targets.bundles {
            self.bundles.insert(target.clone());
        }

        for target in &pkg_targets.subscenes {
            self.subscenes.insert(target.clone());
        }

        for target in &pkg_targets.files {
            self.files.insert(target.clone());
        }

        for target in &pkg_targets.dylibs {
            self.dylibs.insert(target.clone());
        }

        for target in &pkg_targets.zips {
            self.zips.insert(target.clone());
        }
    }
}

impl Display for TargetPaths {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        let targets_to_str = |targets: &HashSet<String>| {
            let mut output = String::new();
            for val in targets {
                output.push_str(&format!("    {}\n", val));
            }
            output
        };

        output.push_str("  bundle_targets:\n");
        output.push_str(&targets_to_str(&self.bundles));

        output.push_str("  subscenes_targets:\n");
        output.push_str(&targets_to_str(&self.subscenes));

        output.push_str("  files_targets:\n");
        output.push_str(&targets_to_str(&self.files));

        output.push_str("  dylibs_targets:\n");
        output.push_str(&targets_to_str(&self.dylibs));

        output.push_str("  zips_targets:\n");
        output.push_str(&targets_to_str(&self.zips));

        f.write_str(&output)
    }
}
