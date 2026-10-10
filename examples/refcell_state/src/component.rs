use std::cell::RefCell;

use custom_element::CustomElement;
use js_sys::{Array, Object};
use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlElement, ShadowRootInit, ShadowRootMode};

struct Model {
    count: u32,
    updates: u32,
}

pub(crate) struct CompoundCounter {
    element: HtmlElement,
    button: HtmlElement,
    output: HtmlElement,
    model: RefCell<Model>,
}

impl CompoundCounter {
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
            model: RefCell::new(Model {
                count: 0,
                updates: 0,
            }),
        }
    }

    fn increment(&self) {
        let (count, label) = {
            let mut model = self.model.borrow_mut();
            model.count = model.count.saturating_add(1);
            model.updates = model.updates.saturating_add(1);
            (
                model.count,
                format!("Count: {}; updates: {}", model.count, model.updates),
            )
        };
        self.output.set_text_content(Some(&label));
        self.element
            .set_attribute("count", &count.to_string())
            .unwrap();
    }
}

impl CustomElement for CompoundCounter {
    fn observed_attributes() -> &'static [&'static str] {
        &["count"]
    }

    fn connected_callback(&self) {
        let label = {
            let model = self.model.borrow();
            format!("Count: {}; updates: {}", model.count, model.updates)
        };
        self.output.set_text_content(Some(&label));
        self.button
            .add_event_listener_with_event_listener("click", self.element.unchecked_ref())
            .unwrap();
    }

    fn disconnected_callback(&self) {
        self.button
            .remove_event_listener_with_event_listener("click", self.element.unchecked_ref())
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

    fn attribute_changed_callback(&self, _name: &str, _old: Option<String>, new: Option<String>) {
        let count = new.and_then(|value| value.parse::<u32>().ok()).unwrap_or(0);
        let label = {
            let mut model = self.model.borrow_mut();
            if model.count != count {
                model.count = count;
                model.updates = model.updates.saturating_add(1);
            }
            format!("Count: {}; updates: {}", model.count, model.updates)
        };
        self.output.set_text_content(Some(&label));
    }
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
