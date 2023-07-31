use funny_utils_rs::scan;
use std::path::Path;

pub fn scan_files(root_path: impl AsRef<Path>, patterns: &[impl AsRef<str>]) -> Vec<String> {
    funny_utils_rs::scan::scan_files(root_path, patterns)
}

pub fn scan_files_rel_path(
    root_path: impl AsRef<Path>,
    patterns: &[impl AsRef<str>],
) -> Vec<String> {
    scan::scan_files_rel_path(root_path, patterns)
}

pub fn scan_files_block_by_pkg(
    root_path: impl AsRef<Path>,
    patterns: &[impl AsRef<str>],
) -> Vec<String> {
    let blocks = [".pkg"];
    let mut patterns: Vec<&str> = patterns.iter().map(|e| e.as_ref()).collect();
    patterns.push("!**/*.pkg");

    scan::scan_files_with_blocks(root_path, &patterns, &blocks)
}

pub fn scan_files_block_by_manifest(
    root_path: impl AsRef<Path>,
    patterns: &[impl AsRef<str>],
) -> Vec<String> {
    let blocks = &["manifest.toml"];
    scan::scan_files_with_blocks(root_path, patterns, blocks)
}

pub fn scan_files_block_by_pkg_manifest(
    root_path: impl AsRef<Path>,
    patterns: &[impl AsRef<str>],
) -> Vec<String> {
    let mut patterns: Vec<&str> = patterns.iter().map(|e| e.as_ref()).collect();
    patterns.push("!**/*.pkg");
    let blocks = [".pkg", "manifest.toml"];

    scan::scan_files_with_blocks(root_path, &patterns, &blocks)
}
