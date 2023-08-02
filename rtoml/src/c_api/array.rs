use toml_edit::{Array, Value};

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
