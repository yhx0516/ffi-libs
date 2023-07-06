use anyhow::anyhow;
use anyhow::Result;
use std::collections::{BTreeMap, HashMap};

use crate::toml::build_target::resolve_dep_path;
use crate::BuildTarget;

#[derive(Default, Debug)]
pub struct Dependencies {
    pub build_targets: Vec<String>,
    pub is_circular: bool,
}

pub fn resolve_build_deps(
    root_path: impl AsRef<str>,
    target_path: impl AsRef<str>,
    target_map: &BTreeMap<String, Box<dyn BuildTarget>>,
) -> Result<Dependencies> {
    let target_path = target_path.as_ref();
    let root_path = root_path.as_ref();

    let Some(target) = target_map.get(target_path) else {
        return Err(anyhow!("not found target: {}",target_path));
    };

    let mut res = Dependencies::default();
    res.build_targets.push(target_path.to_string());

    let deps = resolve_dep_path(root_path, target_path, target.get_deps())?;

    let mut indegree_map = HashMap::new();
    let mut dep_map = HashMap::new();
    indegree_map.insert(target_path.to_string(), 0);
    dep_map.insert(target_path.to_string(), deps.clone());

    // calculate indegree and seek dependencies
    let mut queue = deps;
    while !queue.is_empty() {
        let target_path = queue.pop().unwrap();

        if let Some(val) = indegree_map.get_mut(&target_path) {
            *val += 1;
            continue;
        }

        let Some(target) = target_map.get(&target_path) else {
            return Err(anyhow!("not found {} in target map",&target_path));
        };

        let deps = resolve_dep_path(root_path, &target_path, target.get_deps())?;

        indegree_map.insert(target_path.clone(), 1);
        dep_map.insert(target_path.clone(), deps.clone());
        res.build_targets.push(target_path);
        queue.extend(deps);
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
        for dep_path in &dep_map[&path] {
            if let Some(val) = indegree_map.get_mut(dep_path) {
                *val -= 1;
                if *val == 0 {
                    queue.push(dep_path.to_owned());
                }
            }
        }
    }

    res.is_circular = zero_indegree != dep_map.len();

    Ok(res)
}
