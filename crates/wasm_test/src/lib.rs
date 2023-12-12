use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn andy_main() -> Result<(), JsValue> {
    web_sys::console::log_1(&"hi lol\n".into());
    Ok(())
}

fn setup_canvas() {
    let canvas: web_sys::WebGlRenderingContext = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("my_canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap()
        .get_context("webgl")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();
    canvas.clear_color(0.1, 0.1, 0.9, 1.0);
    canvas.clear(web_sys::WebGlRenderingContext::COLOR_BUFFER_BIT);
}

#[wasm_bindgen]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
    setup_canvas();
}
