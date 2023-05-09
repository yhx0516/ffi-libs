use handlebars::{to_json, Context, Handlebars, Helper, Output, RenderContext};
use std::ffi::c_char;

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
    rutils::str_to_char_ptr(&version)
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
        let helper_name = rutils::char_ptr_to_str(helper_name);

        if let Some(callback) = helper_callback {
            let func = move |h: &Helper,
                             _: &Handlebars,
                             _: &Context,
                             _: &mut RenderContext,
                             out: &mut dyn Output| {
                let out_str = rutils::char_ptr_to_str(callback(h as *const Helper));
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
    let tpl_str = rutils::char_ptr_to_str(tpl_str);

    match handlebars.render_template(&tpl_str, &to_json("")) {
        Ok(r) => rutils::str_to_char_ptr(&r),
        _ => rutils::str_to_char_ptr(""),
    }
}

#[no_mangle]
pub extern "C" fn helper_get_arg_as_str(helper_ptr: *const Helper, idx: usize) -> *const c_char {
    let helper = unsafe { helper_ptr.as_ref().expect("invalid ptr: ") };
    let res = helper
        .param(idx)
        .and_then(|v| v.value().as_str())
        .unwrap_or("");

    rutils::str_to_char_ptr(res)
}

// ============================================================
// Test
// ============================================================
#[cfg(test)]
mod tests {
    use handlebars::{to_json, Context, Handlebars, Helper, Output, RenderContext, RenderError};

    #[test]
    fn block_helper_test() {
        // create the handlebars registry
        let mut handlebars = Handlebars::new();
        let tpl_str = std::fs::read_to_string("../tests/app-settings.hbs").unwrap();

        // register some custom helpers
        handlebars.register_helper("set_time", Box::new(set_time_helper));
        handlebars.register_helper("set_value", Box::new(set_value_helper));

        let out = handlebars.render_template(&tpl_str, &to_json(""));
        let expect = r#"
version = 1

[app]
displayVersion = 2023-05-08:16:55:00
frameRate = app|frameRate
consolePort = console|port
assetServer = download|server

[screen]
windowed = screen|windowed
width = screen|width
height = screen|height

[server]
relay = server|relay
port = server|port
etcdIp = server|etcdIp
etcdPort = server|etcdPort
autoStart = server|autoStart
        "#;
        assert_eq!(expect.trim().replace("\n", "\r\n"), out.unwrap().trim());
    }

    fn set_time_helper(
        _h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> Result<(), RenderError> {
        out.write("2023-05-08:16:55:00")?;
        Ok(())
    }

    fn set_value_helper(
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> Result<(), RenderError> {
        let desc = h
            .param(0)
            .and_then(|v| v.value().as_str())
            .ok_or(RenderError::new(
                "Param 0 with str type is required for set_value helper.",
            ))?;

        out.write(desc)?;
        Ok(())
    }
}
