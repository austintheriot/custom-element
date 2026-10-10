use std::{fmt, rc::Rc};

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

impl fmt::Debug for Call {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Call::Connected => formatter.write_str("Connected"),
            Call::Disconnected => formatter.write_str("Disconnected"),
            Call::Adopted => formatter.write_str("Adopted"),
            Call::ConnectedMove => formatter.write_str("ConnectedMove"),
            Call::Attribute { name, old, new } => formatter
                .debug_struct("Attribute")
                .field("name", name)
                .field("old", old)
                .field("new", new)
                .finish(),
            Call::Event(event) => formatter.debug_tuple("Event").field(event).finish(),
        }
    }
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
    let name = super::names::get_random_custom_element_name();
    let (closure, constructor) = create_custom_element(
        move |element, _arguments| Recorder {
            element: element.into(),
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
