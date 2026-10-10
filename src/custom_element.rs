use wasm_bindgen::JsValue;
use web_sys::Event;

/// Defines callbacks forwarded from a browser custom element to a Rust component.
///
/// Every callback takes `&self` and has a default no-op implementation.
/// Use `Cell` or `RefCell` when a callback changes Rust-owned state. A
/// component can instead keep state in element attributes. Browser operations
/// can invoke another callback before the current callback returns. End a
/// conflicting `RefCell` guard before such an operation. A shared guard
/// conflicts with a nested mutable borrow. A mutable guard conflicts with
/// every nested borrow.
pub trait CustomElement {
    /// Names the attributes whose changes call `attribute_changed_callback`
    ///
    /// The browser reads this list once, when the element is defined. The
    /// default observes no attributes.
    ///
    /// See [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements#responding_to_attribute_changes)
    fn observed_attributes() -> &'static [&'static str]
    where
        Self: Sized,
    {
        &[]
    }

    /// Called each time the element is added to the document    
    ///
    /// See [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements)
    fn connected_callback(&self) {}

    /// Called each time the element is removed from the document    
    ///
    /// See [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements)
    fn disconnected_callback(&self) {}

    /// Called each time the element is moved to a new document
    ///
    /// See [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements)
    fn adopted_callback(&self) {}

    /// Enables using the custom element instance as an event handler catch-all
    ///
    /// see <https://gomakethings.com/the-handleevent-method-is-the-absolute-best-way-to-handle-events-in-web-components/>
    fn handle_event(&self, _event: Event) {}

    /// Called instead of `disconnected_callback` and `connected_callback` each
    /// time `Element.moveBefore()` moves the element inside the document
    ///
    /// The generated element class always defines `connectedMoveCallback`, so
    /// the browser never runs its own disconnect and reconnect for a move. The
    /// default calls `disconnected_callback` and then `connected_callback`,
    /// which matches the browser behavior for an element without the callback.
    /// Override it to keep state across a move.
    ///
    /// See [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements)
    fn connected_move_callback(&self) {
        self.disconnected_callback();
        self.connected_callback();
    }

    /// Called when attributes are changed, added, removed, or replaced
    ///
    /// See [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/Web_components/Using_custom_elements)
    fn attribute_changed_callback(&self, _name: &str, _old_value: JsValue, _new_value: JsValue) {}
}
