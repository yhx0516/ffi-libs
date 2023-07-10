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
    let mount_path = asset_path;
    if let Err(e) = build_map.insert(mount_path, pkgs) {
        panic!("  {}", e.to_string());
    }
    println!("build map:");
    println!("{}\n", build_map.to_string());

    // 获取所有 bundles
    // 同理 subscene、file、dylib、zip
    println!("bundles and scan assets:");
    for target_path in build_map.get_bundle_paths(mount_path).unwrap() {
        println!("  {} assets:", target_path);

        for asset in build_map.get_bundle_assets(target_path) {
            println!("    {}", asset);
        }
    }
}
