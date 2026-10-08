use std::{cell::RefCell, rc::Rc};

use js_sys::{Array, Object, Reflect};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_test::*;
use web_sys::{window, Event, HtmlElement};

#[allow(dead_code)]
mod common;

use common::contracts::{create_recorder, Call};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen(module = "/tests/common/browser.js")]
extern "C" {
    #[wasm_bindgen(js_name = dispatchProbe)]
    fn dispatch_probe(element: &HtmlElement, cancelable: bool) -> JsValue;
    #[wasm_bindgen(js_name = prepareProbe)]
    fn prepare_probe(element: &HtmlElement) -> JsValue;
    #[wasm_bindgen(js_name = shadowProbe)]
    fn shadow_probe(host: &HtmlElement) -> JsValue;
}

fn field(value: &JsValue, name: &str) -> JsValue {
    Reflect::get(value, &JsValue::from_str(name)).unwrap()
}

#[derive(Clone, Copy)]
enum Action {
    CancelAndImmediateStop,
    Stop,
    Continue,
    CancelNoncancelable,
}

fn assert_dispatch(action: Action, accepted: bool, target_count: u32, ancestor_count: u32) {
    let observations = Rc::new(RefCell::new(Vec::<(Event, JsValue, u16, Array)>::new()));
    let dispatch_result = Rc::new(RefCell::new(None::<JsValue>));
    let observed = observations.clone();
    let result_for_hook = dispatch_result.clone();
    let element = create_recorder(Rc::new(move |element, call| match call {
        Call::Connected => {
            *result_for_hook.borrow_mut() = Some(dispatch_probe(
                element,
                !matches!(action, Action::CancelNoncancelable),
            ));
        }
        Call::Event(event) => {
            observed.borrow_mut().push((
                event.clone(),
                event.current_target().map_or(JsValue::NULL, JsValue::from),
                event.event_phase(),
                event.composed_path(),
            ));
            match action {
                Action::CancelAndImmediateStop => {
                    event.prevent_default();
                    event.stop_immediate_propagation();
                }
                Action::Stop => event.stop_propagation(),
                Action::Continue => {}
                Action::CancelNoncancelable => event.prevent_default(),
            }
        }
        _ => {}
    }));
    let setup = prepare_probe(&element);
    let ancestor: HtmlElement = field(&setup, "ancestor").dyn_into().unwrap();
    window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&ancestor)
        .unwrap();
    let result = dispatch_result.borrow();
    let result = result.as_ref().unwrap();
    assert_eq!(field(result, "accepted").as_bool(), Some(accepted));
    let counts = field(&setup, "counts");
    assert_eq!(
        field(&counts, "target").as_f64(),
        Some(f64::from(target_count))
    );
    assert_eq!(
        field(&counts, "ancestor").as_f64(),
        Some(f64::from(ancestor_count))
    );
    let observed = observations.borrow();
    assert_eq!(observed.len(), 1);
    let (event, current_target, phase, path) = &observed[0];
    assert!(Object::is(event.as_ref(), &field(result, "event")));
    assert!(Object::is(current_target, element.as_ref()));
    assert_eq!(*phase, Event::AT_TARGET);
    assert!(path.includes(element.as_ref(), 0));
    assert!(path.includes(ancestor.as_ref(), 0));
    assert!(field(result, "currentTarget").is_null());
    assert_eq!(field(result, "phase").as_f64(), Some(0.0));
    assert_eq!(
        field(result, "path").dyn_into::<Array>().unwrap().length(),
        0
    );
    ancestor.remove();
}

#[wasm_bindgen_test]
fn nested_event_cancellation_preserves_dispatch_context() {
    assert_dispatch(Action::CancelAndImmediateStop, false, 0, 0);
}

#[wasm_bindgen_test]
fn ordinary_stop_allows_later_target_listener() {
    assert_dispatch(Action::Stop, true, 1, 0);
}

#[wasm_bindgen_test]
fn unmodified_dispatch_reaches_ancestor() {
    assert_dispatch(Action::Continue, true, 1, 1);
}

#[wasm_bindgen_test]
fn noncancelable_event_ignores_cancellation() {
    assert_dispatch(Action::CancelNoncancelable, true, 1, 1);
}

#[wasm_bindgen_test]
fn composed_event_retargets_at_shadow_host() {
    let seen = Rc::new(RefCell::new(Vec::<(Event, JsValue, JsValue, Array)>::new()));
    let recorded = seen.clone();
    let host = create_recorder(Rc::new(move |_element, call| {
        if let Call::Event(event) = call {
            recorded.borrow_mut().push((
                event.clone(),
                event.target().map_or(JsValue::NULL, JsValue::from),
                event.current_target().map_or(JsValue::NULL, JsValue::from),
                event.composed_path(),
            ));
        }
    }));
    let result = shadow_probe(&host);
    let button = field(&result, "button");
    let seen = seen.borrow();
    assert_eq!(seen.len(), 1);
    let (event, target, current_target, path) = &seen[0];
    assert!(Object::is(event.as_ref(), &field(&result, "event")));
    assert!(Object::is(target, host.as_ref()));
    assert!(Object::is(current_target, host.as_ref()));
    assert!(path.includes(&button, 0));
    assert!(path.includes(host.as_ref(), 0));
}
