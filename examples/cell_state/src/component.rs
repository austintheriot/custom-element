use std::cell::Cell;

use custom_element::CustomElement;
use js_sys::{Array, Object};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Event, HtmlElement, ShadowRootInit, ShadowRootMode};

pub(crate) struct CellCounter {
    element: HtmlElement,
    button: HtmlElement,
    output: HtmlElement,
    count: Cell<u32>,
}

impl CellCounter {
    pub(crate) fn new(element: HtmlElement, _args: Array) -> Self {
        let shadow = element
            .attach_shadow(&ShadowRootInit::new(ShadowRootMode::Open))
            .unwrap();
        let document = web_sys::window().unwrap().document().unwrap();
        let button: HtmlElement = document.create_element("button").unwrap().unchecked_into();
        let output: HtmlElement = document.create_element("span").unwrap().unchecked_into();
        button.set_attribute("data-role", "increment").unwrap();
        button.set_text_content(Some("Increment"));
        output.set_attribute("data-role", "display").unwrap();
        shadow.append_child(&button).unwrap();
        shadow.append_child(&output).unwrap();
        Self {
            element,
            button,
            output,
            count: Cell::new(0),
        }
    }

    fn increment(&self) {
        let next = self.count.get().saturating_add(1);
        self.count.set(next);
        self.element
            .set_attribute("count", &next.to_string())
            .unwrap();
    }

    fn render(&self) {
        self.output
            .set_text_content(Some(&format!("Count: {}", self.count.get())));
    }
}

impl CustomElement for CellCounter {
    fn observed_attributes() -> &'static [&'static str] {
        &["count"]
    }

    fn connected_callback(&self) {
        self.render();
        self.button
            .add_event_listener_with_callback("click", &self.element.clone().unchecked_into())
            .unwrap();
    }

    fn disconnected_callback(&self) {
        self.button
            .remove_event_listener_with_callback("click", &self.element.clone().unchecked_into())
            .unwrap();
    }

    fn handle_event(&self, event: Event) {
        if event.type_() == "click"
            && event
                .current_target()
                .is_some_and(|target| Object::is(&target, &self.button))
        {
            self.increment();
        }
    }

    fn attribute_changed_callback(&self, _name: &str, _old: JsValue, new: JsValue) {
        let count = new
            .as_string()
            .and_then(|value| value.parse::<u32>().ok())
            .unwrap_or(0);
        self.count.set(count);
        self.render();
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
