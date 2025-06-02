use std::{cell::RefCell, rc::Rc};

use custom_element::{
    constructors::{HTML_ELEMENT_CONSTRUCTOR, HTML_PARAGRAPH_ELEMENT_CONSTRUCTOR},
    create_custom_element, create_custom_element_with_config, CustomElement,
    CustomElementConfiguration, GeneratedConstructor,
};
use js_sys::{Array, Object, Reflect};
use rand::{distributions::Alphanumeric, Rng};
use wasm_bindgen::prelude::*;
use web_sys::{window, Element, HtmlElement};

pub struct MockCustomElement {
    #[allow(dead_code)]
    pub element_instance: JsValue,
    pub args: Array,
}

impl CustomElement for MockCustomElement {}

impl MockCustomElement {
    pub fn new(element_instance: JsValue) -> Self {
        Self {
            element_instance,
            args: Array::new(),
        }
    }

    pub fn new_with_args(element_instance: JsValue, args: Array) -> Self {
        Self {
            element_instance,
            args,
        }
    }
}

fn get_random_letter_string() -> String {
    let s: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .take(10)
        .map(char::from)
        .collect();

    s
}

pub fn get_random_custom_element_name() -> String {
    let random_string_a = get_random_letter_string();
    let random_string_b = get_random_letter_string();
    let random_string_c = get_random_letter_string();
    format!("{random_string_a}-{random_string_b}-{random_string_c}")
}

pub fn create_autonomous_custom_element_constructor() -> GeneratedConstructor {
    let (closure, constructor) = create_custom_element(
        |element_instance, _args| MockCustomElement::new(element_instance),
        vec![String::from("count")],
    );
    closure.forget();
    constructor
}

pub fn create_customized_built_in_element_constructor() -> GeneratedConstructor {
    let (closure, constructor) = create_custom_element_with_config(
        |element_instance, _args| MockCustomElement::new(element_instance),
        vec![String::from("count")],
        CustomElementConfiguration {
            element_constructor: &HTML_PARAGRAPH_ELEMENT_CONSTRUCTOR,
        },
    );
    closure.forget();
    constructor
}

pub fn assert_is_autonomous_element_constructor(v: &JsValue) {
    assert!(v.is_function());
    let prototype = Object::get_prototype_of(v);
    assert!(prototype.is_function());
    let name = Reflect::get(&prototype, &JsValue::from_str("name")).unwrap();
    assert_eq!(name, "HTMLElement");
}

pub fn assert_is_customized_built_in_constructor(v: &JsValue) {
    assert!(v.is_function());
    let prototype = Object::get_prototype_of(v);
    assert!(prototype.is_function());
    let name = Reflect::get(&prototype, &JsValue::from_str("name")).unwrap();
    assert_ne!(name, "HTMLElement");
    assert!(name.as_string().unwrap().contains("Element"))
}

pub fn assert_is_custom_element_instance(v: &JsValue) {
    assert!(v.is_object());
    assert!(v.is_instance_of::<HtmlElement>());
}

#[derive(Clone, Default)]
pub struct CallCounters {
    pub num_calls_connected: Rc<RefCell<u32>>,
    pub num_calls_disconnected: Rc<RefCell<u32>>,
    pub num_calls_adopted: Rc<RefCell<u32>>,
    pub num_calls_attribute_changed: Rc<RefCell<u32>>,
    pub num_calls_handle_event: Rc<RefCell<u32>>,
}

pub struct CustomElementCallCounter {
    call_counters: CallCounters,
}

/// element that tracks the number of calls to its callbacks
impl CustomElementCallCounter {
    pub fn new(call_counters: CallCounters) -> Self {
        Self { call_counters }
    }
}

impl CustomElement for CustomElementCallCounter {
    fn connected_callback(&mut self) {
        *self.call_counters.num_calls_connected.borrow_mut() += 1;
    }

    fn disconnected_callback(&mut self) {
        *self.call_counters.num_calls_disconnected.borrow_mut() += 1;
    }

    fn adopted_callback(&mut self) {
        *self.call_counters.num_calls_adopted.borrow_mut() += 1;
    }

    fn attribute_changed_callback(
        &mut self,
        _name: &str,
        _old_value: wasm_bindgen::JsValue,
        _new_value: wasm_bindgen::JsValue,
    ) {
        *self.call_counters.num_calls_attribute_changed.borrow_mut() += 1;
    }

    fn handle_event(&mut self, _event: web_sys::Event) {
        *self.call_counters.num_calls_handle_event.borrow_mut() += 1;
    }
}

pub fn create_custom_element_with_call_counters() -> (Element, CallCounters) {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let call_counters = CallCounters::default();

    // create constructor
    let (closure, constructor) = {
        let call_counters = call_counters.clone();

        create_custom_element_with_config(
            move |_element_instance, _args| CustomElementCallCounter::new(call_counters.clone()),
            vec![String::from("observed")],
            CustomElementConfiguration {
                element_constructor: &HTML_ELEMENT_CONSTRUCTOR,
            },
        )
    };
    closure.forget();

    let name = get_random_custom_element_name();
    window
        .custom_elements()
        .define(&name, &constructor)
        .unwrap();

    let instance = document.create_element(&name).unwrap();

    (instance, call_counters)
}
