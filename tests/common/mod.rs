pub mod contracts;
pub mod helpers;

use js_sys::{Array, Promise};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/tests/common/browser.js")]
extern "C" {
    #[wasm_bindgen(js_name = captureErrors)]
    pub fn capture_errors(action: &mut dyn FnMut()) -> Array;

    #[wasm_bindgen(js_name = nextTurn)]
    pub fn next_turn() -> Promise;
}

pub fn observe_errors<Output>(mut action: impl FnMut() -> Output) -> (Output, Array) {
    let mut output = None;
    let errors = capture_errors(&mut || output = Some(action()));
    (output.unwrap(), errors)
}

pub fn assert_no_browser_errors<Output>(action: impl FnMut() -> Output) -> Output {
    let (output, errors) = observe_errors(action);
    assert_eq!(errors.length(), 0, "unexpected browser errors: {errors:?}");
    output
}
