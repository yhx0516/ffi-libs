use std::{
    ffi::{c_char, CStr, CString},
    fs,
};
use toml_edit::{Array, ArrayOfTables, Document, InlineTable, Item, Table, Value};

// ============================================================
// Document
// ============================================================
#[no_mangle]
pub extern "C" fn parse_toml(path: *const c_char) -> *const Document {
    let path = unsafe {
        assert!(!path.is_null());
        CStr::from_ptr(path).to_str().unwrap()
    };

    let Ok(content) = fs::read_to_string(path) else {
        eprintln!("read file failed: {:?}",path);
        return std::ptr::null();
    };

    match content.parse::<Document>() {
        Ok(val) => Box::into_raw(Box::new(val)),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn get_from_document(ptr: *const Document, key: *const c_char) -> *const Item {
    let doc = unsafe { ptr.as_ref().expect("invalid ptr: ") };

    let key = unsafe {
        assert!(!key.is_null());
        CStr::from_ptr(key).to_str().unwrap()
    };

    match doc.get(key) {
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
        Some(val) => {
            let c_str = CString::new(val).unwrap();
            c_str.into_raw()
        }
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

// ============================================================
// Value
// ============================================================
#[no_mangle]
pub extern "C" fn type_name_from_value(ptr: *const Value) -> *const c_char {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let val = item.type_name();
    let c_str = CString::new(val).unwrap();
    c_str.into_raw()
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
        Some(val) => {
            let c_str = CString::new(val).unwrap();
            c_str.into_raw()
        }
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
pub extern "C" fn contains_key_from_table(ptr: *const Table, key: *const c_char) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let key = unsafe {
        assert!(!key.is_null());
        CStr::from_ptr(key).to_str().unwrap()
    };
    item.contains_key(key)
}

#[no_mangle]
pub extern "C" fn contains_table_from_table(ptr: *const Table, key: *const c_char) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let key = unsafe {
        assert!(!key.is_null());
        CStr::from_ptr(key).to_str().unwrap()
    };
    item.contains_table(key)
}

#[no_mangle]
pub extern "C" fn contains_value_from_table(ptr: *const Table, key: *const c_char) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let key = unsafe {
        assert!(!key.is_null());
        CStr::from_ptr(key).to_str().unwrap()
    };
    item.contains_value(key)
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

// ============================================================
// ArrayOfTables
// ============================================================
#[no_mangle]
pub extern "C" fn is_empty_from_table_array(ptr: *const InlineTable) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.is_empty()
}

#[no_mangle]
pub extern "C" fn len_from_table_array(ptr: *const InlineTable) -> usize {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    item.len()
}

#[no_mangle]
pub extern "C" fn get_from_table_array(
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
pub extern "C" fn contains_key_from_table_array(
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
