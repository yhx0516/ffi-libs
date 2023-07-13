use lazy_static::lazy_static;
use rutils::ffi;
use rutils::path::norm_path_extreme;
use std::ffi::c_char;
use std::sync::Mutex;

use crate::core::{BuildMap, Dependencies};
use crate::scan_files;
use crate::scan_files_block_by_manifest;
use crate::scan_files_block_by_pkg;
use crate::scan_files_block_by_pkg_manifest;

// ============================================================
// OnceLog 一次性日志，便于外部接入时调试
// ============================================================
lazy_static! {
    static ref ERR_BUFF_INS: Mutex<ErrorBuffer> = Mutex::new(ErrorBuffer::default());
}

#[derive(Debug, Default)]
struct ErrorBuffer {
    buffer: String,
}

impl ErrorBuffer {
    pub fn new(str: impl AsRef<str>) {
        let mut ins = ERR_BUFF_INS.lock().unwrap();
        ins.set_buffer(str);
    }

    pub fn output() -> String {
        let ins = ERR_BUFF_INS.lock().unwrap();
        let buffer = ins.get_buffer();
        match buffer.is_empty() {
            true => String::from("no error in rpkg"),
            false => buffer.to_owned(),
        }
    }

    pub fn set_buffer(&mut self, buffer: impl AsRef<str>) {
        self.buffer = buffer.as_ref().to_string();
    }

    fn get_buffer(&self) -> &String {
        &self.buffer
    }
}

#[no_mangle]
pub extern "C" fn try_get_err() -> *const c_char {
    let info = ErrorBuffer::output();
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
pub extern "C" fn rpkg_scan_files_block_by_pkg(
    root_path: *const c_char,
    patterns: *const *const c_char,
    patterns_len: usize,
) -> *const Vec<String> {
    let root_path = ffi::char_ptr_to_str(root_path);
    let patterns = ffi::arr_ptr_to_strs(patterns, patterns_len as usize);
    let patterns: Vec<&str> = patterns.iter().map(|s| s.as_ref()).collect();

    let files = scan_files_block_by_pkg(root_path, &patterns);
    Box::into_raw(Box::new(files))
}

#[no_mangle]
pub extern "C" fn rpkg_scan_files_block_by_manifest(
    root_path: *const c_char,
    patterns: *const *const c_char,
    patterns_len: usize,
) -> *const Vec<String> {
    let root_path = ffi::char_ptr_to_str(root_path);
    let patterns = ffi::arr_ptr_to_strs(patterns, patterns_len as usize);
    let patterns: Vec<&str> = patterns.iter().map(|s| s.as_ref()).collect();

    let files = scan_files_block_by_manifest(root_path, &patterns);
    Box::into_raw(Box::new(files))
}

#[no_mangle]
pub extern "C" fn rpkg_scan_files_block_by_pkg_manifest(
    root_path: *const c_char,
    patterns: *const *const c_char,
    patterns_len: usize,
) -> *const Vec<String> {
    let root_path = ffi::char_ptr_to_str(root_path);
    let patterns = ffi::arr_ptr_to_strs(patterns, patterns_len as usize);
    let patterns: Vec<&str> = patterns.iter().map(|s| s.as_ref()).collect();

    let files = scan_files_block_by_pkg_manifest(root_path, &patterns);
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
            ErrorBuffer::new(e.to_string());
            return std::ptr::null();
        }
    }
}

