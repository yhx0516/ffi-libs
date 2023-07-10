use std::fmt::Display;

// 0 - path
// 1 = url
#[derive(Debug)]
pub struct Assets(Vec<(String, String)>);

impl Assets {
    pub fn new() -> Assets {
        Assets(Vec::new())
    }

    pub fn push_asset(&mut self, path: String, url: String) {
        self.0.push((path, url));
    }

    pub fn get_paths(&self) -> Vec<&String> {
        self.0.iter().map(|(path, _)| path).collect()
    }

    pub fn get_urls(&self) -> Vec<&String> {
        self.0.iter().map(|(_, url)| url).collect()
    }
}

impl Display for Assets {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        for (path, url) in &self.0 {
            output.push_str(&format!("path:{}, url:{}\n", path, url));
        }
        f.write_str(&output)
    }
}
