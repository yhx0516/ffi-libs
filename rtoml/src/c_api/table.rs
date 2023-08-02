use funny_utils_rs::ffi;
use std::ffi::{c_char, CStr};
use toml_edit::{Item, Table};

#[no_mangle]
pub extern "C" fn table_is_empty(ptr: *const Table) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_empty()
}

#[no_mangle]
pub extern "C" fn table_len(ptr: *const Table) -> usize {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.len()
}

#[no_mangle]
pub extern "C" fn table_get(ptr: *const Table, key: *const c_char) -> *const Item {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let key = ffi::char_ptr_to_str(key);

    match item.get(&key) {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn table_get_keys(ptr: *const Table) -> *const Vec<String> {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let keys = item.iter().map(|(key, _)| key.to_string()).collect();
    Box::into_raw(Box::new(keys))
}

#[no_mangle]
pub extern "C" fn table_get_array_keys(ptr: *const Table) -> *const Vec<String> {
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
pub extern "C" fn table_get_inline_table_keys(ptr: *const Table) -> *const Vec<String> {
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
pub extern "C" fn table_contains_key(ptr: *const Table, key: *const c_char) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let key = ffi::char_ptr_to_str(key);
    item.contains_key(&key)
}

#[no_mangle]
pub extern "C" fn table_contains_table(ptr: *const Table, key: *const c_char) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let key = ffi::char_ptr_to_str(key);
    item.contains_table(&key)
}

#[no_mangle]
pub extern "C" fn table_contains_value(ptr: *const Table, key: *const c_char) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let key = ffi::char_ptr_to_str(key);
    item.contains_value(&key)
}

#[no_mangle]
pub extern "C" fn table_contains_array_of_tables(ptr: *const Table, key: *const c_char) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let key = unsafe {
        assert!(!key.is_null());
        CStr::from_ptr(key).to_str().unwrap()
    };
    item.contains_array_of_tables(key)
}

#[no_mangle]
pub extern "C" fn table_dispose(ptr: *mut Table) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}
