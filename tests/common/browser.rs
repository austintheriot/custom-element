use js_sys::{Array, Promise};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[wasm_bindgen(module = "/tests/common/browser.js")]
extern "C" {
    #[wasm_bindgen(js_name = captureErrors)]
    fn capture_errors(action: &mut dyn FnMut()) -> Array;

    #[wasm_bindgen(js_name = nextTurn)]
    pub fn next_turn() -> Promise;

    #[wasm_bindgen(js_name = dispatchProbe)]
    pub fn dispatch_probe(element: &HtmlElement, cancelable: bool) -> JsValue;

    #[wasm_bindgen(js_name = prepareProbe)]
    pub fn prepare_probe(element: &HtmlElement) -> JsValue;

    #[wasm_bindgen(js_name = shadowProbe)]
    pub fn shadow_probe(host: &HtmlElement) -> JsValue;

    #[wasm_bindgen(js_name = moveCapability)]
    pub fn move_capability() -> bool;

    #[wasm_bindgen(js_name = moveBetweenParents)]
    pub fn move_between_parents(element: &HtmlElement) -> JsValue;

    #[wasm_bindgen(js_name = connectWithPostEffectFailure)]
    pub fn connect_with_post_effect_failure(element: &HtmlElement);

    #[wasm_bindgen(js_name = injectBridgeFailure)]
    pub fn inject_bridge_failure(element: &HtmlElement) -> JsValue;

    #[wasm_bindgen(js_name = isFailedElement)]
    pub fn is_failed_element(element: &JsValue) -> bool;

    #[wasm_bindgen(js_name = plainUpgradeRecursion)]
    pub fn plain_upgrade_recursion(name: &str) -> JsValue;
}

pub fn capture_browser_errors<Output>(mut action: impl FnMut() -> Output) -> (Output, Array) {
    let mut output = None;
    let errors = capture_errors(&mut || output = Some(action()));
    (output.unwrap(), errors)
}

pub fn assert_no_browser_errors<Output>(action: impl FnMut() -> Output) -> Output {
    let (output, errors) = capture_browser_errors(action);
    assert_eq!(errors.length(), 0, "unexpected browser errors: {errors:?}");
    output
}
