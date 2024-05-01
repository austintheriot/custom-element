use wasm_bindgen::prelude::*;

mod app;
mod leptos_app;
mod state;

use app::*;
use state::*;

pub use app::get_app_tag_name;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());
}

#[wasm_bindgen(js_name = initApp)]
pub fn init_app() {
    let state = init_state();
    App::register(state);
}
