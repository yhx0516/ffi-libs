use funny_utils_rs::ffi;
use std::ffi::c_char;
use toml_edit::{Array, ArrayOfTables, InlineTable, Item, Table, Value};

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
        Some(val) => ffi::str_to_char_ptr(val),
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
