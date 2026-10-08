use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use js_sys::{Array, Reflect};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use web_sys::{window, Event};

mod common;

use common::contracts::{create_recorder, invoke, Call};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn connection_delivers_nested_attribute_before_resuming() {
    let trace = Rc::new(RefCell::new(Vec::new()));
    let recorded = trace.clone();
    let element = create_recorder(Rc::new(move |element, call| match call {
        Call::Connected => {
            recorded.borrow_mut().push("connected:start");
            element.set_attribute("observed", "value").unwrap();
            recorded.borrow_mut().push("connected:end");
        }
        Call::Attribute { .. } => recorded.borrow_mut().push("attribute"),
        _ => {}
    }));
    window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&element)
        .unwrap();
    assert_eq!(
        *trace.borrow(),
        vec!["connected:start", "attribute", "connected:end"]
    );
    element.remove();
}

#[wasm_bindgen_test]
fn every_export_accepts_nested_shared_entry() {
    let attribute_arguments = Array::new();
    attribute_arguments.push(&JsValue::from_str("observed"));
    attribute_arguments.push(&JsValue::NULL);
    attribute_arguments.push(&JsValue::from_str("value"));
    let event_arguments = Array::new();
    event_arguments.push(Event::new("sample").unwrap().as_ref());
    let cases = [
        ("connectedCallback", Array::new()),
        ("disconnectedCallback", Array::new()),
        ("adoptedCallback", Array::new()),
        ("connectedMoveCallback", Array::new()),
        ("attributeChangedCallback", attribute_arguments),
        ("handleEvent", event_arguments),
    ];

    let mut failed_methods = Vec::new();
    for (method, arguments) in cases {
        let trace = Rc::new(RefCell::new(Vec::new()));
        let recorded = trace.clone();
        let in_outer_connection = Rc::new(Cell::new(false));
        let guard = in_outer_connection.clone();
        let element = create_recorder(Rc::new(move |element, call| {
            if matches!(call, Call::Connected) && !guard.get() {
                guard.set(true);
                recorded.borrow_mut().push("outer:start");
                let bridge = Reflect::get(element.as_ref(), &JsValue::from_str("struct")).unwrap();
                let _nested_result = invoke(&bridge, method, &arguments);
                recorded.borrow_mut().push("outer:end");
            } else {
                recorded.borrow_mut().push("nested");
            }
        }));
        window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .append_child(&element)
            .unwrap();
        if *trace.borrow() != vec!["outer:start", "nested", "outer:end"] {
            failed_methods.push(method);
        }
        element.remove();
    }
    assert!(
        failed_methods.is_empty(),
        "nested calls failed: {failed_methods:?}"
    );
}
