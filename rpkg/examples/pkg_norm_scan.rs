use rpkg::scan_files;
use rpkg::BuildMap;

fn main() {
    let root_path = "./tests/pkg-dependencies/";
    let asset_path = "./tests/pkg-dependencies/BuildAssets";
    let patterns = ["**/.pkg"];

    // 扫描所有 pkg 文件
    let pkgs = scan_files(asset_path, &patterns);
    println!("total pkgs:");
    for item in &pkgs {
        println!("  {}", item);
    }
    println!();

    // 创建 BuildMap
    let mut build_map = match BuildMap::new(root_path) {
        Ok(v) => v,
        Err(e) => panic!("{}", e.to_string()),
    };

    // 插入 pkgs 文件并解析
    let addon_path = asset_path;
    if let Err(e) = build_map.insert(addon_path, pkgs) {
        panic!("  {}", e.to_string());
    }
    println!("build map:");
    println!("{}\n", build_map.to_string());

    // 获取所有 target_types
    let target_types = build_map.get_target_types(addon_path).unwrap();

    // 遍历 target_types
    for target_type in target_types {
        // 遍历获取所有 bundles、subscene、file、dylib、zip 的 target_path
        println!("{} and scan assets:", target_type);
        for target_path in build_map.get_target_paths(addon_path, target_type).unwrap() {
            // 获取所有 bundles、subscene、file、dylib、zip 对应的资源
            println!("  {} assets:", target_path);
            for asset in build_map.get_target_assets(target_path, target_type) {
                println!("    {}", asset);
            }
        }
    }
}
