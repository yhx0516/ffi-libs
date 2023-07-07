use std::ffi::c_char;

use crate::ffi::str_to_char_ptr;

// ============================================================
// Vec<String>
// ============================================================
#[no_mangle]
pub extern "C" fn strs_len(ptr: *const Vec<String>) -> usize {
    let strs = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    strs.len()
}

#[no_mangle]
pub extern "C" fn strs_get(ptr: *const Vec<String>, index: usize) -> *const c_char {
    let strs = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    match strs.get(index) {
        Some(s) => str_to_char_ptr(s),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn strs_dispose(ptr: *mut Vec<String>) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}
