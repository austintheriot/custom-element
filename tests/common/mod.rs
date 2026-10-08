pub mod contracts;
pub mod helpers;

use js_sys::{Array, Function, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/tests/common/browser.js")]
extern "C" {
    #[wasm_bindgen(js_name = captureErrors)]
    pub fn capture_errors(action: &Function) -> Array;

    #[wasm_bindgen(js_name = nextTurn)]
    pub fn next_turn() -> Promise;
}
