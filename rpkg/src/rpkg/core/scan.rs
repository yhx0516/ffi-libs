use globset::{GlobBuilder, GlobMatcher};
use rutils::path::{canonicalize_path, norm_path};
use std::path::Path;
use walkdir::WalkDir;

struct ScanOptions {
    pub block_by_pkg: bool,
    pub block_by_manifest: bool,
}

pub fn scan_files(root_path: impl AsRef<Path>, patterns: &[impl AsRef<str>]) -> Vec<String> {
    let options = ScanOptions {
        block_by_pkg: false,
        block_by_manifest: false,
    };

    inner_scan_files(root_path, patterns, &options)
}

pub fn scan_files_block_by_pkg(
    root_path: impl AsRef<Path>,
    patterns: &[impl AsRef<str>],
) -> Vec<String> {
    let options = ScanOptions {
        block_by_pkg: true,
        block_by_manifest: false,
    };

    inner_scan_files(root_path, patterns, &options)
}

pub fn scan_files_block_by_manifest(
    root_path: impl AsRef<Path>,
    patterns: &[impl AsRef<str>],
) -> Vec<String> {
    let options = ScanOptions {
        block_by_pkg: false,
        block_by_manifest: true,
    };

    inner_scan_files(root_path, patterns, &options)
}

pub fn scan_files_block_by_pkg_manifest(
    root_path: impl AsRef<Path>,
    patterns: &[impl AsRef<str>],
) -> Vec<String> {
    let options = ScanOptions {
        block_by_pkg: true,
        block_by_manifest: true,
    };

    inner_scan_files(root_path, patterns, &options)
}

fn inner_scan_files(
    root_path: impl AsRef<Path>,
    patterns: &[impl AsRef<str>],
    options: &ScanOptions,
) -> Vec<String> {
    let root_path = root_path.as_ref();
    let git_glob = create_git_glob_macther();

    let mut include_globs = Vec::new();
    let mut exclude_globs = Vec::new();
    create_glob_matcher_from_patterns(&root_path, patterns, &mut include_globs, &mut exclude_globs);

    let mut walk_iter = WalkDir::new(root_path).into_iter();
    let mut include_files = Vec::new();

    loop {
        let entry = match walk_iter.next() {
            Some(Ok(entry)) => entry,
            Some(Err(err)) => {
                eprintln!("{:?}", err);
                continue;
            }
            None => break,
        };
        let path = entry.path();

        if options.block_by_pkg {
            // skip sub directory (.pkg)
            if path.is_dir() && path != root_path && path.join(".pkg").is_file() {
                walk_iter.skip_current_dir();
                continue;
            }

            // skip pkg file
            if path.is_file() && path.ends_with(".pkg") {
                continue;
            }
        }

        if options.block_by_manifest {
            // skip sub directory (manifest.toml)
            if path.is_dir() && path != root_path && path.join("manifest.toml").is_file() {
                walk_iter.skip_current_dir();
                continue;
            }
        }

        // skip .git directory
        if git_glob.is_match(path) {
            walk_iter.skip_current_dir();
            continue;
        }

        // exclude file
        if exclude_globs.iter().any(|m| m.is_match(path)) {
            continue;
        }

        // include file
        if include_globs.iter().any(|m| m.is_match(path)) {
            let path = norm_path(canonicalize_path(path).unwrap());
            include_files.push(path);
        }
    }

    include_files.sort();
    include_files
}

fn create_glob_matcher_from_patterns(
    root_path: impl AsRef<Path>,
    patterns: &[impl AsRef<str>],
    include_globs: &mut Vec<GlobMatcher>,
    exclude_globs: &mut Vec<GlobMatcher>,
) {
    let root_path = root_path.as_ref();

    // build include_globs and exclude_globs from patterns
    for pattern in patterns {
        let pattern = pattern.as_ref();
        if pattern.starts_with("!") {
            let pattern = root_path.join(&pattern[1..]).display().to_string();
            let glob = GlobBuilder::new(&pattern)
                .literal_separator(true)
                .build()
                .unwrap()
                .compile_matcher();
            exclude_globs.push(glob);
        } else {
            let pattern = root_path.join(&pattern).display().to_string();
            let glob = GlobBuilder::new(&pattern)
                .literal_separator(true)
                .build()
                .unwrap()
                .compile_matcher();
            include_globs.push(glob);
        }
    }
}

fn create_git_glob_macther() -> GlobMatcher {
    GlobBuilder::new("**/.git")
        .literal_separator(true)
        .build()
        .unwrap()
        .compile_matcher()
}

#[cfg(test)]
mod tests {

    use super::{scan_files, scan_files_block_by_pkg};
    use std::{fs, path::Path};

