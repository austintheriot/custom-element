use std::{cell::RefCell, rc::Rc};

use custom_element::{create_custom_element, Bridge, CustomElement};
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_test::*;
use web_sys::{window, Event, HtmlElement};

#[allow(dead_code)]
mod common;

use common::contracts::{create_recorder, invoke, Call, Hook};

#[wasm_bindgen(module = "/tests/common/browser.js")]
extern "C" {
    #[wasm_bindgen(js_name = moveCapability)]
    fn move_capability() -> bool;
    #[wasm_bindgen(js_name = moveBetweenParents)]
    fn move_between_parents(element: &HtmlElement) -> JsValue;
    #[wasm_bindgen(js_name = injectBridgeFailure)]
    fn inject_bridge_failure(element: &HtmlElement) -> JsValue;
}

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

#[wasm_bindgen_test]
fn observed_attribute_transitions_keep_exact_values() {
    let attributes = Rc::new(RefCell::new(Vec::<(String, JsValue, JsValue)>::new()));
    let recorded = attributes.clone();
    let element = create_recorder(Rc::new(move |_element, call| {
        if let Call::Attribute { name, old, new } = call {
            recorded.borrow_mut().push((name, old, new));
        }
    }));
    let operations = [
        ("", JsValue::NULL, JsValue::from_str("")),
        ("null", JsValue::from_str(""), JsValue::from_str("null")),
        ("null", JsValue::from_str("null"), JsValue::from_str("null")),
    ];
    for (value, expected_old, expected_new) in operations {
        element.set_attribute("observed", value).unwrap();
        let recorded = attributes.borrow();
        let (name, old, new) = recorded.last().unwrap();
        assert_eq!(name, "observed");
        assert!(Object::is(old, &expected_old));
        assert!(Object::is(new, &expected_new));
    }
    element.remove_attribute("observed").unwrap();
    {
        let recorded = attributes.borrow();
        let (name, old, new) = recorded.last().unwrap();
        assert_eq!(name, "observed");
        assert!(Object::is(old, &JsValue::from_str("null")));
        assert!(Object::is(new, &JsValue::NULL));
        assert_eq!(recorded.len(), 4);
    }
    element.set_attribute("unobserved", "ignored").unwrap();
    assert_eq!(attributes.borrow().len(), 4);
    window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&element)
        .unwrap();
    element.remove();
    element.set_attribute("observed", "detached").unwrap();
    let recorded = attributes.borrow();
    assert_eq!(recorded.len(), 5);
    assert!(Object::is(&recorded[4].1, &JsValue::NULL));
    assert!(Object::is(&recorded[4].2, &JsValue::from_str("detached")));
}

#[wasm_bindgen_test]
fn lifecycle_traces_connection_disconnection_adoption_and_reconnection() {
    let trace = Rc::new(RefCell::new(Vec::new()));
    let recorded = trace.clone();
    let element = create_recorder(Rc::new(move |_element, call| {
        let label = match call {
            Call::Connected => "connected",
            Call::Disconnected => "disconnected",
            Call::Adopted => "adopted",
            _ => return,
        };
        recorded.borrow_mut().push(label);
    }));
    let document = window().unwrap().document().unwrap();
    document.body().unwrap().append_child(&element).unwrap();
    element.remove();
    document.body().unwrap().append_child(&element).unwrap();
    let other = document
        .implementation()
        .unwrap()
        .create_html_document()
        .unwrap();
    other.adopt_node(&element).unwrap();
    other.body().unwrap().append_child(&element).unwrap();
    assert_eq!(
        *trace.borrow(),
        vec![
            "connected",
            "disconnected",
            "connected",
            "disconnected",
            "adopted",
            "connected"
        ]
    );
    let connected = trace
        .borrow()
        .iter()
        .filter(|call| **call == "connected")
        .count();
    assert_eq!(connected, 3);
    element.remove();
}

#[wasm_bindgen_test]
fn native_move_reports_capability_and_preserves_connection() {
    let trace = Rc::new(RefCell::new(Vec::new()));
    let recorded = trace.clone();
    let element = create_recorder(Rc::new(move |_element, call| {
        let label = match call {
            Call::Connected => "connected",
            Call::Disconnected => "disconnected",
            Call::Moved => "moved",
            _ => return,
        };
        recorded.borrow_mut().push(label);
    }));
    if !move_capability() {
        wasm_bindgen_test::console_log!("native move capability: unsupported");
        return;
    }
    wasm_bindgen_test::console_log!("native move capability: supported");
    let parents = move_between_parents(&element);
    assert_eq!(*trace.borrow(), vec!["connected", "moved"]);
    let source: HtmlElement = Reflect::get(&parents, &JsValue::from_str("source"))
        .unwrap()
        .dyn_into()
        .unwrap();
    let destination: HtmlElement = Reflect::get(&parents, &JsValue::from_str("destination"))
        .unwrap()
        .dyn_into()
        .unwrap();
    source.remove();
    destination.remove();
}

struct RecordingBridge(Rc<RefCell<Vec<&'static str>>>);

impl CustomElement for RecordingBridge {
    fn connected_callback(&self) {
        self.0.borrow_mut().push("connected");
    }
    fn disconnected_callback(&self) {
        self.0.borrow_mut().push("disconnected");
    }
    fn adopted_callback(&self) {
        self.0.borrow_mut().push("adopted");
    }
    fn connected_move_callback(&self) {
        self.0.borrow_mut().push("moved");
    }
    fn attribute_changed_callback(&self, _name: &str, _old: JsValue, _new: JsValue) {
        self.0.borrow_mut().push("attribute");
    }
    fn handle_event(&self, _event: Event) {
        self.0.borrow_mut().push("event");
    }
}

#[wasm_bindgen_test]
fn immutable_bridge_construction_paths_forward_all_methods() {
    let first = Rc::new(RefCell::new(Vec::new()));
    let second = Rc::new(RefCell::new(Vec::new()));
    for (bridge, calls) in [
        (Bridge::new(RecordingBridge(first.clone())), first),
        (Bridge::from(RecordingBridge(second.clone())), second),
    ] {
        let bridge: JsValue = bridge.into();
        let attribute = Array::new();
        attribute.push(&JsValue::from_str("observed"));
        attribute.push(&JsValue::NULL);
        attribute.push(&JsValue::from_str("value"));
        let event = Array::new();
        event.push(Event::new("probe").unwrap().as_ref());
        for (method, arguments) in [
            ("connectedCallback", Array::new()),
            ("disconnectedCallback", Array::new()),
            ("adoptedCallback", Array::new()),
            ("connectedMoveCallback", Array::new()),
            ("attributeChangedCallback", attribute),
            ("handleEvent", event),
        ] {
            invoke(&bridge, method, &arguments).unwrap();
        }
        assert_eq!(
            *calls.borrow(),
            vec![
                "connected",
                "disconnected",
                "adopted",
                "moved",
                "attribute",
                "event"
            ]
        );
    }
}

#[wasm_bindgen_test]
fn controlled_bridge_error_reports_one_attempt() {
    let element = create_recorder(Rc::new(move |_element, _call| {}));
    let result = inject_bridge_failure(&element);
    assert_eq!(
        Reflect::get(&result, &JsValue::from_str("attempts"))
            .unwrap()
            .as_f64(),
        Some(1.0)
    );
    assert_eq!(
        Reflect::get(&result, &JsValue::from_str("reported"))
            .unwrap()
            .as_f64(),
        Some(1.0)
    );
    invoke(element.as_ref(), "connectedCallback", &Array::new()).unwrap();
}
