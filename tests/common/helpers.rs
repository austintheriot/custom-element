use custom_element::{
    constructors::HTML_PARAGRAPH_ELEMENT_CONSTRUCTOR, create_custom_element,
    create_custom_element_with_config, CustomElementConfiguration, CustomElement,
    GeneratedConstructor,
};
use js_sys::{Array, Object, Reflect};
use rand::{distributions::Alphanumeric, Rng};
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

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
