use std::ffi::c_char;

use crate::core::dependencies::{seek_dependencies, Dependencies};
use crate::core::match_patterns;

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
// PKG Match Patterns
// ============================================================
// #[no_mangle]
// pub extern "C" fn pkg_match_file(path: *const c_char) -> *const Vec<String> {
//     let file = rutils::char_ptr_to_str(path);
//     let files = pkg::match_file(&file);
//     Box::into_raw(Box::new(files))
// }

#[no_mangle]
pub extern "C" fn pkg_match_patterns(
    root_path: *const c_char,
    patterns: *const *const c_char,
    patterns_len: usize,
) -> *const Vec<String> {
    let root_path = rutils::char_ptr_to_str(root_path);
    let patterns = rutils::arr_ptr_to_strs(patterns, patterns_len as usize);
    let patterns: Vec<&str> = patterns.iter().map(|s| s.as_ref()).collect();

    let files = match_patterns(root_path, &patterns);
    Box::into_raw(Box::new(files))
}

// ============================================================
// PKG Seek Dependencies
// ============================================================
#[no_mangle]
pub extern "C" fn pkg_seek_dependencies(
    root_path: *const c_char,
    cur_pkg: *const c_char,
    patterns: *const *const c_char,
    patterns_len: usize,
) -> *const Dependencies {
    let root_path = rutils::char_ptr_to_str(root_path);
    let cur_pkg = rutils::char_ptr_to_str(cur_pkg);
    let patterns = rutils::arr_ptr_to_strs(patterns, patterns_len as usize);
    let patterns: Vec<&str> = patterns.iter().map(|s| s.as_ref()).collect();

    let deps = seek_dependencies(root_path, cur_pkg, &patterns);
    Box::into_raw(Box::new(deps))
}
#[no_mangle]
pub extern "C" fn dependencies_get_files(ptr: *const Dependencies) -> *const Vec<String> {
    let deps = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    Box::into_raw(Box::new(deps.files.clone()))
}

#[no_mangle]
pub extern "C" fn dependencies_get_invalid_files(ptr: *const Dependencies) -> *const Vec<String> {
    let deps = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    Box::into_raw(Box::new(deps.invalid_files.clone()))
}

#[no_mangle]
pub extern "C" fn dependencies_is_circular(ptr: *const Dependencies) -> bool {
    let deps = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    deps.is_circular
}

#[no_mangle]
pub fn dependencies_dispose(ptr: *mut Dependencies) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}
