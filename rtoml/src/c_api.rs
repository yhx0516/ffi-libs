mod array;
mod array_of_tables;
mod document;
mod inline_table;
mod item;
mod table;
mod value;

pub use array::*;
pub use array_of_tables::*;
pub use document::*;
pub use inline_table::*;
pub use item::*;
pub use table::*;
pub use value::*;

#[no_mangle]
pub extern "C" fn get_version() -> *const std::ffi::c_char {
    let version = format!(
        "{} {}",
        std::env!("CARGO_PKG_NAME"),
        std::env!("CARGO_PKG_VERSION")
    );
    funny_utils_rs::ffi::str_to_char_ptr(&version)
}
