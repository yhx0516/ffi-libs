use once_cell::sync::OnceCell;
use rutils::ffi;
use std::ffi::c_char;

use crate::core::{Assets, BuildMap, Dependencies};
use crate::scan_files;
use crate::scan_files_block_manifest;
use crate::scan_files_block_pkg;
use crate::scan_files_block_pkg_manifest;

// ============================================================
// OnceLog api 一次性日志，便于外部接入时调试
// ============================================================
static ERR_BUFFERR_INS: OnceCell<OnceLog> = OnceCell::new();

#[derive(Debug)]
struct OnceLog {
    buffer: String,
}

impl OnceLog {
    pub fn new(str: impl AsRef<str>) {
        let ins = OnceLog {
            buffer: str.as_ref().to_string(),
        };
        if ERR_BUFFERR_INS.set(ins).is_err() {
            eprintln!("set OnceLog instance multiple times")
        }
    }

    pub fn output() -> String {
        match ERR_BUFFERR_INS.get() {
            Some(ins) => ins.get_buff().to_owned(),
            None => String::from("no error in rpkg"),
        }
    }

    fn get_buff(&self) -> &String {
        &self.buffer
    }
}

#[no_mangle]
pub extern "C" fn try_log_once() -> *const c_char {
    let info = OnceLog::output();
    ffi::str_to_char_ptr(info)
}

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
    ffi::str_to_char_ptr(&version)
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
    let root_path = ffi::char_ptr_to_str(root_path);
    let patterns = ffi::arr_ptr_to_strs(patterns, patterns_len as usize);
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
    let root_path = ffi::char_ptr_to_str(root_path);
    let patterns = ffi::arr_ptr_to_strs(patterns, patterns_len as usize);
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
    let root_path = ffi::char_ptr_to_str(root_path);
    let patterns = ffi::arr_ptr_to_strs(patterns, patterns_len as usize);
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
    let root_path = ffi::char_ptr_to_str(root_path);
    let patterns = ffi::arr_ptr_to_strs(patterns, patterns_len as usize);
    let patterns: Vec<&str> = patterns.iter().map(|s| s.as_ref()).collect();

    let files = scan_files_block_pkg_manifest(root_path, &patterns);
    Box::into_raw(Box::new(files))
}

// ============================================================
// BuildMap api
// ============================================================
#[no_mangle]
pub extern "C" fn bm_new(root_path: *const c_char) -> *const BuildMap {
    let root_path = ffi::char_ptr_to_str(root_path);

    match BuildMap::new(root_path) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            OnceLog::new(e.to_string());
            return std::ptr::null();
        }
    }
}

