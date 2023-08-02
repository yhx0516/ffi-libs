use funny_utils_rs::ffi;
use std::ffi::c_char;
use toml_edit::{Array, InlineTable, Value};
// ============================================================
// Value
// ============================================================
#[no_mangle]
pub extern "C" fn value_type_name(ptr: *const Value) -> *const c_char {
    let item = unsafe { ptr.as_ref().expect("invalid ptr: ") };
    let val = item.type_name();
    ffi::str_to_char_ptr(val)
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
        Some(val) => ffi::str_to_char_ptr(val),
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
