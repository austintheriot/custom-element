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
/// elements.
///
/// Replace mutable scalar captures with `Cell`. For captured compound state,
/// use `RefCell` and end each borrow before nested construction. A nested
/// factory can access the same capture.
///
/// ```no_run
/// use std::cell::Cell;
/// use custom_element::{create_custom_element, CustomElement};
///
/// struct Item { sequence: u32 }
/// impl CustomElement for Item {}
///
/// let sequence = Cell::new(0);
/// let (factory, constructor) = create_custom_element(
///     move |_element, _arguments| {
///         let next = sequence.get() + 1;
///         sequence.set(next);
///         Item { sequence: next }
///     },
///     vec![],
/// );
/// factory.forget();
/// let _ = constructor;
/// ```
///
/// While an existing element upgrades, constructing another instance of the
/// same registered type fails with a browser `TypeError`. Fresh construction
/// permits same-type reentry when application state has no conflicting borrow.
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
