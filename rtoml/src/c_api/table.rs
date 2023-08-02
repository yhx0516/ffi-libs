use funny_utils_rs::ffi;
use std::ffi::{c_char, CStr};
use toml_edit::{Item, Table};

#[no_mangle]
pub extern "C" fn table_is_empty(ptr: *const Table) -> bool {
    let table = unsafe { ptr.as_ref().expect("invalid ptr") };
    table.is_empty()
}

#[no_mangle]
pub extern "C" fn table_len(ptr: *const Table) -> usize {
    let table = unsafe { ptr.as_ref().expect("invalid ptr") };
    table.len()
}

#[no_mangle]
pub extern "C" fn table_get(ptr: *const Table, key: *const c_char) -> *const Item {
    let table = unsafe { ptr.as_ref().expect("invalid ptr") };
    let key = ffi::char_ptr_to_str(key);

    match table.get(&key) {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn table_get_keys(ptr: *const Table) -> *const Vec<String> {
    let table = unsafe { ptr.as_ref().expect("invalid ptr") };
    let keys = table.iter().map(|(key, _)| key.to_string()).collect();
    Box::into_raw(Box::new(keys))
}

#[no_mangle]
pub extern "C" fn table_get_array_keys(ptr: *const Table) -> *const Vec<String> {
    let table = unsafe { ptr.as_ref().expect("invalid ptr") };
    let keys = table
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
    let table = unsafe { ptr.as_ref().expect("invalid ptr") };
    let keys = table
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
    let table = unsafe { ptr.as_ref().expect("invalid ptr") };
    let key = ffi::char_ptr_to_str(key);
    table.contains_key(&key)
}

#[no_mangle]
pub extern "C" fn table_contains_table(ptr: *const Table, key: *const c_char) -> bool {
    let table = unsafe { ptr.as_ref().expect("invalid ptr") };
    let key = ffi::char_ptr_to_str(key);
    table.contains_table(&key)
}

#[no_mangle]
pub extern "C" fn table_contains_value(ptr: *const Table, key: *const c_char) -> bool {
    let table = unsafe { ptr.as_ref().expect("invalid ptr") };
    let key = ffi::char_ptr_to_str(key);
    table.contains_value(&key)
}

#[no_mangle]
pub extern "C" fn table_contains_array_of_tables(ptr: *const Table, key: *const c_char) -> bool {
    let table = unsafe { ptr.as_ref().expect("invalid ptr") };
    let key = unsafe {
        assert!(!key.is_null());
        CStr::from_ptr(key).to_str().unwrap()
    };
    table.contains_array_of_tables(key)
}

#[no_mangle]
pub extern "C" fn table_dispose(ptr: *mut Table) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn table_new() -> *mut Table {
    let table = Table::new();
    Box::into_raw(Box::new(table))
}

#[no_mangle]
pub extern "C" fn table_insert(ptr: *mut Table, key: *const c_char, item: *const Item) -> bool {
    let table = unsafe { ptr.as_mut().expect("invalid ptr") };
    let key = ffi::char_ptr_to_str(key);
    let item = unsafe { item.as_ref().expect("invalid ptr") };
    table.insert(&key, item.to_owned()).is_some()
}

#[no_mangle]
pub extern "C" fn table_remove(ptr: *mut Table, key: *const c_char) -> bool {
    let table = unsafe { ptr.as_mut().expect("invalid ptr") };
    let key = ffi::char_ptr_to_str(key);
    table.remove(&key).is_some()
}

#[no_mangle]
pub extern "C" fn table_clear(ptr: *mut Table) {
    let table = unsafe { ptr.as_mut().expect("invalid ptr") };
    table.clear();
}

#[no_mangle]
pub extern "C" fn table_to_string(ptr: *const Table) -> *const c_char {
    let table = unsafe { ptr.as_ref().expect("invalid ptr") };
    let str = table.to_string();
    ffi::str_to_char_ptr(str)
}
