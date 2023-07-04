use std::ffi::c_char;

use crate::core::{resolve_build_deps, BuildMap, Dependencies};
use crate::scan_files;
use crate::{pkg, scan_files_block_manifest, scan_files_block_pkg, scan_files_block_pkg_manifest};

// ============================================================
// Info
// ============================================================
#[no_mangle]
pub extern "C" fn get_version() -> *const c_char {
    let version = format!(
        "{} {}",
        std::env!("CARGO_PKG_NAME"),
        std::env!("CARGO_PKG_VERSION")
    );
    rutils::str_to_char_ptr(&version)
}

// ============================================================
// Scan api
// ============================================================

#[no_mangle]
pub extern "C" fn rpkg_scan_files(
    root_path: *const c_char,
    patterns: *const *const c_char,
    patterns_len: usize,
) -> *const Vec<String> {
    let root_path = rutils::char_ptr_to_str(root_path);
    let patterns = rutils::arr_ptr_to_strs(patterns, patterns_len as usize);
    let patterns: Vec<&str> = patterns.iter().map(|s| s.as_ref()).collect();

    let files = scan_files(root_path, &patterns);
    Box::into_raw(Box::new(files))
}

#[no_mangle]
pub extern "C" fn rpkg_scan_files_block_pkg(
    root_path: *const c_char,
    patterns: *const *const c_char,
    patterns_len: usize,
) -> *const Vec<String> {
    let root_path = rutils::char_ptr_to_str(root_path);
    let patterns = rutils::arr_ptr_to_strs(patterns, patterns_len as usize);
    let patterns: Vec<&str> = patterns.iter().map(|s| s.as_ref()).collect();

    let files = scan_files_block_pkg(root_path, &patterns);
    Box::into_raw(Box::new(files))
}

#[no_mangle]
pub extern "C" fn rpkg_scan_files_block_manifest(
    root_path: *const c_char,
    patterns: *const *const c_char,
    patterns_len: usize,
) -> *const Vec<String> {
    let root_path = rutils::char_ptr_to_str(root_path);
    let patterns = rutils::arr_ptr_to_strs(patterns, patterns_len as usize);
    let patterns: Vec<&str> = patterns.iter().map(|s| s.as_ref()).collect();

    let files = scan_files_block_manifest(root_path, &patterns);
    Box::into_raw(Box::new(files))
}

#[no_mangle]
pub extern "C" fn rpkg_scan_files_block_pkg_manifest(
    root_path: *const c_char,
    patterns: *const *const c_char,
    patterns_len: usize,
) -> *const Vec<String> {
    let root_path = rutils::char_ptr_to_str(root_path);
    let patterns = rutils::arr_ptr_to_strs(patterns, patterns_len as usize);
    let patterns: Vec<&str> = patterns.iter().map(|s| s.as_ref()).collect();

    let files = scan_files_block_pkg_manifest(root_path, &patterns);
    Box::into_raw(Box::new(files))
}

#[no_mangle]
pub extern "C" fn rpkg_scan_assets_from_file(
    file: *const c_char,
    root_path: *const c_char,
) -> *const Vec<String> {
    let _file = rutils::char_ptr_to_str(file);
    let _root_path = rutils::char_ptr_to_str(root_path);
    // let files = pkg::scan_assets_from_file(file, root_path);
    // Box::into_raw(Box::new(files))
    todo!()
}

// ============================================================
// BuildMap api
// ============================================================
#[no_mangle]
pub extern "C" fn bm_init(
    root_path: *const c_char,
    pkg_paths: *const *const c_char,
    pkg_paths_len: usize,
) -> *const BuildMap {
    let root_path = rutils::char_ptr_to_str(root_path);
    let pkg_paths = rutils::arr_ptr_to_strs(pkg_paths, pkg_paths_len as usize);
    let pkg_paths: Vec<&str> = pkg_paths.iter().map(|s| s.as_ref()).collect();

    let mut build_map = BuildMap::new();
    if let Err(e) = build_map.init(root_path, pkg_paths) {
        eprintln!("{}", e.to_string());
        return std::ptr::null();
    }
    Box::into_raw(Box::new(build_map))
}

#[no_mangle]
pub extern "C" fn bm_dispose(ptr: *mut BuildMap) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn bm_resolve_bundle_deps(
    ptr: *mut BuildMap,
    target_path: *const c_char,
) -> *const Dependencies {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let target_path = rutils::char_ptr_to_str(target_path);

    match build_map.resolve_bundle_deps(target_path) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn bm_resolve_subscene_deps(
    ptr: *mut BuildMap,
    target_path: *const c_char,
) -> *const Dependencies {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let target_path = rutils::char_ptr_to_str(target_path);

    match build_map.resolve_subscene_deps(target_path) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn bm_resolve_dylib_deps(
    ptr: *mut BuildMap,
    target_path: *const c_char,
) -> *const Dependencies {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let target_path = rutils::char_ptr_to_str(target_path);

    match build_map.resolve_dylib_deps(target_path) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn bm_resolve_file_deps(
    ptr: *mut BuildMap,
    target_path: *const c_char,
) -> *const Dependencies {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let target_path = rutils::char_ptr_to_str(target_path);

    match build_map.resolve_file_deps(target_path) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn bm_resolve_zip_deps(
    ptr: *mut BuildMap,
    target_path: *const c_char,
) -> *const Dependencies {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let target_path = rutils::char_ptr_to_str(target_path);

    match build_map.resolve_zip_deps(target_path) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn bm_scan_bundle_assets(
    ptr: *mut BuildMap,
    target_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let target_path = rutils::char_ptr_to_str(target_path);

    match build_map.scan_bundle_assets(target_path) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn bm_scan_subscene_assets(
    ptr: *mut BuildMap,
    target_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let target_path = rutils::char_ptr_to_str(target_path);

    match build_map.scan_subscene_assets(target_path) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn bm_scan_dylib_assets(
    ptr: *mut BuildMap,
    target_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let target_path = rutils::char_ptr_to_str(target_path);

    match build_map.scan_dylib_assets(target_path) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn bm_file_subscene_assets(
    ptr: *mut BuildMap,
    target_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let target_path = rutils::char_ptr_to_str(target_path);

    match build_map.scan_file_assets(target_path) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn bm_scan_zip_assets(
    ptr: *mut BuildMap,
    target_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let target_path = rutils::char_ptr_to_str(target_path);

    match build_map.scan_zip_assets(target_path) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            eprintln!("{}", e.to_string());
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn bm_debug_info(ptr: *const BuildMap) -> *const c_char {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    rutils::str_to_char_ptr(&build_map.to_string())
}

// ============================================================
// Dependencies api
// ============================================================

#[no_mangle]
pub extern "C" fn dependencies_get_files(ptr: *const Dependencies) -> *const Vec<String> {
    let deps = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    Box::into_raw(Box::new(deps.build_targets.clone()))
}

#[no_mangle]
pub extern "C" fn dependencies_get_invalid_files(ptr: *const Dependencies) -> *const Vec<String> {
    let deps = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    Box::into_raw(Box::new(deps.invalid_build_targets.clone()))
}

#[no_mangle]
pub extern "C" fn dependencies_is_circular(ptr: *const Dependencies) -> bool {
    let deps = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    deps.is_circular
}

#[no_mangle]
pub extern "C" fn dependencies_dispose(ptr: *mut Dependencies) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}
