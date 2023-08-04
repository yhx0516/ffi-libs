use std::ffi::c_char;

use funny_utils_rs::ffi;

#[no_mangle]
pub extern "C" fn get_version() -> *const c_char {
    let version = format!(
        "{} {}",
        std::env!("CARGO_PKG_NAME"),
        std::env!("CARGO_PKG_VERSION")
    );
    ffi::str_to_char_ptr(&version)
}

// ============================================================
// ErrorBuffer 便于外部接入时调试
// ============================================================
use lazy_static::lazy_static;
use std::sync::Mutex;

use crate::{Document, Element};

lazy_static! {
    static ref ERR_BUFF_INS: Mutex<ErrorBuffer> = Mutex::new(ErrorBuffer::default());
}

#[derive(Debug, Default)]
struct ErrorBuffer {
    buffer: String,
}

impl ErrorBuffer {
    pub fn new(str: impl AsRef<str>) {
        let mut ins = ERR_BUFF_INS.lock().unwrap();
        ins.set_buffer(str);
    }

    pub fn output() -> String {
        let ins = ERR_BUFF_INS.lock().unwrap();
        let buffer = ins.get_buffer();
        match buffer.is_empty() {
            true => String::from("no error in rpkg"),
            false => buffer.to_owned(),
        }
    }

    pub fn set_buffer(&mut self, buffer: impl AsRef<str>) {
        self.buffer = buffer.as_ref().to_string();
    }

    fn get_buffer(&self) -> &String {
        &self.buffer
    }
}

#[no_mangle]
pub extern "C" fn try_get_err() -> *const c_char {
    let info = ErrorBuffer::output();
    ffi::str_to_char_ptr(info)
}

// ============================================================
// Document
// ============================================================
#[no_mangle]
pub extern "C" fn document_parse_file(path: *const c_char) -> *mut Document {
    let path = ffi::char_ptr_to_str(path);

    match Document::parse_file(path) {
        Ok(doc) => Box::into_raw(Box::new(doc)),
        Err(e) => {
            ErrorBuffer::new(e.to_string());
            return std::ptr::null_mut();
        }
    }
}

#[no_mangle]
pub extern "C" fn document_parse_content(content: *const c_char) -> *mut Document {
    let content = ffi::char_ptr_to_str(content);

    match Document::parse(content) {
        Ok(doc) => Box::into_raw(Box::new(doc)),
        Err(e) => {
            ErrorBuffer::new(e.to_string());
            return std::ptr::null_mut();
        }
    }
}

#[no_mangle]
pub extern "C" fn document_dispose(ptr: *mut Document) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn document_get_version(ptr: *const Document) -> *const c_char {
    let doc = unsafe { ptr.as_ref().expect("invalid ptr") };
    ffi::str_to_char_ptr(&doc.version)
}

#[no_mangle]
pub extern "C" fn document_get_encoding(ptr: *const Document) -> *const c_char {
    let doc = unsafe { ptr.as_ref().expect("invalid ptr") };
    ffi::str_to_char_ptr(&doc.encoding)
}

#[no_mangle]
pub extern "C" fn document_get_children_len(ptr: *const Document) -> u32 {
    let doc = unsafe { ptr.as_ref().expect("invalid ptr") };
    doc.children.len() as u32
}

#[no_mangle]
pub extern "C" fn document_get_child(ptr: *mut Document, index: u32) -> *mut Element {
    let doc = unsafe { ptr.as_mut().expect("invalid ptr") };

    match doc.children.get_mut(index as usize) {
        Some(element) => element as *mut _,
        None => {
            let len = doc.children.len();
            let msg = format!("index({}) out of bounds, array len({})", index, len);
            ErrorBuffer::new(msg);
            return std::ptr::null_mut();
        }
    }
}

#[no_mangle]
pub extern "C" fn document_tree_text(ptr: *const Document) -> *const c_char {
    let doc = unsafe { ptr.as_ref().expect("invalid ptr") };
    ffi::str_to_char_ptr(&doc.to_string())
}

// ============================================================
// Element
// ============================================================
#[no_mangle]
pub extern "C" fn element_dispose(ptr: *mut Document) {
    if ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(ptr) };
}

#[no_mangle]
pub extern "C" fn element_get_name(ptr: *const Element) -> *const c_char {
    let element = unsafe { ptr.as_ref().expect("invalid ptr") };
    ffi::str_to_char_ptr(element.get_name())
}

#[no_mangle]
pub extern "C" fn element_get_text(ptr: *const Element) -> *const c_char {
    let element = unsafe { ptr.as_ref().expect("invalid ptr") };
    ffi::str_to_char_ptr(element.get_text())
}

#[no_mangle]
pub extern "C" fn element_get_attribute_keys(ptr: *const Element) -> *const Vec<String> {
    let element = unsafe { ptr.as_ref().expect("invalid ptr") };
    let keys = element.get_attribute_keys();
    Box::into_raw(Box::new(keys))
}

#[no_mangle]
pub extern "C" fn element_get_attribute_value(
    ptr: *const Element,
    key: *const c_char,
) -> *const c_char {
    let element = unsafe { ptr.as_ref().expect("invalid ptr") };
    let key = ffi::char_ptr_to_str(key);

    match element.get_attribute_value(&key) {
        Some(val) => ffi::str_to_char_ptr(val),
        None => {
            ErrorBuffer::new(format!("not found key {}", key));
            return std::ptr::null();
        }
    }
}

#[no_mangle]
pub extern "C" fn element_get_children_len(ptr: *const Element) -> u32 {
    let element = unsafe { ptr.as_ref().expect("invalid ptr") };
    element.get_children_len() as u32
}

#[no_mangle]
pub extern "C" fn element_get_child(ptr: *mut Element, index: u32) -> *mut Element {
    let element = unsafe { ptr.as_mut().expect("invalid ptr") };

    match element.get_child_mut(index as usize) {
        Some(element) => element as *mut _,
        None => {
            let len = element.get_children_len();
            let msg = format!("index({}) out of bounds, array len({})", index, len);
            ErrorBuffer::new(msg);
            return std::ptr::null_mut();
        }
    }
}
