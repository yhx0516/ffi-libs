use std::path::Path;

use globset::{Glob, GlobSetBuilder};
use ignore::{overrides::OverrideBuilder, WalkBuilder};

pub fn pkg_match(root_path: impl AsRef<Path>, patterns: &[impl AsRef<str>]) -> Vec<String> {
    let mut glob_set_builder = GlobSetBuilder::new();
    let mut override_builder = OverrideBuilder::new(root_path.as_ref());

    // analyse pattern into glob set builder and overrider builder
    for pattern in patterns {
        let pattern = pattern.as_ref();
        if pattern.starts_with("!") {
            override_builder.add(pattern).unwrap();
        } else {
            glob_set_builder.add(Glob::new(pattern).unwrap());
        }
    }
    glob_set_builder.add(Glob::new("**.pkg").unwrap());

    // create work_builder
    let glob_set = glob_set_builder.build().unwrap();
    let mut work_builder = WalkBuilder::new(root_path.as_ref());

    if let Ok(overrides) = override_builder.build() {
        work_builder.overrides(overrides);
    }

    let mut pkg_files = Vec::new();
    let mut asset_files = Vec::new();

    // get files
    for result in work_builder.build() {
        let Ok(entry) = result else{
           eprintln!("{:?}",result.err());
            continue;
        };

        let file_path = entry.path();

        if !glob_set.is_match(file_path) {
            continue;
        }

        if entry.path().ends_with(".pkg") {
            pkg_files.push(entry.path().to_owned());
        } else {
            asset_files.push(entry.path().to_owned())
        }
    }

    // exclude files which path start with other "../.pkg"
    // for pkg_file in pkg_files {
    //     println!("{:?}", pkg_file);
    //     let Some(dir)=pkg_file.parent() else{
    //         eprintln!("{:?} has not parent.",&pkg_file);
    //         continue;
    //     };

    //     let remained_files = asset_files
    //         .iter()
    //         .filter_map(|p| match p.starts_with(dir) {
    //             true => Some(p.to_owned()),
    //             false => None,
    //         })
    //         .collect();
    //     asset_files = remained_files;
    // }

    let mut res: Vec<String> = asset_files
        .iter()
        .map(|p| {
            let path = p.display().to_string();
            path.replace("\\", "/")
        })
        .collect();

    res.sort();

    res
}

#[cfg(test)]
mod tests {
    use super::pkg_match;
    use std::{fs, path::Path};

    #[test]
    // tests/pkg_assets 目录下测试匹配 pkg patterns 的所有文件
    fn pkg_match_files_test() {
        let root_path = Path::new(r"../tests/pkg_assets");
        if root_path.is_dir() {
            fs::remove_dir_all(root_path).unwrap();
        }

        // foo1
        let foo1_path = root_path.join("foo1");
        let pkg_content = r#"patterns = ["**/foo1/*.asset"]"#;
        fs::create_dir_all(&foo1_path).unwrap();
        fs::write(&foo1_path.join(".pkg"), pkg_content).unwrap();
        for i in 0..10 {
            let name = match i % 3 {
                0 => format!("{}.asset", i),
                1 => format!("{}.txt", i),
                _ => format!("{}.toml", i),
            };
            let file = foo1_path.join(&name);

            fs::File::create(&file).unwrap();
        }

        let patterns = ["**/foo1/*.asset"];
        let files = pkg_match(root_path, &patterns);
        let expect_files = [
            "../tests/pkg_assets/foo1/0.asset",
            "../tests/pkg_assets/foo1/3.asset",
            "../tests/pkg_assets/foo1/6.asset",
            "../tests/pkg_assets/foo1/9.asset",
        ];
        let expect_files: Vec<String> = expect_files.iter().map(|s| s.to_string()).collect();
        assert_eq!(files, expect_files);

        // foo2
        let foo2_path = root_path.join("foo2");
        let pkg_content = r#"patterns = ["**/foo2/*.txt", "!**/foo2/bar/*2.txt"]"#;
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

        let patterns = ["**/foo2/*.txt", "!**/foo2/bar/*2.txt"];
        let files = pkg_match(root_path, &patterns);
        let expect_files = [
            "../tests/pkg_assets/foo2/2.txt",
            "../tests/pkg_assets/foo2/3.txt",
            "../tests/pkg_assets/foo2/bar/3.txt",
        ];
        let expect_files: Vec<String> = expect_files.iter().map(|s| s.to_string()).collect();
        assert_eq!(files, expect_files);

        // foo3
        let foo3_path = root_path.join("foo3");
        let pkg_content = r#"patterns = ["**/foo3/*.txt","**/foo3/**/*.txt"]"#;
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

        let patterns = ["**/foo3/*.txt", "**/foo3/**/*.txt"];
        let files = pkg_match(root_path, &patterns);
        let expect_files = [
            "../tests/pkg_assets/foo3/2.txt",
            "../tests/pkg_assets/foo3/3.txt",
            "../tests/pkg_assets/foo3/bar1/2.txt",
            "../tests/pkg_assets/foo3/bar1/3.txt",
            "../tests/pkg_assets/foo3/bar2/2.txt",
            "../tests/pkg_assets/foo3/bar2/3.txt",
        ];
        let expect_files: Vec<String> = expect_files.iter().map(|s| s.to_string()).collect();
        assert_eq!(files, expect_files);

        // fs::remove_dir_all(root_path).unwrap();
    }

    #[allow(dead_code)]
    fn print_files(files: &Vec<String>) {
        for file in files {
            println!("\t{}", file);
        }
        println!();
    }
}
