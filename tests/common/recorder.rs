use std::rc::Rc;

use custom_element::{create_custom_element, CustomElement};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlElement;

pub enum Call {
    Connected,
    Disconnected,
    Adopted,
    ConnectedMove,
    Attribute {
        name: String,
        old: JsValue,
        new: JsValue,
    },
    Event(web_sys::Event),
}

pub type Hook = Rc<dyn Fn(&HtmlElement, Call)>;

struct Recorder {
    element: HtmlElement,
    hook: Hook,
}

impl CustomElement for Recorder {
    fn connected_callback(&self) {
        (self.hook)(&self.element, Call::Connected);
    }

    fn disconnected_callback(&self) {
        (self.hook)(&self.element, Call::Disconnected);
    }

    fn adopted_callback(&self) {
        (self.hook)(&self.element, Call::Adopted);
    }

    fn connected_move_callback(&self) {
        (self.hook)(&self.element, Call::ConnectedMove);
    }

    fn handle_event(&self, event: web_sys::Event) {
        (self.hook)(&self.element, Call::Event(event));
    }

    fn attribute_changed_callback(&self, name: &str, old: JsValue, new: JsValue) {
        (self.hook)(
            &self.element,
            Call::Attribute {
                name: name.to_owned(),
                old,
                new,
            },
        );
    }
}

pub fn create_recorder(hook: Hook) -> HtmlElement {
    let name = super::names::unique_custom_element_name();
    let (closure, constructor) = create_custom_element(
        move |element, _arguments| Recorder {
            element,
            hook: hook.clone(),
        },
        vec!["observed".to_owned()],
    );
    closure.forget();
    web_sys::window()
        .unwrap()
        .custom_elements()
        .define(&name, &constructor)
        .unwrap();
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element(&name)
        .unwrap()
        .dyn_into()
        .unwrap()
}
