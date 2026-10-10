use wasm_bindgen::prelude::*;
use web_sys::Event;

use crate::CustomElement;

/// Currently, wasm-bindgen doesn't support making
/// trait implementations callable from JavaScript.
///
/// This is problematic for allowing arbitrary structs
/// to implement consistent CustomElement functionality.
///
/// `Bridge` provides a hard-coded, non-trait-based implementation
/// of  the trait function bindings that can actually be called from JS.
#[wasm_bindgen]
#[doc(hidden)]
pub struct Bridge {
    inner: Box<dyn CustomElement>,
}

impl Bridge {
    #[doc(hidden)]
    pub fn new<Component: CustomElement + 'static>(custom_element: Component) -> Self {
        Self {
            inner: Box::new(custom_element),
        }
    }
}

impl<Component: CustomElement + 'static> From<Component> for Bridge {
    fn from(custom_element: Component) -> Self {
        Bridge::new(custom_element)
    }
}

#[wasm_bindgen]
impl Bridge {
    #[doc(hidden)]
    #[wasm_bindgen(js_name = connectedCallback)]
    pub fn connected_callback(&self) {
        self.inner.connected_callback()
    }

    #[doc(hidden)]
    #[wasm_bindgen(js_name = disconnectedCallback)]
    pub fn disconnected_callback(&self) {
        self.inner.disconnected_callback()
    }

    #[doc(hidden)]
    #[wasm_bindgen(js_name = adoptedCallback)]
    pub fn adopted_callback(&self) {
        self.inner.adopted_callback()
    }

    #[doc(hidden)]
    #[wasm_bindgen(js_name = connectedMoveCallback)]
    pub fn connected_move_callback(&self) {
        self.inner.connected_move_callback()
    }

    #[doc(hidden)]
    #[wasm_bindgen(js_name = attributeChangedCallback)]
    pub fn attribute_changed_callback(&self, name: &str, old_value: JsValue, new_value: JsValue) {
        self.inner
            .attribute_changed_callback(name, old_value, new_value)
    }

    #[doc(hidden)]
    #[wasm_bindgen(js_name = handleEvent)]
    pub fn handle_event(&self, event: Event) {
        self.inner.handle_event(event)
    }
}
