use wasm_bindgen::prelude::*;
use web_sys::window;

#[wasm_bindgen]
pub fn run() {
    if let Some(window) = window() {
        window.alert_with_message("Hello, WebAssembly!").expect("Should display an alert");
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    run();
    Ok(())
}
