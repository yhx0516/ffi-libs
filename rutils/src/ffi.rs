use std::ffi::{c_char, CStr, CString};

pub fn char_ptr_to_str(ptr: *const c_char) -> String {
    let val = unsafe {
        assert!(!ptr.is_null());
        CStr::from_ptr(ptr).to_str().unwrap()
    };
    val.to_string()
}

pub fn str_to_char_ptr(str: impl AsRef<str>) -> *const c_char {
    let c_str = CString::new(str.as_ref()).unwrap();
    c_str.into_raw()
}

pub fn arr_ptr_to_strs(ptr: *const *const c_char, len: usize) -> Vec<String> {
    let c_string_slice = unsafe { std::slice::from_raw_parts(ptr, len) };
    let rust_strings = c_string_slice
        .iter()
        .map(|&c_string_ptr| {
            let c_str = unsafe { CStr::from_ptr(c_string_ptr) };
            let rust_string = c_str.to_string_lossy().to_string();
            rust_string
        })
        .collect::<Vec<String>>();
    rust_strings
}
