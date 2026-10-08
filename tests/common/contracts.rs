use std::rc::Rc;

use custom_element::{create_custom_element, CustomElement, GeneratedConstructor};
use js_sys::{Array, Function, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlElement;

pub enum Call {
    Connected,
    Disconnected,
    Adopted,
    Moved,
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
        (self.hook)(&self.element, Call::Moved);
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

pub fn register_recorder(hook: Hook) -> (String, GeneratedConstructor) {
    let name = super::helpers::get_random_custom_element_name();
    let (factory, constructor) = create_custom_element(
        move |element, _arguments| Recorder {
            element: element.into(),
            hook: hook.clone(),
        },
        vec!["observed".to_owned()],
    );
    factory.forget();
    web_sys::window()
        .unwrap()
        .custom_elements()
        .define(&name, &constructor)
        .unwrap();
    (name, constructor)
}

pub fn create_recorder(hook: Hook) -> HtmlElement {
    let (name, _constructor) = register_recorder(hook);
    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element(&name)
        .unwrap()
        .dyn_into()
        .unwrap()
}

pub fn invoke(target: &JsValue, method: &str, arguments: &Array) -> Result<JsValue, JsValue> {
    let function = Reflect::get(target, &JsValue::from_str(method))?.dyn_into::<Function>()?;
    function.apply(target, arguments)
}
