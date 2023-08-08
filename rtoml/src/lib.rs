// NOTE: 作为第三方库可以直接调用
pub use funny_utils_rs::ffi;
pub use funny_utils_rs::ffi::c_api::*;

mod c_api;
pub use c_api::*;
