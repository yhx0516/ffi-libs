use std::{
    ffi::{c_char, CStr},
    fs,
};
use toml_edit::{Array, ArrayOfTables, Document, InlineTable, Item, Table, Value};

mod matcher;
mod utils;
// ============================================================
// Info
// ============================================================
#[no_mangle]
pub extern "C" fn get_version() -> *const c_char {
    let val = std::env!("CARGO_PKG_VERSION");
    utils::str_to_char_ptr(val)
}

// ============================================================
// PKG Matcher
// ============================================================
#[no_mangle]
pub extern "C" fn pkg_match(
    root_path: *const c_char,
    patterns: *const *const c_char,
    patterns_len: usize,
) -> *const Vec<String> {
    let root_path = utils::char_ptr_to_str(root_path);
    let patterns = utils::arr_ptr_to_strs(patterns, patterns_len as usize);
    let patterns: Vec<&str> = patterns.iter().map(|s| s.as_ref()).collect();

    let files = matcher::pkg_match(root_path, &patterns);
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
        Some(s) => utils::str_to_char_ptr(s),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub fn dispose_strs(ptr: *mut Vec<String>) {
    unsafe { Box::from_raw(ptr) };
}

// ============================================================
// Document
// ============================================================
#[no_mangle]
pub extern "C" fn parse_toml_file(path: *const c_char) -> *const Document {
    let path = utils::char_ptr_to_str(path);

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
pub extern "C" fn parse_toml_str(content: *const c_char) -> *const Document {
    let content = utils::char_ptr_to_str(content);

    match content.parse::<Document>() {
        Ok(val) => Box::into_raw(Box::new(val)),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn get_from_document(ptr: *const Document, key: *const c_char) -> *const Item {
    let doc = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let key = utils::char_ptr_to_str(key);

    match doc.get(&key) {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn as_item_from_document(ptr: *const Document) -> *const Item {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    Box::into_raw(Box::new(item.as_item().to_owned()))
}

#[no_mangle]
pub extern "C" fn as_table_from_document(ptr: *const Document) -> *const Table {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    Box::into_raw(Box::new(item.as_table().to_owned()))
}

#[no_mangle]
pub extern "C" fn dispose_document(ptr: *mut Document) {
    unsafe { Box::from_raw(ptr) };
}

// ============================================================
// Item
// ============================================================
#[no_mangle]
pub extern "C" fn is_value_from_item(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_value()
}

#[no_mangle]
pub extern "C" fn as_value_from_item(ptr: *const Item) -> *const Value {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_value() {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn is_table_from_item(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_table()
}

#[no_mangle]
pub extern "C" fn as_table_from_item(ptr: *const Item) -> *const Table {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_table() {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn is_array_of_tables_from_item(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_array_of_tables()
}

#[no_mangle]
pub extern "C" fn as_array_of_tables_from_item(ptr: *const Item) -> *const ArrayOfTables {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_array_of_tables() {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn is_none_from_item(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_none()
}

#[no_mangle]
pub extern "C" fn is_integer_from_item(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_integer()
}

#[no_mangle]
pub extern "C" fn as_integer_from_item(ptr: *const Item) -> i64 {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_integer() {
        Some(val) => val,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn is_float_from_item(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_float()
}

#[no_mangle]
pub extern "C" fn as_float_from_item(ptr: *const Item) -> f64 {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_float() {
        Some(val) => val,
        _ => 0.0,
    }
}

#[no_mangle]
pub extern "C" fn is_bool_from_item(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_bool()
}

#[no_mangle]
pub extern "C" fn as_bool_from_item(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_bool() {
        Some(val) => val,
        _ => false,
    }
}

#[no_mangle]
pub extern "C" fn is_str_from_item(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_str()
}

#[no_mangle]
pub extern "C" fn as_str_from_item(ptr: *const Item) -> *const c_char {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_str() {
        Some(val) => utils::str_to_char_ptr(val),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn is_array_from_item(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_array()
}

#[no_mangle]
pub extern "C" fn as_array_from_item(ptr: *const Item) -> *const Array {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_array() {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn is_inline_array_from_item(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_inline_table()
}

#[no_mangle]
pub extern "C" fn as_inline_table_from_item(ptr: *const Item) -> *const InlineTable {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_inline_table() {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn dispose_item(ptr: *mut Item) {
    unsafe { Box::from_raw(ptr) };
}

// ============================================================
// Value
// ============================================================
#[no_mangle]
pub extern "C" fn type_name_from_value(ptr: *const Value) -> *const c_char {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let val = item.type_name();
    utils::str_to_char_ptr(val)
}

#[no_mangle]
pub extern "C" fn is_integer_from_value(ptr: *const Value) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_integer()
}

#[no_mangle]
pub extern "C" fn as_integer_from_value(ptr: *const Value) -> i64 {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_integer() {
        Some(val) => val,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn is_float_from_value(ptr: *const Value) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_float()
}

#[no_mangle]
pub extern "C" fn as_float_from_value(ptr: *const Value) -> f64 {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_float() {
        Some(val) => val,
        _ => 0.0,
    }
}

#[no_mangle]
pub extern "C" fn is_bool_from_value(ptr: *const Value) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_bool()
}

#[no_mangle]
pub extern "C" fn as_bool_from_value(ptr: *const Value) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_bool() {
        Some(val) => val,
        _ => false,
    }
}

#[no_mangle]
pub extern "C" fn is_str_from_value(ptr: *const Value) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_str()
}

#[no_mangle]
pub extern "C" fn as_str_from_value(ptr: *const Value) -> *const c_char {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_str() {
        Some(val) => utils::str_to_char_ptr(val),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn is_array_from_value(ptr: *const Value) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_array()
}

#[no_mangle]
pub extern "C" fn as_array_from_value(ptr: *const Value) -> *const Array {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_array() {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn is_inline_array_from_value(ptr: *const Value) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_inline_table()
}

#[no_mangle]
pub extern "C" fn as_inline_table_from_value(ptr: *const Value) -> *const InlineTable {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_inline_table() {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn dispose_value(ptr: *mut Value) {
    unsafe { Box::from_raw(ptr) };
}

// ============================================================
// Array
// ============================================================
#[no_mangle]
pub extern "C" fn is_empty_from_array(ptr: *const Array) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_empty()
}

#[no_mangle]
pub extern "C" fn len_from_array(ptr: *const Array) -> usize {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.len()
}

#[no_mangle]
pub extern "C" fn get_from_array(ptr: *const Array, index: usize) -> *const Value {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.get(index) {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn dispose_array(ptr: *mut Array) {
    unsafe { Box::from_raw(ptr) };
}

// ============================================================
// Table
// ============================================================
#[no_mangle]
pub extern "C" fn is_empty_from_table(ptr: *const Table) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_empty()
}

#[no_mangle]
pub extern "C" fn len_from_table(ptr: *const Table) -> usize {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.len()
}

#[no_mangle]
pub extern "C" fn get_from_table(ptr: *const Table, key: *const c_char) -> *const Item {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let key = utils::char_ptr_to_str(key);

    match item.get(&key) {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn contains_key_from_table(ptr: *const Table, key: *const c_char) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let key = utils::char_ptr_to_str(key);
    item.contains_key(&key)
}

#[no_mangle]
pub extern "C" fn contains_table_from_table(ptr: *const Table, key: *const c_char) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let key = utils::char_ptr_to_str(key);
    item.contains_table(&key)
}

#[no_mangle]
pub extern "C" fn contains_value_from_table(ptr: *const Table, key: *const c_char) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let key = utils::char_ptr_to_str(key);
    item.contains_value(&key)
}

#[no_mangle]
pub extern "C" fn contains_array_of_tables_from_table(
    ptr: *const Table,
    key: *const c_char,
) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let key = unsafe {
        assert!(!key.is_null());
        CStr::from_ptr(key).to_str().unwrap()
    };
    item.contains_array_of_tables(key)
}

#[no_mangle]
pub extern "C" fn dispose_table(ptr: *mut Table) {
    unsafe { Box::from_raw(ptr) };
}

// ============================================================
// InlineTable
// ============================================================
#[no_mangle]
pub extern "C" fn is_empty_from_inline_table(ptr: *const InlineTable) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_empty()
}

#[no_mangle]
pub extern "C" fn len_inline_table(ptr: *const InlineTable) -> usize {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.len()
}

#[no_mangle]
pub extern "C" fn get_from_inline_table(
    ptr: *const InlineTable,
    key: *const c_char,
) -> *const Value {
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
pub extern "C" fn contains_key_from_inline_table(
    ptr: *const InlineTable,
    key: *const c_char,
) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let key = unsafe {
        assert!(!key.is_null());
        CStr::from_ptr(key).to_str().unwrap()
    };
    item.contains_key(key)
}

#[no_mangle]
pub extern "C" fn dispose_inline_table(ptr: *mut InlineTable) {
    unsafe { Box::from_raw(ptr) };
}

// ============================================================
// ArrayOfTables
// ============================================================
#[no_mangle]
pub extern "C" fn is_empty_from_table_array(ptr: *const ArrayOfTables) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_empty()
}

#[no_mangle]
pub extern "C" fn len_from_table_array(ptr: *const ArrayOfTables) -> usize {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.len()
}

#[no_mangle]
pub extern "C" fn get_from_table_array(ptr: *const ArrayOfTables, index: usize) -> *const Table {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.get(index) {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn dispose_table_array(ptr: *mut ArrayOfTables) {
    unsafe { Box::from_raw(ptr) };
}
