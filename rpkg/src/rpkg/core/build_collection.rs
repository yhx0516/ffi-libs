#[derive(Default)]
pub struct BuildCollection {
    addon_paths: Vec<String>,
    pkg_paths: Vec<String>,
}

impl BuildCollection {
    pub fn new() -> BuildCollection {
        BuildCollection::default()
    }

    pub fn set_addon_paths(&mut self, paths: Vec<String>) {
        self.addon_paths = paths;
    }

    pub fn add_addon_path(&mut self, path: String) {
        self.addon_paths.push(path);
    }

    pub fn get_addon_paths(&self) -> &Vec<String> {
        &self.addon_paths
    }

    pub fn set_pkg_path(&mut self, paths: Vec<String>) {
        self.pkg_paths = paths;
    }

    pub fn add_pkg_path(&mut self, path: String) {
        self.pkg_paths.push(path);
    }

    pub fn get_pkg_paths(&self) -> &Vec<String> {
        &self.pkg_paths
    }
}
