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
        Ok(doc) => Box::into_raw(Box::new(doc)),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn document_parse_content(content: *const c_char) -> *const Document {
    let content = ffi::char_ptr_to_str(content);

    match content.parse::<Document>() {
        Ok(doc) => Box::into_raw(Box::new(doc)),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn document_get(ptr: *const Document, key: *const c_char) -> *const Item {
    let doc = unsafe { ptr.as_ref().expect("invalid ptr") };
    let key = ffi::char_ptr_to_str(key);

    match doc.get(&key) {
        Some(item) => Box::into_raw(Box::new(item.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn document_get_keys(ptr: *const Document) -> *const Vec<String> {
    let doc = unsafe { ptr.as_ref().expect("invalid ptr") };
    let keys = doc.iter().map(|(key, _)| key.to_string()).collect();
    Box::into_raw(Box::new(keys))
}

#[no_mangle]
pub extern "C" fn document_get_array_keys(ptr: *const Document) -> *const Vec<String> {
    let doc = unsafe { ptr.as_ref().expect("invalid ptr") };
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
    let doc = unsafe { ptr.as_ref().expect("invalid ptr") };
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
    let doc = unsafe { ptr.as_ref().expect("invalid ptr") };
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
    let doc = unsafe { ptr.as_ref().expect("invalid ptr") };
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
    let doc = unsafe { ptr.as_ref().expect("invalid ptr") };
    Box::into_raw(Box::new(doc.as_item().to_owned()))
}

#[no_mangle]
pub extern "C" fn document_as_table(ptr: *const Document) -> *const Table {
    let doc = unsafe { ptr.as_ref().expect("invalid ptr") };
    Box::into_raw(Box::new(doc.as_table().to_owned()))
}

#[no_mangle]
pub extern "C" fn document_dispose(ptr: *mut Document) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn document_new() -> *mut Document {
    let doc = Document::new();
    Box::into_raw(Box::new(doc))
}

#[no_mangle]
pub extern "C" fn document_insert(
    ptr: *mut Document,
    key: *const c_char,
    item: *const Item,
) -> bool {
    let doc = unsafe { ptr.as_mut().expect("invalid ptr") };
    let key = ffi::char_ptr_to_str(key);
    let item = unsafe { item.as_ref().expect("invalid ptr") };
    doc.insert(&key, item.to_owned()).is_some()
}

#[no_mangle]
pub extern "C" fn document_remove(ptr: *mut Document, key: *const c_char) -> bool {
    let doc = unsafe { ptr.as_mut().expect("invalid ptr") };
    let key = ffi::char_ptr_to_str(key);
    doc.remove(&key).is_some()
}

#[no_mangle]
pub extern "C" fn document_clear(ptr: *mut Document) {
    let doc = unsafe { ptr.as_mut().expect("invalid ptr") };
    doc.clear();
}

#[no_mangle]
pub extern "C" fn document_to_string(ptr: *const Document) -> *const c_char {
    let doc = unsafe { ptr.as_ref().expect("invalid ptr") };
    let str = doc.to_string();
    ffi::str_to_char_ptr(str)
}
