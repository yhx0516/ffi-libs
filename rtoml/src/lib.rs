use std::ffi::c_char;

// NOTE: 作为第三方库可以直接调用
pub use funny_utils_rs::ffi;
pub use funny_utils_rs::ffi::c_api::*;

mod c_api;
pub use c_api::*;

#[no_mangle]
pub extern "C" fn get_version() -> *const c_char {
    let version = format!(
        "{} {}",
        std::env!("CARGO_PKG_NAME"),
        std::env!("CARGO_PKG_VERSION")
    );
    ffi::str_to_char_ptr(&version)
}
