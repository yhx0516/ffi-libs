use std::path::Path;
use rpkg::scan_files_block_manifest;
use rpkg::BuildMap;
use rutils::norm_path;

fn main() {
    let root_path = r"./tests/pkg-dependencies/";
    let asset_path = "./tests/pkg-dependencies/BuildAssets";
    let patterns = ["**/.pkg"];

    // 创建 BuildMap
    let mut build_map = match BuildMap::new(root_path) {
        Ok(v) => v,
        Err(e) => panic!("{}", e.to_string()),
    };

    let members = ["./", "addon1", "addon2"];
    println!("addons and pkgs:");
    for member in members {
        let addon_path = Path::new(asset_path).join(member);
        let addon_pkgs = scan_files_block_manifest(&addon_path, &patterns);
        println!("  addon \"{}\" pkgs:", member);
        for item in &addon_pkgs {
            println!("    {}", item);
        }

        if let Err(e) = build_map.insert(norm_path(&addon_path), addon_pkgs) {
            panic!("{}", e.to_string());
        }
    }
    println!();
    println!("build map:");
    println!("{}\n", build_map.to_string());

    // bundle
    let mount_path = "./tests/pkg-dependencies/BuildAssets/addon1";
    let target_path = "BuildAssets/addon1/Prefab";
    let to_build = match build_map.resolve_bundle_deps(target_path) {
        Err(e) => panic!("{}", e.to_string()),
        Ok(r) => r,
    };

    assert_eq!(to_build.is_circular, false);

    println!("to_build:");
    for target in &to_build.build_targets {
        println!("  {} assets:", target);
        let assets = match build_map.scan_bundle_assets(mount_path, target) {
            Ok(r) => r,
            Err(e) => panic!("{}", e.to_string()),
        };
        println!("{}", assets);
    }

    // file
    let mount_path = "./tests/pkg-dependencies/BuildAssets/addon2";
    let target_path = "BuildAssets/addon2";
    let to_build = match build_map.resolve_file_deps(target_path) {
        Err(e) => panic!("{}", e.to_string()),
        Ok(r) => r,
    };

    assert_eq!(to_build.is_circular, false);

    println!("to_build:");
    for target in &to_build.build_targets {
        println!("  {} assets:", target);
        let assets = match build_map.scan_file_assets(mount_path, target) {
            Ok(r) => r,
            Err(e) => panic!("{}", e.to_string()),
        };
        println!("{}", assets);
    }
}
