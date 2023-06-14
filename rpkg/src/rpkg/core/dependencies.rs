use globset::GlobBuilder;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use super::pkg;

#[derive(Default, Debug)]
pub struct Dependencies {
    pub files: Vec<String>,
    pub invalid_files: Vec<String>,
    pub is_circular: bool,
}

pub fn seek_dependencies(
    root_path: impl AsRef<Path>,
    cur_pkg: impl AsRef<Path>,
    patterns: &[impl AsRef<str>],
) -> Dependencies {
    let cur_pkg = cur_pkg.as_ref();
    let mut deps = Dependencies::default();

    if pkg::parse(cur_pkg).is_none() {
        deps.invalid_files.push(cur_pkg.display().to_string());
        return deps;
    };

    deps.files.push(cur_pkg.display().to_string());

    let mut indegree_map = HashMap::new();
    let mut dep_map = HashMap::new();

    let mut rel_patterns = Vec::new();
    let mut abs_patterns = Vec::new();
    split_patterns(&patterns, &mut rel_patterns, &mut abs_patterns);

    let mut dep_pkgs = Vec::new();
    dep_pkgs.append(&mut glob_pkgs(cur_pkg.parent().unwrap(), &rel_patterns));
    dep_pkgs.append(&mut glob_pkgs(&root_path, &abs_patterns));

    indegree_map.insert(cur_pkg.to_path_buf(), 0);
    dep_map.insert(cur_pkg.to_path_buf(), dep_pkgs.clone());

    // calculate indegree and seek dependencies
    let mut queue = dep_pkgs;
    while !queue.is_empty() {
        let path = queue.pop().unwrap();
        let file = PathBuf::from(&path);

        let Some(patterns) = pkg::get_dep_patterns_from_file(&file) else {
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

        let mut dep_pkgs = Vec::new();
        dep_pkgs.append(&mut glob_pkgs(file.parent().unwrap(), &rel_patterns));
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

/// split patterns into rel_patterns and abs_patterns.
///   - rel_patterns base on '.pkg' file's parent directory;
///   - abs_patterns base on specify root path.
fn split_patterns(
    patterns: &[impl AsRef<str>],
    rel_patterns: &mut Vec<String>,
    abs_patterns: &mut Vec<String>,
) {
    for pattern in patterns {
        let pattern = pattern.as_ref();
        // NOTE: not support ignore pattern like "!**.pkg" and skip it
        if pattern.starts_with("!") {
            continue;
        }

        match pattern.starts_with("/") {
            true => abs_patterns.push(pattern[1..].to_owned()),
            false => rel_patterns.push(pattern.to_owned()),
        }
    }
}

fn glob_pkgs(base_path: impl AsRef<Path>, patterns: &[impl AsRef<str>]) -> Vec<String> {
    if patterns.as_ref().is_empty() {
        return Vec::new();
    }

    let base_path = base_path.as_ref();
    let mut include_globs = Vec::new();
    let git_glob = GlobBuilder::new("**/.git")
        .literal_separator(true)
        .build()
        .unwrap()
        .compile_matcher();

    // build include_globs from patterns
    for pattern in patterns {
        let pattern = base_path.join(pattern.as_ref()).display().to_string();

        let glob = GlobBuilder::new(pattern.as_ref())
            .literal_separator(true)
            .build()
            .unwrap()
            .compile_matcher();

        include_globs.push(glob);
    }

    let mut walk_iter = WalkDir::new(base_path).into_iter();
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
        if path.is_file() && (path == base_path.join(".pkg") || !path.ends_with(".pkg")) {
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
        let cur_pkg = "../tests/pkg-dependencies/BuildAssets/Prefab/.pkg";

        // global dependencies
        let pattterns = [
            "/pkg-dependencies/BuildAssets/Material/.pkg",
            "/pkg-dependencies/BuildAssets/Material/DepMaterial/.pkg",
        ];
        let mut deps = seek_dependencies(root_path, cur_pkg, &pattterns);
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
        let pattterns = ["**/.pkg"];
        let cur_pkg = "../tests/pkg-dependencies/BuildAssets/rel.pkg";
        let mut deps = seek_dependencies(root_path, cur_pkg, &pattterns);
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
        let pattterns = ["**/PKGTest/.pkg"];
        let cur_pkg = "../tests/pkg-dependencies/BuildAssets/rel2.pkg";
        let mut deps = seek_dependencies(root_path, cur_pkg, &pattterns);
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
        let pattterns = ["/pkg-dependencies/CircularDep/B/.pkg"];
        let cur_pkg = "../tests/pkg-dependencies/CircularDep/A/.pkg";
        let deps = seek_dependencies(root_path, cur_pkg, &pattterns);
        assert_eq!(true, deps.is_circular);
    }
}
