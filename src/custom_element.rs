use wasm_bindgen::JsValue;
use web_sys::Event;

/// These methods are called from a JS wrapper class
/// that extends `HTMLElement`, allowing autonomous,
/// custom elements.
pub trait CustomElement {
    fn observed_attributes() -> Vec<String>
    where
        Self: Sized,
    {
        vec![]
    }

    fn connected_callback(&mut self) {}

    fn disconnected_callback(&mut self) {}

    fn adopted_callback(&mut self) {}

    /// Enables using `this` as an event handler catch-all
    /// see https://gomakethings.com/the-handleevent-method-is-the-absolute-best-way-to-handle-events-in-web-components/
    fn handle_event(&mut self, _event: Event) {}

    fn attribute_changed_callback(
        &mut self,
        _name: &str,
        _old_value: JsValue,
        _new_value: JsValue,
    ) {
    }
}
