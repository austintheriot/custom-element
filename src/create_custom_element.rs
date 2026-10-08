use js_sys::{Array, Function};
use wasm_bindgen::prelude::*;

use crate::{constructors::HTML_ELEMENT_CONSTRUCTOR, Bridge, CustomElement, GeneratedConstructor};

type MakeStruct = Closure<dyn Fn(JsValue, Array) -> JsValue>;

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

impl Default for CustomElementConfiguration<'_> {
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

/// Creates a custom element with the default configuration.
///
/// The factory implements `Fn` and runs once for each new element. It can
/// capture shared configuration, but it must create a component for each
/// call. Keep the returned closure alive while the constructor can create
/// elements. Browser custom-element construction and upgrade restrictions
/// still apply, including the restriction on recursive upgrades.
pub fn create_custom_element<
    Component: CustomElement + 'static,
    Factory: Fn(JsValue, Array) -> Component + 'static,
>(
    factory: Factory,
    attributes: Vec<String>,
) -> (MakeStruct, GeneratedConstructor) {
    let config = CustomElementConfiguration::default();
    create_custom_element_with_config(factory, attributes, config)
}

/// Creates a custom element with a custom configuration.
///
/// The factory has the same per-element and lifetime contract as
/// [`create_custom_element`].
pub fn create_custom_element_with_config<
    Component: CustomElement + 'static,
    Factory: Fn(JsValue, Array) -> Component + 'static,
>(
    factory: Factory,
    attributes: Vec<String>,
    config: CustomElementConfiguration,
) -> (MakeStruct, GeneratedConstructor) {
    let closure = Closure::new(move |element, args| {
        let bridge: Bridge = factory(element, args).into();
        bridge.into()
    });
    let class = _create_custom_element(&closure, attributes, config.element_constructor);
    (closure, GeneratedConstructor(class))
}
