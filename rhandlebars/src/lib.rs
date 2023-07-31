use funny_utils_rs::ffi;
use handlebars::{to_json, Context, Handlebars, Helper, Output, RenderContext};
use std::ffi::c_char;

mod render;

// ============================================================
// Info
// ============================================================
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
// Handlebars Api
// ============================================================
#[no_mangle]
pub extern "C" fn handlebars_new() -> *mut Handlebars<'static> {
    let handlebars = Handlebars::new();
    Box::into_raw(Box::new(handlebars))
}

#[no_mangle]
pub extern "C" fn handlebars_dispose(handlebars_ptr: *mut Handlebars) {
    if handlebars_ptr.is_null() {
        return;
    }
    unsafe { Box::from_raw(handlebars_ptr) };
}

#[no_mangle]
pub extern "C" fn handlebars_register_helper_callback(
    handlebars_ptr: *mut Handlebars,
    helper_name: *const c_char,
    helper_callback: Option<extern "C" fn(*const Helper) -> *const c_char>,
) {
    // 将传入的 helper_callback 封装为 Rust 的闭包，并注册为 Handlebars 的 helper
    unsafe {
        let handlebars = handlebars_ptr.as_mut().expect("invalid ptr: ");
        let helper_name = ffi::char_ptr_to_str(helper_name);

        if let Some(callback) = helper_callback {
            let func = move |h: &Helper,
                             _: &Handlebars,
                             _: &Context,
                             _: &mut RenderContext,
                             out: &mut dyn Output| {
                let out_str = ffi::char_ptr_to_str(callback(h as *const Helper));
                out.write(&out_str)?;
                Ok(())
            };

            handlebars.register_helper(&helper_name, Box::new(func));
        }
    }
}

#[no_mangle]
pub extern "C" fn handlebars_render_template(
    handlebars_ptr: *mut Handlebars,
    tpl_str: *const c_char,
) -> *const c_char {
    let handlebars = unsafe { handlebars_ptr.as_mut().expect("invalid ptr: ") };
    let tpl_str = ffi::char_ptr_to_str(tpl_str);

    match handlebars.render_template(&tpl_str, &to_json("")) {
        Ok(r) => ffi::str_to_char_ptr(&r),
        _ => ffi::str_to_char_ptr(""),
    }
}

#[no_mangle]
pub extern "C" fn helper_get_arg_as_str(helper_ptr: *const Helper, idx: usize) -> *const c_char {
    let helper = unsafe { helper_ptr.as_ref().expect("invalid ptr: ") };
    let res = helper
        .param(idx)
        .and_then(|v| v.value().as_str())
        .unwrap_or("");

    ffi::str_to_char_ptr(res)
}

#[no_mangle]
pub extern "C" fn render_template_from_toml(
    tpl_path: *const c_char,
    toml_path: *const c_char,
) -> *const c_char {
    let tpl_path = ffi::char_ptr_to_str(tpl_path);
    let toml_path = ffi::char_ptr_to_str(toml_path);

    let res = render::render_template_from_toml(tpl_path, toml_path);
    ffi::str_to_char_ptr(res)
}
