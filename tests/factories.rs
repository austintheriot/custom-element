use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use custom_element::{create_custom_element, CustomElement};
use js_sys::{Array, Object};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use web_sys::window;

mod common;

wasm_bindgen_test_configure!(run_in_browser);

struct Empty;

impl CustomElement for Empty {}

#[wasm_bindgen_test]
fn factory_can_create_another_registered_element_before_returning() {
    let name = common::helpers::get_random_custom_element_name();
    let calls = Rc::new(Cell::new(0));
    let instances = Rc::new(RefCell::new(Vec::<JsValue>::new()));
    let recorded_calls = calls.clone();
    let recorded_instances = instances.clone();
    let nested_name = name.clone();
    let (factory, constructor) = create_custom_element(
        move |instance, _arguments: Array| {
            let next_call = recorded_calls.get() + 1;
            recorded_calls.set(next_call);
            recorded_instances.borrow_mut().push(instance);
            if next_call == 1 {
                let _nested = window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .create_element(&nested_name)
                    .unwrap();
            }
            Empty
        },
        vec![],
    );
    factory.forget();
    window()
        .unwrap()
        .custom_elements()
        .define(&name, &constructor)
        .unwrap();

    let outer = window()
        .unwrap()
        .document()
        .unwrap()
        .create_element(&name)
        .unwrap();

    assert_eq!(calls.get(), 2);
    let recorded = instances.borrow();
    assert_eq!(recorded.len(), 2);
    assert!(Object::is(outer.as_ref(), &recorded[0]));
    assert!(!Object::is(&recorded[0], &recorded[1]));
}
