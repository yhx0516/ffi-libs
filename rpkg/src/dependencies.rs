use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use globset::GlobBuilder;
use walkdir::WalkDir;

use crate::pkg::load_toml_pkg;

#[derive(Default, Debug)]
pub struct Dependencies {
    pub files: Vec<String>,
    pub invalid_files: Vec<String>,
    pub is_circular: bool,
}

pub fn seek_dependencies(root_path: impl AsRef<Path>, file: impl AsRef<Path>) -> Dependencies {
    let file = file.as_ref();
    let mut deps = Dependencies::default();
    let Some(patterns) = get_dep_patterns(file) else {
        deps.invalid_files.push(file.display().to_string());
        return deps;
    };

    deps.files.push(file.display().to_string());

    let mut indegree_map = HashMap::new();
    let mut dep_map = HashMap::new();

    let mut rel_patterns = Vec::new();
    let mut abs_patterns = Vec::new();
    split_patterns(&patterns, &mut rel_patterns, &mut abs_patterns);

    let mut dep_pkgs = glob_pkgs(file.parent().unwrap(), &rel_patterns);
    dep_pkgs.append(&mut glob_pkgs(&root_path, &abs_patterns));

    indegree_map.insert(file.to_path_buf(), 0);
    dep_map.insert(file.to_path_buf(), dep_pkgs.clone());

    // calculate indegree and seek dependencies
    let mut queue = dep_pkgs;
    while !queue.is_empty() {
        let path = queue.pop().unwrap();
        let file = PathBuf::from(&path);

        let Some(patterns) = get_dep_patterns(&file) else {
            deps.invalid_files.push(path);
            continue;
        };

        if let Some(val) = indegree_map.get_mut(&file) {
            *val += 1;
            continue;
        }

        let mut rel_patterns = Vec::new();
        let mut abs_patterns = Vec::new();
        split_patterns(&patterns, &mut rel_patterns, &mut abs_patterns);

        let mut dep_pkgs = glob_pkgs(file.parent().unwrap(), &rel_patterns);
        dep_pkgs.append(&mut glob_pkgs(&root_path, &abs_patterns));

        indegree_map.insert(file.clone(), 1);
        dep_map.insert(file.clone(), dep_pkgs.clone());
        deps.files.push(file.display().to_string());
        queue.extend(dep_pkgs);
    }

    let mut queue: Vec<_> = indegree_map
        .iter()
        .filter_map(|(k, v)| match *v == 0 {
            true => Some(k.to_owned()),
            false => None,
        })
        .collect();

    // check if dependencies is circular
    let mut zero_indegree = 0;
    while !queue.is_empty() {
        zero_indegree += 1;
        let path = queue.pop().unwrap();
        for file in &dep_map[&path] {
            let file = PathBuf::from(file);
            if let Some(val) = indegree_map.get_mut(&file) {
                *val -= 1;
                if *val == 0 {
                    queue.push(file);
                }
            }
        }
    }

    deps.is_circular = zero_indegree != dep_map.len();
    deps
}

fn get_dep_patterns(file: impl AsRef<Path>) -> Option<Vec<String>> {
    let Some(pkg) = load_toml_pkg(file) else {
        return  None;
    };

    match pkg.dependencies {
        Some(r) => Some(r),
        None => Some(Vec::new()),
    }
}

/// split patterns into rel_patterns and abs_patterns.
///   - rel_patterns base on '.pkg' file's parent directory;
///   - abs_patterns base on specify root path.
fn split_patterns(
    patterns: &Vec<String>,
    rel_patterns: &mut Vec<String>,
    abs_patterns: &mut Vec<String>,
) {
    for pattern in patterns {
        // NOTE: not support ignore pattern like "!**.pkg" and skip it
        if pattern.starts_with("!") {
            continue;
        }

        match pattern.starts_with("/") {
            true => abs_patterns.push(pattern.to_owned()),
            false => rel_patterns.push(pattern.to_owned()),
        }
    }
}

