use std::{
    ffi::{c_char, CStr},
    fs,
};
use toml_edit::{Array, ArrayOfTables, Document, InlineTable, Item, Table, Value};

// NOTE: 作为第三方库可以直接调用
pub use rutils::{str_dispose, strs_dispose, strs_get, strs_len};

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
// Document
// ============================================================
#[no_mangle]
pub extern "C" fn document_parse_file(path: *const c_char) -> *const Document {
    let path = rutils::char_ptr_to_str(path);

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
    let content = rutils::char_ptr_to_str(content);

    match content.parse::<Document>() {
        Ok(val) => Box::into_raw(Box::new(val)),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn document_get(ptr: *const Document, key: *const c_char) -> *const Item {
    let doc = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let key = rutils::char_ptr_to_str(key);

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

// ============================================================
// Item
// ============================================================
#[no_mangle]
pub extern "C" fn item_is_value(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_value()
}

#[no_mangle]
pub extern "C" fn item_as_value(ptr: *const Item) -> *const Value {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_value() {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn item_is_table(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_table()
}

#[no_mangle]
pub extern "C" fn item_as_table(ptr: *const Item) -> *const Table {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_table() {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn item_is_array_of_tables(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_array_of_tables()
}

#[no_mangle]
pub extern "C" fn item_as_array_of_tables(ptr: *const Item) -> *const ArrayOfTables {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_array_of_tables() {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn item_is_none(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_none()
}

#[no_mangle]
pub extern "C" fn item_is_integer(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_integer()
}

#[no_mangle]
pub extern "C" fn item_as_int32(ptr: *const Item) -> i32 {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_integer() {
        Some(val) => val as i32,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn item_as_int64(ptr: *const Item) -> i64 {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_integer() {
        Some(val) => val,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn item_is_float(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_float()
}

#[no_mangle]
pub extern "C" fn item_as_float(ptr: *const Item) -> f32 {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_float() {
        Some(val) => val as f32,
        _ => 0.0,
    }
}

#[no_mangle]
pub extern "C" fn item_as_double(ptr: *const Item) -> f64 {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_float() {
        Some(val) => val,
        _ => 0.0,
    }
}

#[no_mangle]
pub extern "C" fn item_is_bool(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_bool()
}

#[no_mangle]
pub extern "C" fn item_as_bool(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_bool() {
        Some(val) => val,
        _ => false,
    }
}

#[no_mangle]
pub extern "C" fn item_is_str(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_str()
}

#[no_mangle]
pub extern "C" fn item_as_str(ptr: *const Item) -> *const c_char {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_str() {
        Some(val) => rutils::str_to_char_ptr(val),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn item_is_array(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_array()
}

#[no_mangle]
pub extern "C" fn item_as_array(ptr: *const Item) -> *const Array {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_array() {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn item_is_inline_table(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_inline_table()
}

#[no_mangle]
pub extern "C" fn item_as_inline_table(ptr: *const Item) -> *const InlineTable {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_inline_table() {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn item_dispose(ptr: *mut Item) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}

// ============================================================
// Value
// ============================================================
#[no_mangle]
pub extern "C" fn value_type_name(ptr: *const Value) -> *const c_char {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let val = item.type_name();
    rutils::str_to_char_ptr(val)
}

#[no_mangle]
pub extern "C" fn value_is_integer(ptr: *const Value) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_integer()
}

#[no_mangle]
pub extern "C" fn value_as_int32(ptr: *const Value) -> i32 {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_integer() {
        Some(val) => val as i32,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn value_as_int64(ptr: *const Value) -> i64 {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_integer() {
        Some(val) => val,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn value_is_float(ptr: *const Value) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_float()
}

#[no_mangle]
pub extern "C" fn value_as_float(ptr: *const Value) -> f32 {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_float() {
        Some(val) => val as f32,
        _ => 0.0,
    }
}

#[no_mangle]
pub extern "C" fn value_as_double(ptr: *const Value) -> f64 {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_float() {
        Some(val) => val,
        _ => 0.0,
    }
}

#[no_mangle]
pub extern "C" fn value_is_bool(ptr: *const Value) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_bool()
}

#[no_mangle]
pub extern "C" fn value_as_bool(ptr: *const Value) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_bool() {
        Some(val) => val,
        _ => false,
    }
}

#[no_mangle]
pub extern "C" fn value_is_str(ptr: *const Value) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_str()
}

#[no_mangle]
pub extern "C" fn value_as_str(ptr: *const Value) -> *const c_char {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_str() {
        Some(val) => rutils::str_to_char_ptr(val),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn value_is_array(ptr: *const Value) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_array()
}

#[no_mangle]
pub extern "C" fn value_as_array(ptr: *const Value) -> *const Array {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_array() {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn value_is_inline_table(ptr: *const Value) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_inline_table()
}

#[no_mangle]
pub extern "C" fn value_as_inline_table(ptr: *const Value) -> *const InlineTable {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.as_inline_table() {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn value_dispose(ptr: *mut Value) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}

// ============================================================
// Array
// ============================================================
#[no_mangle]
pub extern "C" fn array_is_empty(ptr: *const Array) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_empty()
}

#[no_mangle]
pub extern "C" fn array_len(ptr: *const Array) -> usize {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.len()
}

#[no_mangle]
pub extern "C" fn array_get(ptr: *const Array, index: usize) -> *const Value {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.get(index) {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn array_dispose(ptr: *mut Array) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}

// ============================================================
// Table
// ============================================================
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
    let key = rutils::char_ptr_to_str(key);

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
    let key = rutils::char_ptr_to_str(key);
    item.contains_key(&key)
}

#[no_mangle]
pub extern "C" fn table_contains_table(ptr: *const Table, key: *const c_char) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let key = rutils::char_ptr_to_str(key);
    item.contains_table(&key)
}

#[no_mangle]
pub extern "C" fn table_contains_value(ptr: *const Table, key: *const c_char) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let key = rutils::char_ptr_to_str(key);
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

// ============================================================
// InlineTable
// ============================================================
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

// ============================================================
// ArrayOfTables
// ============================================================
#[no_mangle]
pub extern "C" fn table_array_is_empty(ptr: *const ArrayOfTables) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_empty()
}

#[no_mangle]
pub extern "C" fn table_array_len(ptr: *const ArrayOfTables) -> usize {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.len()
}

#[no_mangle]
pub extern "C" fn table_array_get(ptr: *const ArrayOfTables, index: usize) -> *const Table {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    match item.get(index) {
        Some(val) => Box::into_raw(Box::new(val.to_owned())),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn table_array_dispose(ptr: *mut ArrayOfTables) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}
