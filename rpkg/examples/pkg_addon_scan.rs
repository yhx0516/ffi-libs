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

    // 获取 members
    let members = ["./", "addon1", "addon2"];

    // 遍历 members
    println!("addons and pkgs:");
    for member in members {
        // 搜索 member 下的 pkg 文件
        let addon_path = Path::new(asset_path).join(member);
        let addon_pkgs = scan_files_block_manifest(&addon_path, &patterns);
        println!("  addon \"{}\" pkgs:", member);
        for item in &addon_pkgs {
            println!("    {}", item);
        }

        // 插入 pkg 文件并解析
        if let Err(e) = build_map.insert(norm_path(&addon_path), addon_pkgs) {
            panic!("{}", e.to_string());
        }
    }
    println!();
    println!("build map:");
    println!("{}\n", build_map.to_string());

    // 省略遍历 member 后，遍历其所有 pkg 文件，再遍历 pkg 里所有 target 的操作
    // 此处展示单个 target 的资源查询

    // bundle
    // 根据 bundle_path 查询与之关联的所有 target
    let mount_path = "./tests/pkg-dependencies/BuildAssets/addon1";
    let target_path = "BuildAssets/addon1/Prefab";
    let to_build = match build_map.resolve_bundle_deps(target_path) {
        Err(e) => panic!("{}", e.to_string()),
        Ok(r) => r,
    };

    // 可判断是否依赖循环
    assert_eq!(to_build.is_circular, false);

    // 根据 target 获取与之关联的所有资源路径
    println!("to_build:");
    for target in &to_build.build_targets {
        println!("  {} assets:", target);
        let assets = match build_map.scan_bundle_assets(mount_path, target) {
            Ok(r) => r,
            Err(e) => panic!("{}", e.to_string()),
        };
        println!("{}", assets);
    }

    // file 类型
    // 根据 file 查询与之关联的所有 target
    let mount_path = "./tests/pkg-dependencies/BuildAssets/addon2";
    let target_path = "BuildAssets/addon2";
    let to_build = match build_map.resolve_file_deps(target_path) {
        Err(e) => panic!("{}", e.to_string()),
        Ok(r) => r,
    };

    assert_eq!(to_build.is_circular, false);

    // 获取与之关联的所有资源路径
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
