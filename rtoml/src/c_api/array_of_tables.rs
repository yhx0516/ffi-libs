use toml_edit::{ArrayOfTables, Table};

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
