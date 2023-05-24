use chrono::Local;
use handlebars::Handlebars;
use std::path::Path;

pub fn render_template_from_toml(
    tpl_path: impl AsRef<Path>,
    toml_path: impl AsRef<Path>,
) -> String {
    let toml_str = std::fs::read_to_string(toml_path).expect("Failed to read file");
    let toml_value: toml::Value = toml::from_str(&toml_str).expect("Failed to parse TOML");

    let mut json_value = serde_json::to_value(toml_value).expect("Failed to convert to JSON");
    if let Some(json_obj) = json_value.as_object_mut() {
        let local_time = Local::now().format("%m-%d:%H:%M:%S").to_string();
        json_obj.insert(String::from("$date"), serde_json::Value::String(local_time));
    }

    let handlebars = Handlebars::new();
    let tpl_str = std::fs::read_to_string(tpl_path).expect("Failed to read file");
    let res = handlebars.render_template(&tpl_str, &json_value);
    match res {
        Ok(r) => r,
        _ => String::new(),
    }
}

// ============================================================
// Test
// ============================================================
#[cfg(test)]
mod tests {
    use super::render_template_from_toml;
    use handlebars::{to_json, Context, Handlebars, Helper, Output, RenderContext, RenderError};

    #[test]
    fn block_helper_test() {
        // create the handlebars registry
        let mut handlebars = Handlebars::new();
        let tpl_str =
            std::fs::read_to_string("../tests/handlebars-tpl/block_helper_template.hbs").unwrap();

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

    #[test]
    fn render_template_from_toml_test() {
        let toml_path = "../tests/handlebars-tpl/app_android_dev.toml";
        let tpl_path = "../tests/handlebars-tpl/app_android_dev_template.toml";
        let out = render_template_from_toml(tpl_path, toml_path);

        println!("{}", out);
    }
}
