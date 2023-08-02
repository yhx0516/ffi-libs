use funny_utils_rs::ffi;
use std::ffi::c_char;
use std::fs;
use toml_edit::{Document, Item, Table};

#[no_mangle]
pub extern "C" fn document_parse_file(path: *const c_char) -> *const Document {
    let path = ffi::char_ptr_to_str(path);

    let Ok(content) = fs::read_to_string(&path) else {
        eprintln!("read file failed: {:?}",path);
        return std::ptr::null();
    };

    match content.parse::<Document>() {
        Ok(val) => Box::into_raw(Box::new(val)),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn document_parse_content(content: *const c_char) -> *const Document {
    let content = ffi::char_ptr_to_str(content);

    match content.parse::<Document>() {
        Ok(val) => Box::into_raw(Box::new(val)),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn document_get(ptr: *const Document, key: *const c_char) -> *const Item {
    let doc = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let key = ffi::char_ptr_to_str(key);

    match doc.get(&key) {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn document_get_keys(ptr: *const Document) -> *const Vec<String> {
    let doc = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let keys = doc.iter().map(|(key, _)| key.to_string()).collect();
    Box::into_raw(Box::new(keys))
}

#[no_mangle]
pub extern "C" fn document_get_array_keys(ptr: *const Document) -> *const Vec<String> {
    let doc = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let keys = doc
        .iter()
        .filter_map(|(key, item)| match item.is_array() {
            true => Some(key.to_string()),
            false => None,
        })
        .collect();
    Box::into_raw(Box::new(keys))
}

#[no_mangle]
pub extern "C" fn document_get_table_keys(ptr: *const Document) -> *const Vec<String> {
    let doc = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let keys = doc
        .iter()
        .filter_map(|(key, item)| match item.is_table() {
            true => Some(key.to_string()),
            false => None,
        })
        .collect();
    Box::into_raw(Box::new(keys))
}

#[no_mangle]
pub extern "C" fn document_get_inline_table_keys(ptr: *const Document) -> *const Vec<String> {
    let doc = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let keys = doc
        .iter()
        .filter_map(|(key, item)| match item.is_inline_table() {
            true => Some(key.to_string()),
            false => None,
        })
        .collect();
    Box::into_raw(Box::new(keys))
}

#[no_mangle]
pub extern "C" fn document_get_table_array_keys(ptr: *const Document) -> *const Vec<String> {
    let doc = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let keys = doc
        .iter()
        .filter_map(|(key, item)| match item.is_array_of_tables() {
            true => Some(key.to_string()),
            false => None,
        })
        .collect();
    Box::into_raw(Box::new(keys))
}

#[no_mangle]
pub extern "C" fn document_as_item(ptr: *const Document) -> *const Item {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    Box::into_raw(Box::new(item.as_item().to_owned()))
}

#[no_mangle]
pub extern "C" fn document_as_table(ptr: *const Document) -> *const Table {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    Box::into_raw(Box::new(item.as_table().to_owned()))
}

#[no_mangle]
pub extern "C" fn document_dispose(ptr: *mut Document) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}
