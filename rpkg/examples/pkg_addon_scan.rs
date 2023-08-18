use funny_utils_rs::path::norm_path;
use rpkg::scan_files_block_by_manifest;
use rpkg::BuildMap;
use std::path::Path;

fn main() {
    let root_path = "./tests/pkg-dependencies/";
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
        let addon_pkgs = scan_files_block_by_manifest(&addon_path, &patterns);
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
    println!("{}", build_map.to_string());

    // 省略遍历 member 后，遍历其所有 pkg 文件，再遍历 pkg 里所有 target 的操作
    // 此处展示单个 target 的资源查询

    // bundle
    // 根据 bundle_path 查询与之关联的所有 target
    let addon_path = "./tests/pkg-dependencies/BuildAssets/addon1";

    // 获取 addon pkgs
    let addon_pkg_paths = build_map
        .get_addon_pkg_paths("./tests/pkg-dependencies/BuildAssets/addon1")
        .unwrap();
    println!("addon1 addon_pkg_paths:");
    for pkg_path in addon_pkg_paths {
        println!("  {}", pkg_path);
    }

    // 判断目录是否是 addon
    let is_addon = build_map.is_addon(addon_path);

    if is_addon {
        let inner_addon_paths = build_map.get_inner_addon_paths(addon_path);
        println!("inner addon path:");
        for item in &inner_addon_paths {
            println!("  {}", item);
        }
    }

    // 判断目录下层是否关联 addon（含本身）
    let has_inner_addon = build_map.has_inner_addon(addon_path);

    // 获取其关联的 addon_path
    if has_inner_addon {
        let inner_addon_paths = build_map.get_inner_addon_paths(addon_path);
        println!("inner addon path:");
        for item in &inner_addon_paths {
            println!("  {}", item);
        }
    }

    // 判断上层目录是否关联 addon
    let has_outer_addon = build_map.has_outer_addon(addon_path);

    // 获取最近上层的 addon_path
    if has_outer_addon {
        let outer_addon_path = build_map.get_outer_addon_path(addon_path);
        println!("outer addon path:");
        println!("  {}", outer_addon_path);
    }

    // 收集指定目录 addon 与 pkg 的信息
    let select_paths = [
        "./tests/pkg-dependencies",
        "./tests/pkg-dependencies/BuildAssets",
        "./tests/pkg-dependencies/BuildAssets/addon1",
        "./tests/pkg-dependencies/BuildAssets/addon2",
        "./tests/pkg-dependencies/BuildAssets/manifest.toml",
        "./tests/pkg-dependencies/BuildAssets/addon1/manifest.toml",
        "./tests/pkg-dependencies/BuildAssets/addon1/Prefab/.pkg",
        "./tests/pkg-dependencies/CircularDep",
        "./tests/pkg-dependencies/CircularDep/A/.pkg",
    ];

    for select_path in select_paths {
        let build_collection = build_map.seek_build_collection(select_path).unwrap();

        println!("select_path: {}", select_path);
        println!("  seek addon paths");
        for addon_path in build_collection.get_addon_paths() {
            println!("      {}", addon_path);
        }

        println!("  seek pkg paths");
        for pkg_path in build_collection.get_pkg_paths() {
            println!("      {}", pkg_path);
        }
    }

    let target_path = "BuildAssets/addon1/Prefab";

    // 获取所有 target_types
    let target_types = build_map.get_target_types(addon_path).unwrap();

    // 遍历 target_types
    for target_type in target_types {
        // 获取依赖项
        let deps = match build_map.resolve_target_deps(target_path, target_type) {
            Err(e) => panic!("{}, {}", e.root_cause().to_string(), e.to_string()),
            Ok(r) => r,
        };

        // 可判断是否依赖循环
        assert_eq!(deps.is_circular, false);

        // 获取要 build 的 target_path
        let mut to_build = deps.target_paths.clone();
        // 加入自身
        to_build.push(target_path.to_string());

        // 根据 target 获取与之关联的所有资源路径
        println!("to_build:");
        for target_path in &to_build {
            println!("  {} assets:", target_path);
            for asset in build_map.get_target_assets(target_path, target_type) {
                println!("    {}", asset);
            }
        }
    }

    // 获取 asset_urls
    println!("asset_urls:");
    for url in build_map.get_asset_urls(addon_path).unwrap() {
        println!("  {}", url);
    }
    println!();

    // file 类型
    // 根据 file 查询与之关联的所有 target
    let addon_path = "./tests/pkg-dependencies/BuildAssets/addon2";
    let target_path = "BuildAssets/addon2";
    // 获取所有 target_types
    let target_types = build_map.get_target_types(addon_path).unwrap();

    // 遍历 target_types
    for target_type in target_types {
        let deps = match build_map.resolve_target_deps(target_path, target_type) {
            Err(e) => panic!("{}", e.to_string()),
            Ok(r) => r,
        };

        assert_eq!(deps.is_circular, false);

        // 获取依赖项
        let mut to_build = deps.target_paths.clone();

        // 加入自身
        to_build.push(target_path.to_string());

        // 获取与之关联的所有资源路径
        println!("to_build:");
        for target_path in &to_build {
            println!("  {} assets:", target_path);
            for asset in build_map.get_target_assets(target_path, target_type) {
                println!("    {}", asset);
            }
        }
    }

    // 获取 asset_urls
    println!("asset_urls:");
    for url in build_map.get_asset_urls(addon_path).unwrap() {
        println!("  {}", url);
    }
    println!();
}
