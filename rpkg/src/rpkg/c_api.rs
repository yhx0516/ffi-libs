
use rutils::ffi;
use std::ffi::c_char;


use crate::core::{BuildMap, Dependencies};
use crate::scan_files;
use crate::scan_files_block_by_manifest;
use crate::scan_files_block_by_pkg;
use crate::scan_files_block_by_pkg_manifest;

// ============================================================
// ErrorBuffer 便于外部接入时调试
// ============================================================
use lazy_static::lazy_static;
use std::sync::Mutex;

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
pub extern "C" fn bm_get_target_types(
    ptr: *mut BuildMap,
    addon_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let addon_path = ffi::char_ptr_to_str(addon_path);

    let target_types = match build_map.get_target_types(addon_path) {
        Ok(v) => v,
        Err(e) => {
            ErrorBuffer::new(e.to_string());
            return std::ptr::null();
        }
    };

    let target_types = target_types.iter().map(|s| s.to_string()).collect();
    Box::into_raw(Box::new(target_types))
}

#[no_mangle]
pub extern "C" fn bm_get_target_types_from_pkg(
    ptr: *mut BuildMap,
    pkg_path: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let pkg_path = ffi::char_ptr_to_str(pkg_path);

    let target_types = match build_map.get_target_types_from_pkg(pkg_path) {
        Ok(v) => v,
        Err(e) => {
            ErrorBuffer::new(e.to_string());
            return std::ptr::null();
        }
    };

    let target_types = target_types.iter().map(|s| s.to_string()).collect();
    Box::into_raw(Box::new(target_types))
}

#[no_mangle]
pub extern "C" fn bm_get_target_paths(
    ptr: *mut BuildMap,
    addon_path: *const c_char,
    target_type: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let addon_path = ffi::char_ptr_to_str(addon_path);
    let target_type = ffi::char_ptr_to_str(target_type);

    let target_paths = match build_map.get_target_paths(addon_path, target_type) {
        Ok(v) => v,
        Err(e) => {
            ErrorBuffer::new(e.to_string());
            return std::ptr::null();
        }
    };

    let target_paths: Vec<_> = target_paths.iter().map(|s| s.to_string()).collect();
    Box::into_raw(Box::new(target_paths))
}

#[no_mangle]
pub extern "C" fn bm_get_target_paths_from_pkg(
    ptr: *mut BuildMap,
    pkg_path: *const c_char,
    target_type: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let pkg_path = ffi::char_ptr_to_str(pkg_path);
    let target_type = ffi::char_ptr_to_str(target_type);

    let target_paths = match build_map.get_target_paths_from_pkg(pkg_path, target_type) {
        Ok(v) => v,
        Err(e) => {
            ErrorBuffer::new(e.to_string());
            return std::ptr::null();
        }
    };

    let target_paths: Vec<_> = target_paths.iter().map(|s| s.to_string()).collect();
    Box::into_raw(Box::new(target_paths))
}

#[no_mangle]
pub extern "C" fn bm_resolve_target_deps(
    ptr: *mut BuildMap,
    target_path: *const c_char,
    target_type: *const c_char,
) -> *const Dependencies {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let target_path = ffi::char_ptr_to_str(target_path);
    let target_type = ffi::char_ptr_to_str(target_type);

    match build_map.resolve_target_deps(target_path, target_type) {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(e) => {
            let str = format!("{}, {}", e.root_cause().to_string(), e.to_string());
            ErrorBuffer::new(str);
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn bm_get_target_assets(
    ptr: *mut BuildMap,
    target_path: *const c_char,
    target_type: *const c_char,
) -> *const Vec<String> {
    let build_map = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let target_path = ffi::char_ptr_to_str(target_path);
    let target_type = ffi::char_ptr_to_str(target_type);

    let assets: Vec<_> = build_map
        .get_target_assets(target_path, target_type)
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
