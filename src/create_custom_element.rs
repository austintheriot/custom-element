use js_sys::{Array, Function};
use wasm_bindgen::prelude::*;

use crate::{Bridge, CustomElement, GeneratedConstructor, HTML_ELEMENT_CONSTRUCTOR};

type MakeStruct = Closure<dyn FnMut(JsValue, Array) -> JsValue>;

/// Javascript shim for wrapping Rust structs
#[wasm_bindgen(module = "/src/createCustomElement.js")]
extern "C" {
    #[wasm_bindgen(js_name = createCustomElement)]
    pub(self) fn _create_custom_element(
        make_rust_struct: &MakeStruct,
        attributes: Vec<String>,
        constructor: &Function,
    ) -> Function;
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct CustomElemenConfiguration<'a> {
    pub element_constructor: &'a Function,
}

impl<'a> Default for CustomElemenConfiguration<'a> {
    fn default() -> Self {
        Self {
            element_constructor: &HTML_ELEMENT_CONSTRUCTOR,
        }
    }
}

/// Create custom element with default configuration
pub fn create_custom_element<
    C: CustomElement + 'static,
    F: FnMut(JsValue, Array) -> C + 'static,
>(
    create_cutom_element: F,
    attributes: Vec<String>,
) -> (MakeStruct, GeneratedConstructor) {
    let config = CustomElemenConfiguration::default();
    create_custom_element_with_config(create_cutom_element, attributes, config)
}

/// Registers the component and returns the created closure
///
/// This closure must be held in memory, or else the JS
/// side will error when trying to call the callback
///
/// To have the closure live indefinitely (probably the
/// behavior you want), you can just call `closure.forget()`
pub fn create_custom_element_with_config<
    C: CustomElement + 'static,
    F: FnMut(JsValue, Array) -> C + 'static,
>(
    mut create_cutom_element: F,
    attributes: Vec<String>,
    config: CustomElemenConfiguration,
) -> (MakeStruct, GeneratedConstructor) {
    let closure = Closure::new(move |element, args| {
        let bridge: Bridge = create_cutom_element(element, args).into();
        bridge.into()
    });
    let class = _create_custom_element(&closure, attributes, config.element_constructor);
    (closure, GeneratedConstructor(class))
}