#[no_mangle]
pub extern "C" fn bm_insert(
    ptr: *mut BuildMap,
    addon_path: *const c_char,
    pkg_paths: *const *const c_char,
    pkg_paths_len: usize,
) -> bool {
    let build_map = unsafe { ptr.as_mut().expect("invalid ptr: ") };
    let addon_path = ffi::char_ptr_to_str(addon_path);
    let pkg_paths = ffi::arr_ptr_to_strs(pkg_paths, pkg_paths_len as usize);
    let pkg_paths: Vec<&str> = pkg_paths.iter().map(|s| s.as_ref()).collect();

    match build_map.insert(addon_path, pkg_paths) {
        Ok(_) => true,
        Err(e) => {
            ErrorBuffer::new(e.to_string());
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
            let str = format!("{}, {}", e.root_cause().to_string(), e.to_string());
            ErrorBuffer::new(str);
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
            let str = format!("{}, {}", e.root_cause().to_string(), e.to_string());
            ErrorBuffer::new(str);
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
            let str = format!("{}, {}", e.root_cause().to_string(), e.to_string());
            ErrorBuffer::new(str);
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
            let str = format!("{}, {}", e.root_cause().to_string(), e.to_string());
            ErrorBuffer::new(str);
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
            let str = format!("{}, {}", e.root_cause().to_string(), e.to_string());
            ErrorBuffer::new(str);
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn bm_get_bundle_paths(
    ptr: *mut BuildMap,
    addon_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let addon_path = ffi::char_ptr_to_str(addon_path);

    let target_paths = match build_map.get_bundle_paths(addon_path) {
        Ok(v) => v,
        Err(e) => {
            ErrorBuffer::new(e.to_string());
            return std::ptr::null();
        }
    };

    let assets: Vec<_> = target_paths.iter().map(|s| s.to_string()).collect();
    Box::into_raw(Box::new(assets))
}

#[no_mangle]
pub extern "C" fn bm_get_subscene_paths(
    ptr: *mut BuildMap,
    addon_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let addon_path = ffi::char_ptr_to_str(addon_path);

    let target_paths = match build_map.get_subscene_paths(addon_path) {
        Ok(v) => v,
        Err(e) => {
            ErrorBuffer::new(e.to_string());
            return std::ptr::null();
        }
    };

    let assets: Vec<_> = target_paths.iter().map(|s| s.to_string()).collect();
    Box::into_raw(Box::new(assets))
}

#[no_mangle]
pub extern "C" fn bm_get_file_paths(
    ptr: *mut BuildMap,
    addon_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let addon_path = ffi::char_ptr_to_str(addon_path);

    let target_paths = match build_map.get_file_paths(addon_path) {
        Ok(v) => v,
        Err(e) => {
            ErrorBuffer::new(e.to_string());
            return std::ptr::null();
        }
    };

    let assets: Vec<_> = target_paths.iter().map(|s| s.to_string()).collect();
    Box::into_raw(Box::new(assets))
}

#[no_mangle]
pub extern "C" fn bm_get_dylib_paths(
    ptr: *mut BuildMap,
    addon_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let addon_path = ffi::char_ptr_to_str(addon_path);

    let target_paths = match build_map.get_dylib_paths(addon_path) {
        Ok(v) => v,
        Err(e) => {
            ErrorBuffer::new(e.to_string());
            return std::ptr::null();
        }
    };

    let assets: Vec<_> = target_paths.iter().map(|s| s.to_string()).collect();
    Box::into_raw(Box::new(assets))
}

#[no_mangle]
pub extern "C" fn bm_get_zip_paths(
    ptr: *mut BuildMap,
    addon_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let addon_path = ffi::char_ptr_to_str(addon_path);

    let target_paths = match build_map.get_zip_paths(addon_path) {
        Ok(v) => v,
        Err(e) => {
            ErrorBuffer::new(e.to_string());
            return std::ptr::null();
        }
    };

    let assets: Vec<_> = target_paths.iter().map(|s| s.to_string()).collect();
    Box::into_raw(Box::new(assets))
}

#[no_mangle]
pub extern "C" fn bm_get_bundle_paths_from_pkg(
    ptr: *mut BuildMap,
    addon_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let addon_path = ffi::char_ptr_to_str(addon_path);

    let target_paths = match build_map.get_bundle_paths_from_pkg(addon_path) {
        Ok(v) => v,
        Err(e) => {
            ErrorBuffer::new(e.to_string());
            return std::ptr::null();
        }
    };

    let assets: Vec<_> = target_paths.iter().map(|s| s.to_string()).collect();
    Box::into_raw(Box::new(assets))
}

#[no_mangle]
pub extern "C" fn bm_get_subscene_paths_from_pkg(
    ptr: *mut BuildMap,
    addon_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let addon_path = ffi::char_ptr_to_str(addon_path);

    let target_paths = match build_map.get_subscene_paths_from_pkg(addon_path) {
        Ok(v) => v,
        Err(e) => {
            ErrorBuffer::new(e.to_string());
            return std::ptr::null();
        }
    };

    let assets: Vec<_> = target_paths.iter().map(|s| s.to_string()).collect();
    Box::into_raw(Box::new(assets))
}

#[no_mangle]
pub extern "C" fn bm_get_file_paths_from_pkg(
    ptr: *mut BuildMap,
    addon_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let addon_path = ffi::char_ptr_to_str(addon_path);

    let target_paths = match build_map.get_file_paths_from_pkg(addon_path) {
        Ok(v) => v,
        Err(e) => {
            ErrorBuffer::new(e.to_string());
            return std::ptr::null();
        }
    };

    let assets: Vec<_> = target_paths.iter().map(|s| s.to_string()).collect();
    Box::into_raw(Box::new(assets))
}

#[no_mangle]
pub extern "C" fn bm_get_dylib_paths_from_pkg(
    ptr: *mut BuildMap,
    addon_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let addon_path = ffi::char_ptr_to_str(addon_path);

    let target_paths = match build_map.get_dylib_paths_from_pkg(addon_path) {
        Ok(v) => v,
        Err(e) => {
            ErrorBuffer::new(e.to_string());
            return std::ptr::null();
        }
    };

    let assets: Vec<_> = target_paths.iter().map(|s| s.to_string()).collect();
    Box::into_raw(Box::new(assets))
}

#[no_mangle]
pub extern "C" fn bm_get_zip_paths_from_pkg(
    ptr: *mut BuildMap,
    addon_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let addon_path = ffi::char_ptr_to_str(addon_path);

    let target_paths = match build_map.get_zip_paths_from_pkg(addon_path) {
        Ok(v) => v,
        Err(e) => {
            ErrorBuffer::new(e.to_string());
            return std::ptr::null();
        }
    };

    let assets: Vec<_> = target_paths.iter().map(|s| s.to_string()).collect();
    Box::into_raw(Box::new(assets))
}

// asset path's ancestor is "Assets"
#[no_mangle]
pub extern "C" fn bm_get_bundle_assets(
    ptr: *mut BuildMap,
    target_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let target_path = ffi::char_ptr_to_str(target_path);

    let assets: Vec<_> = build_map
        .get_bundle_assets(target_path)
        .iter()
        .map(|s| s.to_string())
        .collect();
    Box::into_raw(Box::new(assets))
}

// asset path's ancestor is target_path, and
#[no_mangle]
pub extern "C" fn bm_get_bundle_addresable_assets(
    ptr: *mut BuildMap,
    target_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let target_path = ffi::char_ptr_to_str(target_path);

    let assets: Vec<_> = build_map
        .get_bundle_assets(&target_path)
        .iter()
        .map(|s| {
            let path = s.strip_prefix(&target_path).unwrap();
            norm_path_extreme(path).to_lowercase()
        })
        .collect();
    Box::into_raw(Box::new(assets))
}

#[no_mangle]
pub extern "C" fn bm_get_subscene_assets(
    ptr: *mut BuildMap,
    target_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let target_path = ffi::char_ptr_to_str(target_path);

    let assets: Vec<_> = build_map
        .get_subscene_assets(target_path)
        .iter()
        .map(|s| s.to_string())
        .collect();
    Box::into_raw(Box::new(assets))
}

#[no_mangle]
pub extern "C" fn bm_get_dylib_assets(
    ptr: *mut BuildMap,
    target_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let target_path = ffi::char_ptr_to_str(target_path);

    let assets: Vec<_> = build_map
        .get_dylib_assets(target_path)
        .iter()
        .map(|s| s.to_string())
        .collect();
    Box::into_raw(Box::new(assets))
}

#[no_mangle]
pub extern "C" fn bm_get_file_assets(
    ptr: *mut BuildMap,
    target_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let target_path = ffi::char_ptr_to_str(target_path);

    let assets: Vec<_> = build_map
        .get_file_assets(target_path)
        .iter()
        .map(|s| s.to_string())
        .collect();
    Box::into_raw(Box::new(assets))
}

#[no_mangle]
pub extern "C" fn bm_get_zip_assets(
    ptr: *mut BuildMap,
    target_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let target_path = ffi::char_ptr_to_str(target_path);

    let assets: Vec<_> = build_map
        .get_zip_assets(target_path)
        .iter()
        .map(|s| s.to_string())
        .collect();
    Box::into_raw(Box::new(assets))
}

#[no_mangle]
pub extern "C" fn bm_get_asset_urls(
    ptr: *const BuildMap,
    addon_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let addon_path = ffi::char_ptr_to_str(addon_path);

    let urls = match build_map.get_asset_urls(addon_path) {
        Ok(v) => v,
        Err(e) => {
            ErrorBuffer::new(e.to_string());
            return std::ptr::null();
        }
    };

    let urls: Vec<_> = urls.iter().map(|s| s.to_string()).collect();
    Box::into_raw(Box::new(urls))
}

#[no_mangle]
pub extern "C" fn bm_find_bundle_path(
    ptr: *const BuildMap,
    bundle_path: *const c_char,
) -> *const c_char {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let bundle_path = ffi::char_ptr_to_str(bundle_path);
    match build_map.find_bundle_path(bundle_path) {
        Ok(v) => ffi::str_to_char_ptr(v),
        Err(e) => {
            ErrorBuffer::new(e.to_string());
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
    Box::into_raw(Box::new(deps.target_paths.clone()))
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
