use std::{cell::RefCell, rc::Rc};

use custom_element::{create_custom_element, CustomElement};
use js_sys::{Array, Object};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use web_sys::{window, Event};

mod common;

use common::contracts::{create_recorder, invoke, Call, Hook};

wasm_bindgen_test_configure!(run_in_browser);

fn forwarding_cases() -> Vec<(&'static str, Array, Call)> {
    let event = Event::new("sample").unwrap();
    let attribute_arguments = Array::new();
    attribute_arguments.push(&JsValue::from_str("observed"));
    attribute_arguments.push(&JsValue::NULL);
    attribute_arguments.push(&JsValue::from_str("value"));
    let event_arguments = Array::new();
    event_arguments.push(event.as_ref());

    vec![
        ("connectedCallback", Array::new(), Call::Connected),
        ("disconnectedCallback", Array::new(), Call::Disconnected),
        ("adoptedCallback", Array::new(), Call::Adopted),
        ("connectedMoveCallback", Array::new(), Call::Moved),
        (
            "attributeChangedCallback",
            attribute_arguments,
            Call::Attribute {
                name: "observed".to_owned(),
                old: JsValue::NULL,
                new: JsValue::from_str("value"),
            },
        ),
        ("handleEvent", event_arguments, Call::Event(event)),
    ]
}

fn assert_call(actual: Call, expected: Call) {
    match (actual, expected) {
        (Call::Connected, Call::Connected)
        | (Call::Disconnected, Call::Disconnected)
        | (Call::Adopted, Call::Adopted)
        | (Call::Moved, Call::Moved) => {}
        (
            Call::Attribute {
                name: actual_name,
                old: actual_old,
                new: actual_new,
            },
            Call::Attribute {
                name: expected_name,
                old: expected_old,
                new: expected_new,
            },
        ) => {
            assert_eq!(actual_name, expected_name);
            assert!(Object::is(&actual_old, &expected_old));
            assert!(Object::is(&actual_new, &expected_new));
        }
        (Call::Event(actual_event), Call::Event(expected_event)) => {
            assert!(Object::is(actual_event.as_ref(), expected_event.as_ref()));
        }
        _ => panic!("forwarded the wrong callback"),
    }
}

#[wasm_bindgen_test]
fn forwards_each_callback_with_original_arguments() {
    for (method, arguments, expected) in forwarding_cases() {
        let calls = Rc::new(RefCell::new(Vec::new()));
        let calls_for_hook = calls.clone();
        let hook: Hook = Rc::new(move |_element, call| {
            calls_for_hook.borrow_mut().push(call);
        });
        let element = create_recorder(hook);

        invoke(element.as_ref(), method, &arguments).unwrap();

        let actual_calls = std::mem::take(&mut *calls.borrow_mut());
        assert_eq!(actual_calls.len(), 1, "{method} call count");
        assert_call(actual_calls.into_iter().next().unwrap(), expected);
    }
}

struct Empty;

impl CustomElement for Empty {}

#[wasm_bindgen_test]
fn default_callbacks_accept_every_invocation() {
    for (method, arguments, _expected) in forwarding_cases() {
        let (factory, constructor) =
            create_custom_element(|_element, _arguments| Empty, vec!["observed".to_owned()]);
        factory.forget();
        let name = common::helpers::get_random_custom_element_name();
        window()
            .unwrap()
            .custom_elements()
            .define(&name, &constructor)
            .unwrap();
        let element = window()
            .unwrap()
            .document()
            .unwrap()
            .create_element(&name)
            .unwrap();

        invoke(element.as_ref(), method, &arguments).unwrap();
    }
}
