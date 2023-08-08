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
pub extern "C" fn array_get(ptr: *mut Array, index: usize) -> *mut Value {
    let array = unsafe { ptr.as_mut().expect("invalid ptr") };

    match array.get_mut(index) {
        Some(val) => val as *mut _,
        _ => std::ptr::null_mut(),
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
    let val = val.to_owned();
    let val = val.decorated(format!("\n{}", " ".repeat(4)), "");
    array.push(val);
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

#[no_mangle]
pub extern "C" fn array_pretty(ptr: *mut Array) {
    let array = unsafe { ptr.as_mut().expect("invalid ptr") };
    array.iter_mut().for_each(|val| {
        val.as_inline_table_mut().and_then(|t| Some(t.fmt()));

        val.decor_mut().set_prefix(format!("\n{}", " ".repeat(4)));
    });
}
