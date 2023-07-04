mod ffi;
mod path;

pub use ffi::arr_ptr_to_strs;
pub use ffi::char_ptr_to_str;
pub use ffi::str_dispose;
pub use ffi::str_to_char_ptr;
pub use ffi::strs_dispose;
pub use ffi::strs_get;
pub use ffi::strs_len;

pub use path::canonicalize_path;
pub use path::norm_path;
