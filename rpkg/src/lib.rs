use std::ffi::c_char;

pub mod pkg;
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
// PKG Matcher
// ============================================================
#[no_mangle]
pub extern "C" fn pkg_match_file(path: *const c_char) -> *const Vec<String> {
    let file = rutils::char_ptr_to_str(path);
    let files = pkg::match_file(&file);
    Box::into_raw(Box::new(files))
}

#[no_mangle]
pub extern "C" fn pkg_match_patterns(
    root_path: *const c_char,
    patterns: *const *const c_char,
    patterns_len: usize,
) -> *const Vec<String> {
    let root_path = rutils::char_ptr_to_str(root_path);
    let patterns = rutils::arr_ptr_to_strs(patterns, patterns_len as usize);
    let patterns: Vec<&str> = patterns.iter().map(|s| s.as_ref()).collect();

    let files = pkg::match_patterns(root_path, &patterns);
    Box::into_raw(Box::new(files))
}

#[no_mangle]
pub extern "C" fn strs_len(ptr: *const Vec<String>) -> usize {
    let strs = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    strs.len()
}

#[no_mangle]
pub extern "C" fn strs_get(ptr: *const Vec<String>, index: usize) -> *const c_char {
    let strs = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    match strs.get(index) {
        Some(s) => rutils::str_to_char_ptr(s),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub fn dispose_strs(ptr: *mut Vec<String>) {
    unsafe { Box::from_raw(ptr) };
}