fn glob_pkgs(root_path: impl AsRef<Path>, patterns: &[impl AsRef<str>]) -> Vec<String> {
    let root_path = root_path.as_ref();
    let mut include_globs = Vec::new();
    let git_glob = GlobBuilder::new("**/.git")
        .literal_separator(true)
        .build()
        .unwrap()
        .compile_matcher();

    // build include_globs from patterns
    for pattern in patterns {
        let pattern = pattern.as_ref();

        let pattern = match pattern.starts_with("/") {
            true => root_path.join(&pattern[1..]).display().to_string(),
            false => pattern.to_string(),
        };

        let glob = GlobBuilder::new(&pattern)
            .literal_separator(true)
            .build()
            .unwrap()
            .compile_matcher();

        include_globs.push(glob);
    }

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

        // skip .git directory
        if git_glob.is_match(path) {
            walk_iter.skip_current_dir();
            continue;
        }

        // skip invalid pkg file
        if path.is_file() && (path == root_path.join(".pkg") || !path.ends_with(".pkg")) {
            continue;
        }

        // include file
        if include_globs.iter().any(|m| m.is_match(path)) {
            let path = path.display().to_string().replace("\\", "/");
            include_files.push(path);
        }
    }

    // sort files
    include_files.sort();

    include_files
}

mod tests {
    #[allow(unused_imports)]
    use super::seek_dependencies;

    #[test]
    fn circular_dependencies_test() {
        let root_path = "../tests";
        // global dependencies
        let path = "../tests/pkg-dependencies/BuildAssets/Prefab/.pkg";
        let mut deps = seek_dependencies(root_path, path);
        assert_eq!(false, deps.is_circular);

        deps.files.sort();
        let expect_deps = vec![
            "../tests/pkg-dependencies/BuildAssets/Material/.pkg",
            "../tests/pkg-dependencies/BuildAssets/Material/DepMaterial/.pkg",
            "../tests/pkg-dependencies/BuildAssets/Prefab/.pkg",
            "../tests/pkg-dependencies/BuildAssets/Shader/.pkg",
        ];
        assert_eq!(expect_deps, deps.files);

        // relative dependencies
        let path = "../tests/pkg-dependencies/BuildAssets/rel.pkg";
        let mut deps = seek_dependencies(root_path, path);
        assert_eq!(false, deps.is_circular);
        deps.files.sort();
        let expect_deps = vec![
            "../tests/pkg-dependencies/BuildAssets/Material/.pkg",
            "../tests/pkg-dependencies/BuildAssets/Material/DepMaterial/.pkg",
            "../tests/pkg-dependencies/BuildAssets/Material/SubMaterial/.pkg",
            "../tests/pkg-dependencies/BuildAssets/PKGTest/.pkg",
            "../tests/pkg-dependencies/BuildAssets/Prefab/.pkg",
            "../tests/pkg-dependencies/BuildAssets/Shader/.pkg",
            "../tests/pkg-dependencies/BuildAssets/rel.pkg",
        ];
        assert_eq!(expect_deps, deps.files);

        // relative dependencies
        let path = "../tests/pkg-dependencies/BuildAssets/rel2.pkg";
        let mut deps = seek_dependencies(root_path, path);
        assert_eq!(false, deps.is_circular);
        deps.files.sort();
        let expect_deps = vec![
            "../tests/pkg-dependencies/BuildAssets/Material/.pkg",
            "../tests/pkg-dependencies/BuildAssets/Material/DepMaterial/.pkg",
            "../tests/pkg-dependencies/BuildAssets/PKGTest/.pkg",
            "../tests/pkg-dependencies/BuildAssets/Shader/.pkg",
            "../tests/pkg-dependencies/BuildAssets/rel2.pkg",
        ];
        assert_eq!(expect_deps, deps.files);

        // circular dependencies
        let path = "../tests/pkg-dependencies/CircularDep/A/.pkg";
        let deps = seek_dependencies(root_path, path);
        assert_eq!(true, deps.is_circular);
    }
}
