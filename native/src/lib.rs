extern crate color_thief;
extern crate image;
#[macro_use]
extern crate neon;

use std::path;
use color_thief::{Color, ColorFormat};
use neon::mem::Handle;
use neon::vm::{Call, JsResult};
use neon::js::{JsFunction, JsNull, JsString, JsUndefined};

fn find_color(t: image::ColorType) -> ColorFormat {
    match t {
        image::ColorType::RGB(8) => ColorFormat::Rgb,
        image::ColorType::RGBA(8) => ColorFormat::Rgba,
        _ => unreachable!(),
    }
}

fn get_dominant_color(call: Call) -> JsResult<JsUndefined> {
    let f = call.arguments
        .require(call.scope, 0)?
        .check::<JsString>()?
        .value();

    let img = image::open(&path::Path::new(&f)).unwrap();
    let color_type = find_color(img.color());
    let colors = color_thief::get_palette(&img.raw_pixels(), color_type, 10, 10).unwrap();
    let Color {
        r: _r,
        g: _g,
        b: _b,
    } = colors[0];

    let mut rr: String = "".to_owned();
    rr.push_str("#");
    rr.push_str(&format!("{:x}", _r));
    rr.push_str(&format!("{:x}", _g));
    rr.push_str(&format!("{:x}", _b));

    let callback = call.arguments.get(call.scope, 1).unwrap();
    let file_name = JsString::new(call.scope, &rr).unwrap();

    if let Some(function) = callback.downcast::<JsFunction>() {
        let args: Vec<Handle<JsString>> = vec![file_name];
        let _ = function.call(call.scope, JsNull::new(), args);
    }
    Ok(JsUndefined::new())
}

register_module!(m, { m.export("getDominantColor", get_dominant_color) });
