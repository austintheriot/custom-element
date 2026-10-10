use js_sys::{Array, Function};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::HtmlElement;

use crate::{
    bridge::Bridge, constructors::html_element_constructor, CustomElement, GeneratedConstructor,
};

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
/// let (closure, constructor) = create_custom_element(
///     move |_element, _arguments| {
///         let next = sequence.get() + 1;
///         sequence.set(next);
///         Item { sequence: next }
///     },
/// );
/// closure.forget();
/// let _ = constructor;
/// ```
///
/// The factory must not add attributes or children to the element. When
/// `document.createElement` creates the element, the browser rejects such a
/// constructor and returns an `HTMLUnknownElement` with no Rust component.
/// The parser, an upgrade, and `new` keep the additions, so the defect stays
/// hidden until a caller uses `document.createElement`. Add children to a
/// shadow root, or add them in `connected_callback`. Setting an observed
/// attribute inside the factory also runs `attributeChangedCallback` before
/// the element holds its component, and the shim throws a `TypeError`. This
/// `TypeError` happens on every construction path.
///
/// While an existing element upgrades, constructing another instance of the
/// same registered type makes the browser report a `TypeError`. The nested
/// call returns an element with no Rust component. Fresh construction
/// permits same-type reentry when application state has no conflicting borrow.
pub fn create_custom_element<
    Component: CustomElement + 'static,
    Factory: Fn(HtmlElement, Array) -> Component + 'static,
>(
    factory: Factory,
) -> (MakeStruct, GeneratedConstructor) {
    create_custom_element_extending(&html_element_constructor(), factory)
}

/// Creates a custom element whose class extends `base`.
///
/// Pass a function from [`crate::constructors`] as `base`. For a customized
/// built-in element, define the returned constructor with the matching
/// `extends` option, for example `p` for
/// [`crate::constructors::html_paragraph_element_constructor`]. The factory
/// has the same per-element and lifetime contract as
/// [`create_custom_element`].
pub fn create_custom_element_extending<
    Component: CustomElement + 'static,
    Factory: Fn(HtmlElement, Array) -> Component + 'static,
>(
    base: &Function,
    factory: Factory,
) -> (MakeStruct, GeneratedConstructor) {
    let attributes = Component::observed_attributes()
        .iter()
        .map(|attribute| (*attribute).to_owned())
        .collect();
    let closure = Closure::new(move |element: JsValue, args| {
        let bridge: Bridge = factory(element.unchecked_into(), args).into();
        bridge.into()
    });
    let class = _create_custom_element(&closure, attributes, base);
    (closure, GeneratedConstructor(class))
}
