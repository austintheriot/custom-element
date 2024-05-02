use wasm_bindgen::prelude::*;

mod custom_element_app;
mod leptos_app;
mod state;

use custom_element_app::*;
use state::*;

pub use custom_element_app::get_app_tag_name;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
}

#[wasm_bindgen(js_name = initApp)]
pub fn init_app() {
    let state = init_state();
    CustomElementApp::register(state);
}
