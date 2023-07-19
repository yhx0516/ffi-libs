use rpkg::scan_files_block_by_manifest;
use rpkg::BuildMap;
use rutils::path::norm_path;
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
    let target_path = "BuildAssets/addon1/Prefab";
    let deps = match build_map.resolve_bundle_deps(target_path) {
        Err(e) => panic!("{}", e.to_string()),
        Ok(r) => r,
    };

    // 可判断是否依赖循环
    assert_eq!(deps.is_circular, false);

    // 获取依赖项
    let mut to_build = deps.target_paths.clone();

    // 加入自身
    to_build.push(target_path.to_string());

    // 根据 target 获取与之关联的所有资源路径
    println!("to_build:");
    for target_path in &to_build {
        println!("  {} assets:", target_path);
        for asset in build_map.get_bundle_assets(target_path) {
            println!("    {}", asset);
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
    let deps = match build_map.resolve_file_deps(target_path) {
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
        for asset in build_map.get_file_assets(target_path) {
            println!("    {}", asset);
        }
    }

    // 获取 asset_urls
    println!("asset_urls:");
    for url in build_map.get_asset_urls(addon_path).unwrap() {
        println!("  {}", url);
    }
    println!();
}