#[no_mangle]
pub extern "C" fn bm_insert(
    ptr: *mut BuildMap,
    mount_path: *const c_char,
    pkg_paths: *const *const c_char,
    pkg_paths_len: usize,
) -> bool {
    let build_map = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    let mount_path = ffi::char_ptr_to_str(mount_path);
    let pkg_paths = ffi::arr_ptr_to_strs(pkg_paths, pkg_paths_len as usize);
    let pkg_paths: Vec<&str> = pkg_paths.iter().map(|s| s.as_ref()).collect();

    match build_map.insert(mount_path, pkg_paths) {
        Ok(_) => true,
        Err(e) => {
            OnceLog::new(e.to_string());
            eprintln!("{}", e.to_string());
            return false;
        }
    }
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
    let target_path = ffi::char_ptr_to_str(target_path);

    match build_map.resolve_bundle_deps(target_path) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            OnceLog::new(e.to_string());
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
    let target_path = ffi::char_ptr_to_str(target_path);

    match build_map.resolve_subscene_deps(target_path) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            OnceLog::new(e.to_string());
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
    let target_path = ffi::char_ptr_to_str(target_path);

    match build_map.resolve_dylib_deps(target_path) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            OnceLog::new(e.to_string());
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
    let target_path = ffi::char_ptr_to_str(target_path);

    match build_map.resolve_file_deps(target_path) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            OnceLog::new(e.to_string());
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
    let target_path = ffi::char_ptr_to_str(target_path);

    match build_map.resolve_zip_deps(target_path) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            OnceLog::new(e.to_string());
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn bm_scan_bundle_assets(
    ptr: *mut BuildMap,
    mount_path: *const c_char,
    target_path: *const c_char,
) -> *const Assets {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let mount_path = ffi::char_ptr_to_str(mount_path);
    let target_path = ffi::char_ptr_to_str(target_path);

    match build_map.scan_bundle_assets(mount_path, target_path) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            OnceLog::new(e.to_string());
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn bm_scan_subscene_assets(
    ptr: *mut BuildMap,
    mount_path: *const c_char,
    target_path: *const c_char,
) -> *const Assets {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let mount_path = ffi::char_ptr_to_str(mount_path);
    let target_path = ffi::char_ptr_to_str(target_path);

    match build_map.scan_subscene_assets(mount_path, target_path) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            OnceLog::new(e.to_string());
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn bm_scan_dylib_assets(
    ptr: *mut BuildMap,
    mount_path: *const c_char,
    target_path: *const c_char,
) -> *const Assets {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let mount_path = ffi::char_ptr_to_str(mount_path);
    let target_path = ffi::char_ptr_to_str(target_path);

    match build_map.scan_dylib_assets(mount_path, target_path) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            OnceLog::new(e.to_string());
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn bm_scan_file_assets(
    ptr: *mut BuildMap,
    mount_path: *const c_char,
    target_path: *const c_char,
) -> *const Assets {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let mount_path = ffi::char_ptr_to_str(mount_path);
    let target_path = ffi::char_ptr_to_str(target_path);

    match build_map.scan_file_assets(mount_path, target_path) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            OnceLog::new(e.to_string());
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn bm_scan_zip_assets(
    ptr: *mut BuildMap,
    mount_path: *const c_char,
    target_path: *const c_char,
) -> *const Assets {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let mount_path = ffi::char_ptr_to_str(mount_path);
    let target_path = ffi::char_ptr_to_str(target_path);

    match build_map.scan_zip_assets(mount_path, target_path) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            OnceLog::new(e.to_string());
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn bm_find_bundle_url(
    ptr: *const BuildMap,
    bundle_path: *const c_char,
) -> *const c_char {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let bundle_path = ffi::char_ptr_to_str(bundle_path);
    match build_map.find_bundle_url(bundle_path) {
        Ok(v) => ffi::str_to_char_ptr(v),
        Err(e) => {
            OnceLog::new(e.to_string());
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn bm_get_root_path(ptr: *const BuildMap) -> *const c_char {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let root_path = build_map.get_root_path();
    ffi::str_to_char_ptr(root_path)
}

#[no_mangle]
pub extern "C" fn bm_debug_info(ptr: *const BuildMap) -> *const c_char {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    ffi::str_to_char_ptr(&build_map.to_string())
}

// ============================================================
// Dependencies api
// ============================================================
#[no_mangle]
pub extern "C" fn dependencies_get_targets(ptr: *const Dependencies) -> *const Vec<String> {
    let deps = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    Box::into_raw(Box::new(deps.build_targets.clone()))
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

// ============================================================
// Assets api
// ============================================================
#[no_mangle]
pub extern "C" fn assets_dispose(ptr: *mut Assets) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn assets_get_paths(ptr: *const Assets) -> *const Vec<String> {
    let assets = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let paths: Vec<String> = assets.get_paths().iter().map(ToString::to_string).collect();
    Box::into_raw(Box::new(paths))
}

#[no_mangle]
pub extern "C" fn assets_get_urls(ptr: *const Assets) -> *const Vec<String> {
    let assets = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let paths: Vec<String> = assets.get_urls().iter().map(ToString::to_string).collect();
    Box::into_raw(Box::new(paths))
}
