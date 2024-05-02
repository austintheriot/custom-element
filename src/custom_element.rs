use wasm_bindgen::JsValue;
use web_sys::Event;

/// This trait is the focal point of this crate, and implementing it
/// allows your Rust struct to be used as a custom element.
///
/// It specifies all the methods can be called from the JavaScript wrapper class.
///
/// All of these functions have been provided default, no-op implementations.
/// You may optionally provide implementations to hook into the custom element
/// lifecycle.
pub trait CustomElement {
    /// Called each time the element is added to the document    
    ///
    /// See [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements)
    fn connected_callback(&mut self) {}

    /// Called each time the element is removed from the document    
    ///
    /// See [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements)
    fn disconnected_callback(&mut self) {}

    /// Called each time the element is moved to a new document
    ///
    /// See [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements)
    fn adopted_callback(&mut self) {}

    /// Enables using the custom element instance as an event handler catch-all
    ///
    /// see <https://gomakethings.com/the-handleevent-method-is-the-absolute-best-way-to-handle-events-in-web-components/>
    fn handle_event(&mut self, _event: Event) {}

    /// Called when attributes are changed, added, removed, or replaced
    ///
    /// See [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements)
    fn attribute_changed_callback(
        &mut self,
        _name: &str,
        _old_value: JsValue,
        _new_value: JsValue,
    ) {
    }
}
