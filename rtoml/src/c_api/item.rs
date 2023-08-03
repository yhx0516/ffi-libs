use funny_utils_rs::ffi;
use std::ffi::c_char;
use toml_edit::{Array, ArrayOfTables, InlineTable, Item, Table, Value};

#[no_mangle]
pub extern "C" fn item_is_value(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr") };
    item.is_value()
}

#[no_mangle]
pub extern "C" fn item_as_value(ptr: *mut Item) -> *mut Value {
    let item = unsafe { ptr.as_mut().expect("invalid ptr") };

    match item.as_value_mut() {
        Some(val) => val as *mut _,
        _ => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn item_is_table(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr") };
    item.is_table()
}

#[no_mangle]
pub extern "C" fn item_as_table(ptr: *mut Item) -> *mut Table {
    let item = unsafe { ptr.as_mut().expect("invalid ptr") };

    match item.as_table_mut() {
        Some(table) => table as *mut _,
        _ => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn item_is_array_of_tables(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr") };
    item.is_array_of_tables()
}

#[no_mangle]
pub extern "C" fn item_as_array_of_tables(ptr: *mut Item) -> *mut ArrayOfTables {
    let item = unsafe { ptr.as_mut().expect("invalid ptr") };

    match item.as_array_of_tables_mut() {
        Some(table_array) => table_array as *mut _,
        _ => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn item_is_none(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr") };
    item.is_none()
}

#[no_mangle]
pub extern "C" fn item_is_integer(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr") };
    item.is_integer()
}

#[no_mangle]
pub extern "C" fn item_as_int32(ptr: *const Item) -> i32 {
    let item = unsafe { ptr.as_ref().expect("invalid ptr") };

    match item.as_integer() {
        Some(val) => val as i32,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn item_as_int64(ptr: *const Item) -> i64 {
    let item = unsafe { ptr.as_ref().expect("invalid ptr") };

    match item.as_integer() {
        Some(val) => val,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn item_is_float(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr") };
    item.is_float()
}

#[no_mangle]
pub extern "C" fn item_as_float(ptr: *const Item) -> f32 {
    let item = unsafe { ptr.as_ref().expect("invalid ptr") };

    match item.as_float() {
        Some(val) => val as f32,
        _ => 0.0,
    }
}

#[no_mangle]
pub extern "C" fn item_as_double(ptr: *const Item) -> f64 {
    let item = unsafe { ptr.as_ref().expect("invalid ptr") };

    match item.as_float() {
        Some(val) => val,
        _ => 0.0,
    }
}

#[no_mangle]
pub extern "C" fn item_is_bool(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr") };
    item.is_bool()
}

#[no_mangle]
pub extern "C" fn item_as_bool(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr") };

    match item.as_bool() {
        Some(val) => val,
        _ => false,
    }
}

#[no_mangle]
pub extern "C" fn item_is_str(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr") };
    item.is_str()
}

#[no_mangle]
pub extern "C" fn item_as_str(ptr: *const Item) -> *const c_char {
    let item = unsafe { ptr.as_ref().expect("invalid ptr") };

    match item.as_str() {
        Some(val) => ffi::str_to_char_ptr(val),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn item_is_array(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr") };
    item.is_array()
}

#[no_mangle]
pub extern "C" fn item_as_array(ptr: *mut Item) -> *mut Array {
    let item = unsafe { ptr.as_mut().expect("invalid ptr") };

    match item.as_array_mut() {
        Some(array) => array as *mut _,
        _ => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn item_is_inline_table(ptr: *const Item) -> bool {
    let item = unsafe { ptr.as_ref().expect("invalid ptr") };
    item.is_inline_table()
}

#[no_mangle]
pub extern "C" fn item_as_inline_table(ptr: *mut Item) -> *const InlineTable {
    let item = unsafe { ptr.as_mut().expect("invalid ptr") };

    match item.as_inline_table_mut() {
        Some(inline_table) => inline_table as *mut _,
        _ => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn item_dispose(ptr: *mut Item) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn item_from_i32(val: i32) -> *mut Item {
    let item = toml_edit::value(val as i64);
    Box::into_raw(Box::new(item))
}

#[no_mangle]
pub extern "C" fn item_from_i64(val: i64) -> *mut Item {
    let item = toml_edit::value(val);
    Box::into_raw(Box::new(item))
}

#[no_mangle]
pub extern "C" fn item_from_float(val: f32) -> *mut Item {
    let item = toml_edit::value(val as f64);
    Box::into_raw(Box::new(item))
}

#[no_mangle]
pub extern "C" fn item_from_double(val: f64) -> *mut Item {
    let item = toml_edit::value(val);
    Box::into_raw(Box::new(item))
}

#[no_mangle]
pub extern "C" fn item_from_bool(val: bool) -> *mut Item {
    let item = toml_edit::value(val);
    Box::into_raw(Box::new(item))
}

#[no_mangle]
pub extern "C" fn item_from_str(str: *const c_char) -> *mut Item {
    let str = ffi::char_ptr_to_str(str);
    let item = toml_edit::value(str);
    Box::into_raw(Box::new(item))
}

#[no_mangle]
pub extern "C" fn item_from_value(ptr: *const Value) -> *mut Item {
    let val = unsafe { ptr.as_ref().expect("invalid ptr") };
    let item = toml_edit::value(val.to_owned());
    Box::into_raw(Box::new(item))
}

#[no_mangle]
pub extern "C" fn item_from_inline_table(ptr: *const InlineTable) -> *mut Item {
    let inline_table = unsafe { ptr.as_ref().expect("invalid ptr") };
    let item = toml_edit::value(inline_table.to_owned());
    Box::into_raw(Box::new(item))
}

#[no_mangle]
pub extern "C" fn item_from_table(ptr: *const Table) -> *mut Item {
    let table = unsafe { ptr.as_ref().expect("invalid ptr") };
    let item = toml_edit::Item::Table(table.to_owned());
    Box::into_raw(Box::new(item))
}

#[no_mangle]
pub extern "C" fn item_from_array(ptr: *const Array) -> *mut Item {
    let array = unsafe { ptr.as_ref().expect("invalid ptr") };
    let item = toml_edit::value(array.to_owned());
    Box::into_raw(Box::new(item))
}

#[no_mangle]
pub extern "C" fn item_from_table_array(ptr: *const ArrayOfTables) -> *mut Item {
    let table_array = unsafe { ptr.as_ref().expect("invalid ptr") };
    let item = toml_edit::Item::ArrayOfTables(table_array.to_owned());
    Box::into_raw(Box::new(item))
}

#[no_mangle]
pub extern "C" fn item_to_string(ptr: *const Item) -> *const c_char {
    let item = unsafe { ptr.as_ref().expect("invalid ptr") };
    let str = item.to_string();
    ffi::str_to_char_ptr(str)
}
