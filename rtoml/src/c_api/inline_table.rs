use funny_utils_rs::ffi;
use std::ffi::{c_char, CStr};
use toml_edit::{InlineTable, Value};

#[no_mangle]
pub extern "C" fn inline_table_is_empty(ptr: *const InlineTable) -> bool {
    let inline_table = unsafe { ptr.as_ref().expect("invalid ptr") };
    inline_table.is_empty()
}

#[no_mangle]
pub extern "C" fn inline_table_len(ptr: *const InlineTable) -> usize {
    let inline_table = unsafe { ptr.as_ref().expect("invalid ptr") };
    inline_table.len()
}

#[no_mangle]
pub extern "C" fn inline_table_get(ptr: *mut InlineTable, key: *const c_char) -> *mut Value {
    let inline_table = unsafe { ptr.as_mut().expect("invalid ptr") };
    let key = unsafe {
        assert!(!key.is_null());
        CStr::from_ptr(key).to_str().unwrap()
    };

    match inline_table.get_mut(key) {
        Some(val) => val as *mut _,
        _ => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn inline_table_get_keys(ptr: *const InlineTable) -> *const Vec<String> {
    let inline_table = unsafe { ptr.as_ref().expect("invalid ptr") };
    let keys = inline_table
        .iter()
        .map(|(key, _)| key.to_string())
        .collect();
    Box::into_raw(Box::new(keys))
}

#[no_mangle]
pub extern "C" fn inline_table_get_array_keys(ptr: *const InlineTable) -> *const Vec<String> {
    let inline_table = unsafe { ptr.as_ref().expect("invalid ptr") };
    let keys = inline_table
        .iter()
        .filter_map(|(key, val)| match val.is_array() {
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
    let inline_table = unsafe { ptr.as_ref().expect("invalid ptr") };
    let keys = inline_table
        .iter()
        .filter_map(|(key, val)| match val.is_inline_table() {
            true => Some(key.to_string()),
            false => None,
        })
        .collect();
    Box::into_raw(Box::new(keys))
}

#[no_mangle]
pub extern "C" fn inline_table_contains_key(ptr: *const InlineTable, key: *const c_char) -> bool {
    let inline_table = unsafe { ptr.as_ref().expect("invalid ptr") };
    let key = unsafe {
        assert!(!key.is_null());
        CStr::from_ptr(key).to_str().unwrap()
    };
    inline_table.contains_key(key)
}

#[no_mangle]
pub extern "C" fn inline_table_dispose(ptr: *mut InlineTable) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn inline_table_new() -> *mut InlineTable {
    let inline_table = InlineTable::new();
    Box::into_raw(Box::new(inline_table))
}

#[no_mangle]
pub extern "C" fn inline_table_insert(
    ptr: *mut InlineTable,
    key: *const c_char,
    val: *const Value,
) -> bool {
    let inline_table = unsafe { ptr.as_mut().expect("invalid ptr") };
    let key = ffi::char_ptr_to_str(key);
    let val = unsafe { val.as_ref().expect("invalid ptr") };
    inline_table.insert(key, val.to_owned()).is_some()
}

#[no_mangle]
pub extern "C" fn inline_table_remove(ptr: *mut InlineTable, key: *const c_char) -> bool {
    let inline_table = unsafe { ptr.as_mut().expect("invalid ptr") };
    let key = ffi::char_ptr_to_str(key);
    inline_table.remove(&key).is_some()
}

#[no_mangle]
pub extern "C" fn inline_table_clear(ptr: *mut InlineTable) {
    let inline_table = unsafe { ptr.as_mut().expect("invalid ptr") };
    inline_table.clear();
}

#[no_mangle]
pub extern "C" fn inline_table_to_string(ptr: *const InlineTable) -> *const c_char {
    let inline_table = unsafe { ptr.as_ref().expect("invalid ptr") };
    let str = inline_table.to_string();
    ffi::str_to_char_ptr(str)
}
