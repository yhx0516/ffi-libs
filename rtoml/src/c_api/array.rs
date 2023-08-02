use funny_utils_rs::ffi;
use std::ffi::c_char;
use toml_edit::{Array, Value};

#[no_mangle]
pub extern "C" fn array_is_empty(ptr: *const Array) -> bool {
    let array = unsafe { ptr.as_ref().expect("invalid ptr") };
    array.is_empty()
}

#[no_mangle]
pub extern "C" fn array_len(ptr: *const Array) -> usize {
    let array = unsafe { ptr.as_ref().expect("invalid ptr") };
    array.len()
}

#[no_mangle]
pub extern "C" fn array_get(ptr: *const Array, index: usize) -> *const Value {
    let array = unsafe { ptr.as_ref().expect("invalid ptr") };

    match array.get(index) {
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

#[no_mangle]
pub extern "C" fn array_new() -> *mut Array {
    let array = Array::new();
    Box::into_raw(Box::new(array))
}

#[no_mangle]
pub extern "C" fn array_push(ptr: *mut Array, val: *const Value) {
    let array = unsafe { ptr.as_mut().expect("invalid ptr") };
    let val = unsafe { val.as_ref().expect("invalid ptr") };
    array.push(val.to_owned());
}

#[no_mangle]
pub extern "C" fn array_insert(ptr: *mut Array, index: usize, val: *const Value) {
    let array = unsafe { ptr.as_mut().expect("invalid ptr") };
    let val = unsafe { val.as_ref().expect("invalid ptr") };
    array.insert(index, val.to_owned());
}

#[no_mangle]
pub extern "C" fn array_replace(ptr: *mut Array, index: usize, val: *const Value) {
    let array = unsafe { ptr.as_mut().expect("invalid ptr") };
    let val = unsafe { val.as_ref().expect("invalid ptr") };
    array.replace(index, val.to_owned());
}

#[no_mangle]
pub extern "C" fn array_remove(ptr: *mut Array, index: usize) {
    let array = unsafe { ptr.as_mut().expect("invalid ptr") };
    array.remove(index);
}

#[no_mangle]
pub extern "C" fn array_clear(ptr: *mut Array) {
    let array = unsafe { ptr.as_mut().expect("invalid ptr") };
    array.clear();
}

#[no_mangle]
pub extern "C" fn array_to_string(ptr: *const Array) -> *const c_char {
    let array = unsafe { ptr.as_ref().expect("invalid ptr") };
    let str = array.to_string();
    ffi::str_to_char_ptr(str)
}
