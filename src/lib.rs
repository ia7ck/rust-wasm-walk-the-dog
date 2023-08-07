use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, points: [(f64, f64); 3]) {
    let [(top_x, top_y), (left_x, left_y), (right_x, right_y)] = points;
    context.move_to(top_x, top_y);
    context.begin_path();
    context.line_to(left_x, left_y);
    context.line_to(right_x, right_y);
    context.line_to(top_x, top_y);
    context.close_path();
    context.stroke();
}

fn sierpinski(context: &web_sys::CanvasRenderingContext2d, points: [(f64, f64); 3], depth: u8) {
    if depth == 0 {
        draw_triangle(context, points);
    } else {
        let midpoint = |(px, py), (qx, qy)| ((px + qx) / 2.0, (py + qy) / 2.0);
        let [top, left, right] = points;
        let left_middle = midpoint(top, left);
        let right_middle = midpoint(top, right);
        let bottom_middle = midpoint(left, right);
        sierpinski(context, [top, left_middle, right_middle], depth - 1);
        sierpinski(context, [left_middle, left, bottom_middle], depth - 1);
        sierpinski(context, [right_middle, bottom_middle, right], depth - 1);
    }
}

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    sierpinski(&context, [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)], 5);
    Ok(())
}
