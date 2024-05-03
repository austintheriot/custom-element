use js_sys::{Array, Function};
use wasm_bindgen::prelude::*;

use crate::{constructors::HTML_ELEMENT_CONSTRUCTOR, Bridge, CustomElement, GeneratedConstructor};

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

/// Allows specifying details about how your custom element
/// should be created. Currently, this is limited to specifying
/// which constructor function your custom element should inherit from.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct CustomElementConfiguration<'a> {
    /// Which constructor your custom element should inherit from
    pub element_constructor: &'a Function,
}

impl<'a> Default for CustomElementConfiguration<'a> {
    fn default() -> Self {
        Self {
            element_constructor: &HTML_ELEMENT_CONSTRUCTOR,
        }
    }
}

/// The result of creating a custom element.
pub struct CustomElementResult {
    /// The [`CustomElementResult`].constructor is the generated
    /// class for your custom element.
    pub constructor: GeneratedConstructor,

    /// The returned [`CustomElementResult`].closure contains
    /// the function you gave. It must be held in memory, or else the JS
    /// side will error when trying to call the callback.
    ///
    /// To have the closure live indefinitely (probably the
    /// behavior you want), you can just call `closure.forget()`.
    pub closure: MakeStruct,
}

/// Create a custom element with the default configuration
pub fn create_custom_element<
    C: CustomElement + 'static,
    F: FnMut(JsValue, Array) -> C + 'static,
>(
    create_cutom_element: F,
    attributes: Vec<String>,
) -> (MakeStruct, GeneratedConstructor) {
    let config = CustomElementConfiguration::default();
    create_custom_element_with_config(create_cutom_element, attributes, config)
}

/// Create a custom element with a custom configuration
pub fn create_custom_element_with_config<
    C: CustomElement + 'static,
    F: FnMut(JsValue, Array) -> C + 'static,
>(
    mut create_cutom_element: F,
    attributes: Vec<String>,
    config: CustomElementConfiguration,
) -> (MakeStruct, GeneratedConstructor) {
    let closure = Closure::new(move |element, args| {
        let bridge: Bridge = create_cutom_element(element, args).into();
        bridge.into()
    });
    let class = _create_custom_element(&closure, attributes, config.element_constructor);
    (closure, GeneratedConstructor(class))
}
