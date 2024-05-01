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
    pub fn new<C: CustomElement + 'static>(custom_element: C) -> Self {
        Self {
            inner: Box::new(custom_element),
        }
    }
}

impl<C: CustomElement + 'static> From<C> for Bridge {
    fn from(custom_element: C) -> Self {
        Bridge::new(custom_element)
    }
}

#[wasm_bindgen]
impl Bridge {
    #[doc(hidden)]
    #[wasm_bindgen(js_name = connectedCallback)]
    pub fn connected_callback(&mut self) {
        self.inner.connected_callback()
    }

    #[doc(hidden)]
    #[wasm_bindgen(js_name = disconnectedCallback)]
    pub fn disconnected_callback(&mut self) {
        self.inner.disconnected_callback()
    }

    #[doc(hidden)]
    #[wasm_bindgen(js_name = adoptedCallback)]
    pub fn adopted_callback(&mut self) {
        self.inner.adopted_callback()
    }

    #[doc(hidden)]
    #[wasm_bindgen(js_name = attributeChangedCallback)]
    pub fn attribute_changed_callback(
        &mut self,
        name: &str,
        old_value: JsValue,
        new_value: JsValue,
    ) {
        self.inner
            .attribute_changed_callback(name, old_value, new_value)
    }

    #[doc(hidden)]
    #[wasm_bindgen(js_name = handleEvent)]
    pub fn handle_event(&mut self, event: Event) {
        self.inner.handle_event(event)
    }
}
