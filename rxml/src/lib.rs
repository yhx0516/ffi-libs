mod c_api;
mod core;

pub use c_api::*;
pub use funny_utils_rs::ffi::c_api::*;

pub use self::core::Document;
pub use self::core::Element;
