use std::ffi::{c_char, CStr};
use toml_edit::{InlineTable, Value};

#[no_mangle]
pub extern "C" fn inline_table_is_empty(ptr: *const InlineTable) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_empty()
}

#[no_mangle]
pub extern "C" fn inline_table_len(ptr: *const InlineTable) -> usize {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.len()
}

#[no_mangle]
pub extern "C" fn inline_table_get(ptr: *const InlineTable, key: *const c_char) -> *const Value {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let key = unsafe {
        assert!(!key.is_null());
        CStr::from_ptr(key).to_str().unwrap()
    };

    match item.get(key) {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn inline_table_get_keys(ptr: *const InlineTable) -> *const Vec<String> {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let keys = item.iter().map(|(key, _)| key.to_string()).collect();
    Box::into_raw(Box::new(keys))
}

#[no_mangle]
pub extern "C" fn inline_table_get_array_keys(ptr: *const InlineTable) -> *const Vec<String> {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let keys = item
        .iter()
        .filter_map(|(key, item)| match item.is_array() {
            true => Some(key.to_string()),
            false => None,
        })
        .collect();
    Box::into_raw(Box::new(keys))
}

#[no_mangle]
pub extern "C" fn inline_table_get_inline_table_keys(
    ptr: *const InlineTable,
) -> *const Vec<String> {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let keys = item
        .iter()
        .filter_map(|(key, item)| match item.is_inline_table() {
            true => Some(key.to_string()),
            false => None,
        })
        .collect();
    Box::into_raw(Box::new(keys))
}

#[no_mangle]
pub extern "C" fn inline_table_contains_key(ptr: *const InlineTable, key: *const c_char) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let key = unsafe {
        assert!(!key.is_null());
        CStr::from_ptr(key).to_str().unwrap()
    };
    item.contains_key(key)
}

#[no_mangle]
pub extern "C" fn inline_table_dispose(ptr: *mut InlineTable) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}
