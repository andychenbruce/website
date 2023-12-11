use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}



#[wasm_bindgen]
pub async fn andy_main() -> Result<(), JsValue> {
    
    web_sys::console::log_1(&"hi lol\n".into());
    Ok(())
}
