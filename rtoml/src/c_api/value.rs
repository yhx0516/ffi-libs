use funny_utils_rs::ffi;
use std::ffi::c_char;
use toml_edit::{Array, InlineTable, Item, Value};

#[no_mangle]
pub extern "C" fn value_type_name(ptr: *const Value) -> *const c_char {
    let val = unsafe { ptr.as_ref().expect("invalid ptr") };
    let name = val.type_name();
    ffi::str_to_char_ptr(name)
}

#[no_mangle]
pub extern "C" fn value_is_integer(ptr: *const Value) -> bool {
    let val = unsafe { ptr.as_ref().expect("invalid ptr") };
    val.is_integer()
}

#[no_mangle]
pub extern "C" fn value_as_int32(ptr: *const Value) -> i32 {
    let val = unsafe { ptr.as_ref().expect("invalid ptr") };

    match val.as_integer() {
        Some(v) => v as i32,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn value_as_int64(ptr: *const Value) -> i64 {
    let val = unsafe { ptr.as_ref().expect("invalid ptr") };

    match val.as_integer() {
        Some(v) => v,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn value_is_float(ptr: *const Value) -> bool {
    let val = unsafe { ptr.as_ref().expect("invalid ptr") };
    val.is_float()
}

#[no_mangle]
pub extern "C" fn value_as_float(ptr: *const Value) -> f32 {
    let val = unsafe { ptr.as_ref().expect("invalid ptr") };

    match val.as_float() {
        Some(v) => v as f32,
        _ => 0.0,
    }
}

#[no_mangle]
pub extern "C" fn value_as_double(ptr: *const Value) -> f64 {
    let val = unsafe { ptr.as_ref().expect("invalid ptr") };

    match val.as_float() {
        Some(v) => v,
        _ => 0.0,
    }
}

#[no_mangle]
pub extern "C" fn value_is_bool(ptr: *const Value) -> bool {
    let val = unsafe { ptr.as_ref().expect("invalid ptr") };
    val.is_bool()
}

#[no_mangle]
pub extern "C" fn value_as_bool(ptr: *const Value) -> bool {
    let val = unsafe { ptr.as_ref().expect("invalid ptr") };

    match val.as_bool() {
        Some(v) => v,
        _ => false,
    }
}

#[no_mangle]
pub extern "C" fn value_is_str(ptr: *const Value) -> bool {
    let val = unsafe { ptr.as_ref().expect("invalid ptr") };
    val.is_str()
}

#[no_mangle]
pub extern "C" fn value_as_str(ptr: *const Value) -> *const c_char {
    let val = unsafe { ptr.as_ref().expect("invalid ptr") };

    match val.as_str() {
        Some(val) => ffi::str_to_char_ptr(val),
        _ => std::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn value_is_array(ptr: *const Value) -> bool {
    let val = unsafe { ptr.as_ref().expect("invalid ptr") };
    val.is_array()
}

#[no_mangle]
pub extern "C" fn value_as_array(ptr: *mut Value) -> *mut Array {
    let val = unsafe { ptr.as_mut().expect("invalid ptr") };

    match val.as_array_mut() {
        Some(array) => array as *mut _,
        _ => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn value_is_inline_table(ptr: *const Value) -> bool {
    let val = unsafe { ptr.as_ref().expect("invalid ptr") };
    val.is_inline_table()
}

#[no_mangle]
pub extern "C" fn value_as_inline_table(ptr: *mut Value) -> *mut InlineTable {
    let val = unsafe { ptr.as_mut().expect("invalid ptr") };

    match val.as_inline_table_mut() {
        Some(inline_table) => inline_table as *mut _,
        _ => std::ptr::null_mut(),
    }
}

#[no_mangle]
pub extern "C" fn value_dispose(ptr: *mut Value) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn value_from_i32(val: i32) -> *mut Value {
    let val = Value::from(val as i64);
    Box::into_raw(Box::new(val))
}

#[no_mangle]
pub extern "C" fn value_from_i64(val: i64) -> *mut Value {
    let val = Value::from(val);
    Box::into_raw(Box::new(val))
}
#[no_mangle]
pub extern "C" fn value_from_float(val: f32) -> *mut Value {
    let val = Value::from(val as f64);
    Box::into_raw(Box::new(val))
}

#[no_mangle]
pub extern "C" fn value_from_double(val: f64) -> *mut Value {
    let val = Value::from(val);
    Box::into_raw(Box::new(val))
}

#[no_mangle]
pub extern "C" fn value_from_bool(val: bool) -> *mut Value {
    let val = Value::from(val);
    Box::into_raw(Box::new(val))
}

#[no_mangle]
pub extern "C" fn value_from_str(str: *const c_char) -> *mut Value {
    let str = ffi::char_ptr_to_str(str);
    let val = Value::from(str);
    Box::into_raw(Box::new(val))
}

#[no_mangle]
pub extern "C" fn value_from_item(ptr: *const Item) -> *mut Value {
    let item = unsafe { ptr.as_ref().expect("invalid ptr") };
    let val = item.to_owned().into_value().unwrap();
    Box::into_raw(Box::new(val))
}

#[no_mangle]
pub extern "C" fn value_from_inline_table(ptr: *const InlineTable) -> *mut Value {
    let inline_table = unsafe { ptr.as_ref().expect("invalid ptr") };
    let val: Value = Value::from(inline_table.to_owned());
    Box::into_raw(Box::new(val))
}

#[no_mangle]
pub extern "C" fn value_from_array(ptr: *const Array) -> *mut Value {
    let array = unsafe { ptr.as_ref().expect("invalid ptr") };
    let val = Value::from(array.to_owned());
    Box::into_raw(Box::new(val))
}

#[no_mangle]
pub extern "C" fn value_to_string(ptr: *const Value) -> *const c_char {
    let val: &Value = unsafe { ptr.as_ref().expect("invalid ptr") };
    let str = val.to_string();
    ffi::str_to_char_ptr(str)
}
