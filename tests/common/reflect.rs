use js_sys::{Array, Function, Reflect};
use wasm_bindgen::{JsCast, JsValue};

pub fn invoke(target: &JsValue, method: &str, arguments: &Array) -> Result<JsValue, JsValue> {
    let function = Reflect::get(target, &JsValue::from_str(method))?.dyn_into::<Function>()?;
    function.apply(target, arguments)
}
