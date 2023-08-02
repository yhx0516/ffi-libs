use funny_utils_rs::ffi;
use std::ffi::c_char;
use toml_edit::{Array, ArrayOfTables, Table};

#[no_mangle]
pub extern "C" fn table_array_is_empty(ptr: *const ArrayOfTables) -> bool {
    let table_array = unsafe { ptr.as_ref().expect("invalid ptr") };
    table_array.is_empty()
}

#[no_mangle]
pub extern "C" fn table_array_len(ptr: *const ArrayOfTables) -> usize {
    let table_array = unsafe { ptr.as_ref().expect("invalid ptr") };
    table_array.len()
}

#[no_mangle]
pub extern "C" fn table_array_get(ptr: *const ArrayOfTables, index: usize) -> *const Table {
    let table_array = unsafe { ptr.as_ref().expect("invalid ptr") };

    match table_array.get(index) {
        Some(table) => Box::into_raw(Box::new(table.to_owned())),
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

#[no_mangle]
pub extern "C" fn table_array_new() -> *mut ArrayOfTables {
    let table_array = ArrayOfTables::new();
    Box::into_raw(Box::new(table_array))
}

#[no_mangle]
pub extern "C" fn table_array_push(ptr: *mut ArrayOfTables, table: *const Table) {
    let table_array = unsafe { ptr.as_mut().expect("invalid ptr") };
    let table = unsafe { table.as_ref().expect("invalid ptr") };
    table_array.push(table.to_owned());
}

#[no_mangle]
pub extern "C" fn table_array_remove(ptr: *mut ArrayOfTables, index: usize) {
    let table_array = unsafe { ptr.as_mut().expect("invalid ptr") };
    table_array.remove(index);
}

#[no_mangle]
pub extern "C" fn table_array_clear(ptr: *mut ArrayOfTables) {
    let table_array = unsafe { ptr.as_mut().expect("invalid ptr") };
    table_array.clear();
}

#[no_mangle]
pub extern "C" fn table_array_to_array(ptr: *const ArrayOfTables) -> *const Array {
    let table_array = unsafe { ptr.as_ref().expect("invalid ptr") };
    let array = table_array.to_owned().into_array();
    Box::into_raw(Box::new(array))
}

#[no_mangle]
pub extern "C" fn table_array_to_string(ptr: *const ArrayOfTables) -> *const c_char {
    let table_array = unsafe { ptr.as_ref().expect("invalid ptr") };
    let str = table_array.to_string();
    ffi::str_to_char_ptr(str)
}