    #[test]
    // tests/pkg_assets 目录下测试匹配 pkg patterns 的所有文件
    fn pkg_scan_files_ignore_pkg_test() {
        let root_path = Path::new("../target/tmp/pkg_assets");
        if root_path.is_dir() {
            fs::remove_dir_all(root_path).unwrap();
        }

        // foo1
        let foo1_path = root_path.join("foo1");
        let pkg_content = r#"patterns = ["*.asset"]"#;
        fs::create_dir_all(&foo1_path.join("bar")).unwrap();
        fs::write(&foo1_path.join(".pkg"), pkg_content).unwrap();
        for i in 0..2 {
            for j in 0..6 {
                let name = match i {
                    0 => match j {
                        0 | 1 => format!("{}.asset", j),
                        2 | 3 => format!("{}.txt", j),
                        _ => format!("{}.toml", j),
                    },
                    _ => match j {
                        0 | 1 => format!("bar/{}.asset", j),
                        2 | 3 => format!("bar/{}.txt", j),
                        _ => format!("bar/{}.toml", j),
                    },
                };

                let file = foo1_path.join(&name);
                fs::File::create(&file).expect(&file.display().to_string());
            }
        }

        let patterns = ["*.asset"];
        let files = scan_files_block_by_pkg(foo1_path, &patterns);
        let expect_files = [
            "../target/tmp/pkg_assets/foo1/0.asset",
            "../target/tmp/pkg_assets/foo1/1.asset",
        ];
        let expect_files: Vec<String> = expect_files.iter().map(|s| s.to_string()).collect();
        assert_eq!(files, expect_files);

        // foo2
        let foo2_path = root_path.join("foo2");
        let pkg_content = r#"patterns = ["*.txt", "!bar/*2.txt"]"#;
        fs::create_dir_all(&foo2_path.join("bar")).unwrap();
        fs::write(&foo2_path.join(".pkg"), pkg_content).unwrap();
        for i in 0..2 {
            for j in 0..6 {
                let name = match i {
                    0 => match j {
                        0 | 1 => format!("{}.asset", j),
                        2 | 3 => format!("{}.txt", j),
                        _ => format!("{}.toml", j),
                    },
                    _ => match j {
                        0 | 1 => format!("bar/{}.asset", j),
                        2 | 3 => format!("bar/{}.txt", j),
                        _ => format!("bar/{}.toml", j),
                    },
                };

                let file = foo2_path.join(&name);
                fs::File::create(&file).expect(&file.display().to_string());
            }
        }

        let patterns = ["bar/*.txt", "!bar/*3.txt"];
        let files = scan_files_block_by_pkg(foo2_path, &patterns);
        let expect_files = ["../target/tmp/pkg_assets/foo2/bar/2.txt"];
        let expect_files: Vec<String> = expect_files.iter().map(|s| s.to_string()).collect();
        assert_eq!(files, expect_files);

        // foo3
        let foo3_path = root_path.join("foo3");
        let pkg_content = r#"patterns = ["*.txt","**/*.txt"]"#;
        fs::create_dir_all(&foo3_path.join("bar1")).unwrap();
        fs::create_dir_all(&foo3_path.join("bar2")).unwrap();
        fs::write(&foo3_path.join(".pkg"), pkg_content).unwrap();
        fs::write(&foo3_path.join("bar1/.pkg"), pkg_content).unwrap();
        for i in 0..3 {
            for j in 0..6 {
                let name = match i {
                    0 => match j {
                        0 | 1 => format!("{}.asset", j),
                        2 | 3 => format!("{}.txt", j),
                        _ => format!("{}.toml", j),
                    },
                    1 => match j {
                        0 | 1 => format!("bar1/{}.asset", j),
                        2 | 3 => format!("bar1/{}.txt", j),
                        _ => format!("bar1/{}.toml", j),
                    },
                    _ => match j {
                        0 | 1 => format!("bar2/{}.asset", j),
                        2 | 3 => format!("bar2/{}.txt", j),
                        _ => format!("bar2/{}.toml", j),
                    },
                };

                let file = foo3_path.join(&name);
                fs::File::create(&file).expect(&file.display().to_string());
            }
        }

        let patterns = ["*.txt", "**/*.txt"];
        let files = scan_files_block_by_pkg(foo3_path, &patterns);
        let expect_files = [
            "../target/tmp/pkg_assets/foo3/2.txt",
            "../target/tmp/pkg_assets/foo3/3.txt",
            "../target/tmp/pkg_assets/foo3/bar2/2.txt",
            "../target/tmp/pkg_assets/foo3/bar2/3.txt",
        ];
        let expect_files: Vec<String> = expect_files.iter().map(|s| s.to_string()).collect();
        assert_eq!(files, expect_files);

        // NOTE: 注释本行代码，可以运行 charp 示例代码
        fs::remove_dir_all(root_path).unwrap();
    }

    #[allow(dead_code)]
    fn print_files(files: &Vec<String>) {
        for file in files {
            println!("\t{}", file);
        }
        println!();
    }

    #[test]
    fn pkg_scan_pkg_files_test() {
        let root_path = Path::new("../tests/pkg-dependencies/BuildAssets");
        let patterns = ["**/.pkg", "!**/DepMaterial/**/.pkg"];
        let files = scan_files(root_path, &patterns);
        let expect_files = [
            "../tests/pkg-dependencies/BuildAssets/Material/.pkg",
            "../tests/pkg-dependencies/BuildAssets/Material/SubMaterial/.pkg",
            "../tests/pkg-dependencies/BuildAssets/PKGTest/.pkg",
            "../tests/pkg-dependencies/BuildAssets/Prefab/.pkg",
            "../tests/pkg-dependencies/BuildAssets/Shader/.pkg",
        ];
        let expect_files: Vec<String> = expect_files.iter().map(|s| s.to_string()).collect();
        assert_eq!(files, expect_files);
    }
}