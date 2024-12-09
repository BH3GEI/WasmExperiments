// 引入相关的依赖
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use js_sys::Math;

// 给js调用的方法
#[wasm_bindgen]
pub fn draw_circles(canvas: HtmlCanvasElement) {
    // 获取ctx绘画上下文
    let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
    let width = canvas.client_width() as f64;
    let height = canvas.client_height() as f64;

    // 循环绘制
    for _ in 0..100_000 {
        // 设置一下写x，y的位置
        let x = Math::random() * width;
        let y = Math::random() * height;
        let radius = 10.0;
        let color = format!(
            "rgba({}, {}, {}, {})",
            (Math::random() * 255.0) as u8,
            (Math::random() * 255.0) as u8,
            (Math::random() * 255.0) as u8,
            Math::random()
        );
        draw_circle(&context, x, y, radius, &color);
    }
}

fn draw_circle(context: &CanvasRenderingContext2d, x: f64, y: f64, radius: f64, color: &str) {
    // 调用canvas的API绘制
    context.begin_path();
    context.arc(x, y, radius, 0.0, 2.0 * std::f64::consts::PI).unwrap();
    context.set_fill_style(&wasm_bindgen::JsValue::from_str(color));
    context.fill();
    context.stroke();
}

#[wasm_bindgen]
pub fn fb_wasm(n: i32) -> i32 {
    if n <= 1 {
        1
    } else {
        fb_wasm(n - 1) + fb_wasm(n - 2)
    }
}
