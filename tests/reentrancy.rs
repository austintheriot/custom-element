use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use js_sys::{Array, Reflect};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;
use web_sys::{window, Event, HtmlElement};

#[allow(dead_code)]
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
    common::assert_no_browser_errors(|| {
        window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .append_child(&element)
            .unwrap()
    });
    assert_eq!(
        *trace.borrow(),
        vec!["connected:start", "attribute", "connected:end"]
    );
    common::assert_no_browser_errors(|| element.remove());
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
                let nested_result = invoke(&bridge, method, &arguments);
                assert!(nested_result.is_ok(), "nested {method} invocation failed");
                recorded.borrow_mut().push("outer:end");
            } else {
                recorded.borrow_mut().push("nested");
            }
        }));
        common::assert_no_browser_errors(|| {
            window()
                .unwrap()
                .document()
                .unwrap()
                .body()
                .unwrap()
                .append_child(&element)
                .unwrap()
        });
        if *trace.borrow() != vec!["outer:start", "nested", "outer:end"] {
            failed_methods.push(method);
        }
        common::assert_no_browser_errors(|| element.remove());
    }
    assert!(
        failed_methods.is_empty(),
        "nested calls failed: {failed_methods:?}"
    );
}

#[wasm_bindgen_test]
fn equal_attribute_write_reenters_before_outer_return() {
    let trace = Rc::new(RefCell::new(Vec::new()));
    let recorded = trace.clone();
    let nested = Rc::new(Cell::new(false));
    let nested_guard = nested.clone();
    let element = create_recorder(Rc::new(move |element, call| {
        if let Call::Attribute { old, new, .. } = call {
            let old_value = old.as_string();
            let new_value = new.as_string().unwrap();
            recorded
                .borrow_mut()
                .push(("enter", old_value.clone(), new_value.clone()));
            if !nested_guard.replace(true) {
                element.set_attribute("observed", "value").unwrap();
            }
            recorded.borrow_mut().push(("exit", old_value, new_value));
        }
    }));
    common::assert_no_browser_errors(|| element.set_attribute("observed", "value").unwrap());
    assert_eq!(
        *trace.borrow(),
        vec![
            ("enter", None, "value".to_owned()),
            ("enter", Some("value".to_owned()), "value".to_owned()),
            ("exit", Some("value".to_owned()), "value".to_owned()),
            ("exit", None, "value".to_owned()),
        ]
    );
}

#[wasm_bindgen_test]
fn scoped_state_borrow_observes_nested_mutation() {
    let state = Rc::new(RefCell::new(0));
    let resumed = Rc::new(Cell::new(0));
    let state_for_hook = state.clone();
    let resumed_for_hook = resumed.clone();
    let element = create_recorder(Rc::new(move |element, call| match call {
        Call::Connected => {
            *state_for_hook.borrow_mut() = 1;
            element.set_attribute("observed", "value").unwrap();
            resumed_for_hook.set(*state_for_hook.borrow());
        }
        Call::Attribute { .. } => *state_for_hook.borrow_mut() = 2,
        _ => {}
    }));
    common::assert_no_browser_errors(|| {
        window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .append_child(&element)
            .unwrap()
    });
    assert_eq!(resumed.get(), 2);
    assert_eq!(*state.borrow(), 2);
    common::assert_no_browser_errors(|| element.remove());
}

#[wasm_bindgen_test]
fn held_borrows_report_conflicts_without_panicking() {
    let state = Rc::new(RefCell::new(0));
    let conflicts = Rc::new(RefCell::new(Vec::new()));
    let state_for_hook = state.clone();
    let conflicts_for_hook = conflicts.clone();
    let element = create_recorder(Rc::new(move |element, call| match call {
        Call::Connected => {
            let mut guard = state_for_hook.borrow_mut();
            *guard = 1;
            element.set_attribute("observed", "mutable").unwrap();
            drop(guard);
            assert!(state_for_hook.try_borrow_mut().is_ok());

            let guard = state_for_hook.borrow();
            element.set_attribute("observed", "immutable").unwrap();
            drop(guard);
            assert!(state_for_hook.try_borrow_mut().is_ok());
        }
        Call::Attribute { new, .. } => {
            conflicts_for_hook.borrow_mut().push((
                new.as_string().unwrap(),
                state_for_hook.try_borrow().is_ok(),
                state_for_hook.try_borrow_mut().is_ok(),
            ));
        }
        _ => {}
    }));
    common::assert_no_browser_errors(|| {
        window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .append_child(&element)
            .unwrap()
    });
    assert_eq!(
        *conflicts.borrow(),
        vec![
            ("mutable".to_owned(), false, false),
            ("immutable".to_owned(), true, false)
        ]
    );
    common::assert_no_browser_errors(|| element.remove());
}

#[wasm_bindgen_test]
fn two_instances_keep_separate_state_during_nested_chain() {
    let trace = Rc::new(RefCell::new(Vec::new()));
    let first_state = Rc::new(RefCell::new(0));
    let second_state = Rc::new(RefCell::new(0));
    let second_handle = Rc::new(RefCell::new(None::<HtmlElement>));
    let first_trace = trace.clone();
    let first_value = first_state.clone();
    let first_peer = second_handle.clone();
    let first = create_recorder(Rc::new(move |_element, call| {
        if let Call::Attribute { new, .. } = call {
            let value = new.as_string().unwrap();
            first_trace
                .borrow_mut()
                .push(format!("first:{value}:enter"));
            *first_value.borrow_mut() += 1;
            if value == "start" {
                let peer = first_peer.borrow().as_ref().unwrap().clone();
                peer.set_attribute("observed", "middle").unwrap();
            }
            first_trace
                .borrow_mut()
                .push(format!("first:{value}:return"));
        }
    }));
    let second_trace = trace.clone();
    let second_value = second_state.clone();
    let first_handle = first.clone();
    let second = create_recorder(Rc::new(move |_element, call| {
        if let Call::Attribute { .. } = call {
            second_trace.borrow_mut().push("second:enter".to_owned());
            *second_value.borrow_mut() += 1;
            first_handle.set_attribute("observed", "finish").unwrap();
            second_trace.borrow_mut().push("second:return".to_owned());
        }
    }));
    *second_handle.borrow_mut() = Some(second);
    common::assert_no_browser_errors(|| first.set_attribute("observed", "start").unwrap());
    assert_eq!(
        *trace.borrow(),
        vec![
            "first:start:enter",
            "second:enter",
            "first:finish:enter",
            "first:finish:return",
            "second:return",
            "first:start:return"
        ]
    );
    assert_eq!(*first_state.borrow(), 2);
    assert_eq!(*second_state.borrow(), 1);
}
