#![cfg(target_arch = "wasm32")]

use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use custom_element::{
    constructors::{html_element_constructor, html_paragraph_element_constructor},
    create_custom_element, create_custom_element_extending, CustomElement,
};
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{
    window, ElementCreationOptions, ElementDefinitionOptions, HtmlElement, HtmlParagraphElement,
};

mod common;

use common::browser::{is_failed_element, next_turn, plain_upgrade_recursion};

wasm_bindgen_test_configure!(run_in_browser);

struct Empty;

impl CustomElement for Empty {}

#[wasm_bindgen_test]
fn factory_can_create_another_registered_element_before_returning() {
    let name = common::names::unique_custom_element_name();
    let calls = Rc::new(Cell::new(0));
    let instances = Rc::new(RefCell::new(Vec::<JsValue>::new()));
    let recorded_calls = calls.clone();
    let recorded_instances = instances.clone();
    let nested_name = name.clone();
    let (closure, constructor) = create_custom_element(move |instance, _arguments: Array| {
        let next_call = recorded_calls.get() + 1;
        recorded_calls.set(next_call);
        recorded_instances.borrow_mut().push(instance.into());
        if next_call == 1 {
            let _nested = window()
                .unwrap()
                .document()
                .unwrap()
                .create_element(&nested_name)
                .unwrap();
        }
        Empty
    });
    closure.forget();
    common::browser::assert_no_browser_errors(|| {
        window()
            .unwrap()
            .custom_elements()
            .define(&name, &constructor)
            .unwrap()
    });

    let outer = common::browser::assert_no_browser_errors(|| {
        window()
            .unwrap()
            .document()
            .unwrap()
            .create_element(&name)
            .unwrap()
    });

    assert_eq!(calls.get(), 2);
    let recorded = instances.borrow();
    assert_eq!(recorded.len(), 2);
    assert!(Object::is(outer.as_ref(), &recorded[0]));
    assert!(!Object::is(&recorded[0], &recorded[1]));
}

struct ConnectionCounter(Rc<Cell<u32>>);

impl CustomElement for ConnectionCounter {
    fn connected_callback(&self) {
        self.0.set(self.0.get() + 1);
    }
}

#[wasm_bindgen_test]
fn fresh_construction_inside_a_factory_builds_a_complete_element() {
    let name = common::names::unique_custom_element_name();
    let connections = Rc::new(Cell::new(0));
    let calls = Rc::new(Cell::new(0));
    let nested = Rc::new(RefCell::new(None));
    let recorded_connections = connections.clone();
    let recorded_calls = calls.clone();
    let recorded_nested = nested.clone();
    let nested_name = name.clone();
    let (closure, constructor) = create_custom_element(move |_element, _arguments| {
        recorded_calls.set(recorded_calls.get() + 1);
        if recorded_calls.get() == 1 {
            let element = window()
                .unwrap()
                .document()
                .unwrap()
                .create_element(&nested_name)
                .unwrap();
            *recorded_nested.borrow_mut() = Some(element);
        }
        ConnectionCounter(recorded_connections.clone())
    });
    closure.forget();
    common::browser::assert_no_browser_errors(|| {
        window()
            .unwrap()
            .custom_elements()
            .define(&name, &constructor)
            .unwrap()
    });
    let document = window().unwrap().document().unwrap();

    let outer =
        common::browser::assert_no_browser_errors(|| document.create_element(&name).unwrap());

    assert_eq!(calls.get(), 2);
    let nested = nested.borrow_mut().take().unwrap();
    assert!(!is_failed_element(nested.as_ref()));
    assert!(!Object::is(outer.as_ref(), nested.as_ref()));
    common::browser::assert_no_browser_errors(|| {
        document.body().unwrap().append_child(&nested).unwrap()
    });
    assert_eq!(connections.get(), 1);
    common::browser::assert_no_browser_errors(|| nested.remove());
}

#[wasm_bindgen_test]
fn recursive_factory_releases_state_before_nested_construction() {
    let state = Rc::new(RefCell::new(Vec::new()));
    let recorded = state.clone();
    let name = common::names::unique_custom_element_name();
    let nested_name = name.clone();
    let (closure, constructor) = create_custom_element(move |_element, _arguments| {
        let index = {
            let mut entries = recorded.borrow_mut();
            entries.push("enter");
            entries.len()
        };
        if index == 1 {
            window()
                .unwrap()
                .document()
                .unwrap()
                .create_element(&nested_name)
                .unwrap();
        }
        recorded.borrow_mut().push("return");
        Empty
    });
    closure.forget();
    common::browser::assert_no_browser_errors(|| {
        window()
            .unwrap()
            .custom_elements()
            .define(&name, &constructor)
            .unwrap()
    });
    common::browser::assert_no_browser_errors(|| {
        window()
            .unwrap()
            .document()
            .unwrap()
            .create_element(&name)
            .unwrap()
    });
    assert_eq!(*state.borrow(), vec!["enter", "enter", "return", "return"]);
}

#[wasm_bindgen_test]
fn arguments_preserve_order_and_identity() {
    let seen = Rc::new(RefCell::new(Vec::<(JsValue, Array)>::new()));
    let recorded = seen.clone();
    let (closure, constructor) = create_custom_element(move |element, arguments| {
        recorded.borrow_mut().push((element.into(), arguments));
        Empty
    });
    let name = common::names::unique_custom_element_name();
    common::browser::assert_no_browser_errors(|| {
        window()
            .unwrap()
            .custom_elements()
            .define(&name, &constructor)
            .unwrap()
    });
    let object = Object::new();
    let arguments = Array::new();
    arguments.push(&JsValue::from_f64(17.0));
    arguments.push(&JsValue::NULL);
    arguments.push(&object);
    let element = common::browser::assert_no_browser_errors(|| {
        Reflect::construct(&constructor, &arguments).unwrap()
    });
    let empty = common::browser::assert_no_browser_errors(|| {
        Reflect::construct(&constructor, &Array::new()).unwrap()
    });
    let seen = seen.borrow();
    assert_eq!(seen.len(), 2);
    assert!(Object::is(&seen[0].0, &element));
    assert_eq!(seen[0].1.length(), 3);
    assert_eq!(seen[0].1.get(0).as_f64(), Some(17.0));
    assert!(seen[0].1.get(1).is_null());
    assert!(Object::is(&seen[0].1.get(2), &object));
    assert!(Object::is(&seen[1].0, &empty));
    assert_eq!(seen[1].1.length(), 0);
    drop(closure);
}

#[wasm_bindgen_test]
fn extending_html_element_constructs_an_autonomous_element() {
    let (closure, constructor) =
        create_custom_element_extending(&html_element_constructor(), |_element, _arguments| Empty);
    let name = common::names::unique_custom_element_name();
    common::browser::assert_no_browser_errors(|| {
        window()
            .unwrap()
            .custom_elements()
            .define(&name, &constructor)
            .unwrap()
    });
    let element = common::browser::assert_no_browser_errors(|| {
        Reflect::construct(&constructor, &Array::new()).unwrap()
    });
    assert!(element.is_instance_of::<HtmlElement>());
    drop(closure);
}

#[wasm_bindgen_test]
fn extending_paragraph_uses_correct_creation_path() {
    let connections = Rc::new(Cell::new(0));
    let recorded = connections.clone();
    struct ParagraphState(Rc<Cell<u32>>);
    impl CustomElement for ParagraphState {
        fn connected_callback(&self) {
            self.0.set(self.0.get() + 1);
        }
    }
    let (closure, constructor) = create_custom_element_extending(
        &html_paragraph_element_constructor(),
        move |_element, _arguments| ParagraphState(recorded.clone()),
    );
    closure.forget();
    let name = common::names::unique_custom_element_name();
    let options = ElementDefinitionOptions::new();
    options.set_extends("p");
    window()
        .unwrap()
        .custom_elements()
        .define_with_options(&name, &constructor, &options)
        .unwrap();
    let creation_options = ElementCreationOptions::new();
    creation_options.set_is(&name);
    let element = common::browser::assert_no_browser_errors(|| {
        window()
            .unwrap()
            .document()
            .unwrap()
            .create_element_with_element_creation_options("p", &creation_options)
            .unwrap()
    });
    assert!(element.is_instance_of::<HtmlParagraphElement>());
    assert_eq!(element.local_name(), "p");
    common::browser::assert_no_browser_errors(|| {
        window()
            .unwrap()
            .document()
            .unwrap()
            .body()
            .unwrap()
            .append_child(&element)
            .unwrap()
    });
    assert_eq!(connections.get(), 1);
    common::browser::assert_no_browser_errors(|| element.remove());
}

#[wasm_bindgen_test]
async fn factory_remains_callable_across_turns() {
    let calls = Rc::new(Cell::new(0));
    let recorded = calls.clone();
    let (closure, constructor) = create_custom_element(move |_element, _arguments| {
        recorded.set(recorded.get() + 1);
        Empty
    });
    let name = common::names::unique_custom_element_name();
    common::browser::assert_no_browser_errors(|| {
        window()
            .unwrap()
            .custom_elements()
            .define(&name, &constructor)
            .unwrap()
    });
    let first = common::browser::assert_no_browser_errors(|| {
        Reflect::construct(&constructor, &Array::new()).unwrap()
    });
    JsFuture::from(next_turn()).await.unwrap();
    let second = common::browser::assert_no_browser_errors(|| {
        Reflect::construct(&constructor, &Array::new()).unwrap()
    });
    assert_eq!(calls.get(), 2);
    assert!(!Object::is(&first, &second));
    drop(closure);
}

#[wasm_bindgen_test]
fn dropping_factory_fails_visibly() {
    let calls = Rc::new(Cell::new(0));
    let recorded = calls.clone();
    let (closure, constructor) = create_custom_element(move |_element, _arguments| {
        recorded.set(recorded.get() + 1);
        Empty
    });
    let name = common::names::unique_custom_element_name();
    common::browser::assert_no_browser_errors(|| {
        window()
            .unwrap()
            .custom_elements()
            .define(&name, &constructor)
            .unwrap()
    });
    common::browser::assert_no_browser_errors(|| {
        Reflect::construct(&constructor, &Array::new()).unwrap()
    });
    assert_eq!(calls.get(), 1);
    drop(closure);
    assert!(Reflect::construct(&constructor, &Array::new()).is_err());
    assert_eq!(calls.get(), 1);
}

#[wasm_bindgen_test]
fn factory_state_is_local_to_registration() {
    let register = || {
        let sequence = Rc::new(Cell::new(0));
        let recorded = Rc::new(RefCell::new(Vec::new()));
        let next = sequence.clone();
        let output = recorded.clone();
        let (closure, constructor) = create_custom_element(move |_element, _arguments| {
            next.set(next.get() + 1);
            output.borrow_mut().push(next.get());
            Empty
        });
        let name = common::names::unique_custom_element_name();
        common::browser::assert_no_browser_errors(|| {
            window()
                .unwrap()
                .custom_elements()
                .define(&name, &constructor)
                .unwrap()
        });
        (closure, constructor, recorded)
    };
    let (first_closure, first, first_calls) = register();
    let (second_closure, second, second_calls) = register();
    for _ in 0..2 {
        common::browser::assert_no_browser_errors(|| {
            Reflect::construct(&first, &Array::new()).unwrap()
        });
        common::browser::assert_no_browser_errors(|| {
            Reflect::construct(&second, &Array::new()).unwrap()
        });
    }
    assert_eq!(*first_calls.borrow(), vec![1, 2]);
    assert_eq!(*second_calls.borrow(), vec![1, 2]);
    drop((first_closure, second_closure));
}

struct PrivateState {
    value: Cell<u32>,
    observed: Rc<RefCell<Vec<u32>>>,
}

impl CustomElement for PrivateState {
    fn connected_callback(&self) {
        self.value.set(self.value.get() + 1);
        self.observed.borrow_mut().push(self.value.get());
    }
}

#[wasm_bindgen_test]
fn cloned_element_has_separate_component() {
    let calls = Rc::new(Cell::new(0));
    let observed = Rc::new(RefCell::new(Vec::new()));
    let recorded_calls = calls.clone();
    let recorded = observed.clone();
    let (closure, constructor) = create_custom_element(move |_element, _arguments| {
        recorded_calls.set(recorded_calls.get() + 1);
        PrivateState {
            value: Cell::new(0),
            observed: recorded.clone(),
        }
    });
    closure.forget();
    let name = common::names::unique_custom_element_name();
    common::browser::assert_no_browser_errors(|| {
        window()
            .unwrap()
            .custom_elements()
            .define(&name, &constructor)
            .unwrap()
    });
    let document = window().unwrap().document().unwrap();
    let original =
        common::browser::assert_no_browser_errors(|| document.create_element(&name).unwrap());
    let clone = original
        .clone_node()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    assert_eq!(calls.get(), 2);
    assert!(!Object::is(original.as_ref(), clone.as_ref()));
    common::browser::assert_no_browser_errors(|| {
        document.body().unwrap().append_child(&original).unwrap()
    });
    common::browser::assert_no_browser_errors(|| {
        document.body().unwrap().append_child(&clone).unwrap()
    });
    common::browser::assert_no_browser_errors(|| original.remove());
    common::browser::assert_no_browser_errors(|| {
        document.body().unwrap().append_child(&original).unwrap()
    });
    assert_eq!(*observed.borrow(), vec![1, 1, 2]);
    common::browser::assert_no_browser_errors(|| original.remove());
    common::browser::assert_no_browser_errors(|| clone.remove());
}

struct UpgradeRecorder(Rc<RefCell<Vec<&'static str>>>);

impl CustomElement for UpgradeRecorder {
    fn observed_attributes() -> &'static [&'static str] {
        &["observed"]
    }

    fn attribute_changed_callback(&self, _name: &str, _old: Option<String>, _new: Option<String>) {
        self.0.borrow_mut().push("attribute");
    }
    fn connected_callback(&self) {
        self.0.borrow_mut().push("connected");
    }
}

#[wasm_bindgen_test]
fn upgrade_initializes_bridge_before_callbacks() {
    let document = window().unwrap().document().unwrap();
    let name = common::names::unique_custom_element_name();
    let element =
        common::browser::assert_no_browser_errors(|| document.create_element(&name).unwrap());
    common::browser::assert_no_browser_errors(|| {
        element.set_attribute("observed", "existing").unwrap()
    });
    common::browser::assert_no_browser_errors(|| {
        document.body().unwrap().append_child(&element).unwrap()
    });
    let trace = Rc::new(RefCell::new(Vec::new()));
    let recorded = trace.clone();
    let (closure, constructor) = create_custom_element(move |_element, _arguments| {
        recorded.borrow_mut().push("factory");
        UpgradeRecorder(recorded.clone())
    });
    common::browser::assert_no_browser_errors(|| {
        window()
            .unwrap()
            .custom_elements()
            .define(&name, &constructor)
            .unwrap()
    });
    assert_eq!(*trace.borrow(), vec!["factory", "attribute", "connected"]);
    common::browser::assert_no_browser_errors(|| element.remove());
    drop(closure);
}

#[wasm_bindgen_test]
fn upgrade_recursion_preserves_browser_restriction() {
    let baseline_name = common::names::unique_custom_element_name();
    let baseline = plain_upgrade_recursion(&baseline_name);
    let baseline_count = Reflect::get(&baseline, &JsValue::from_str("bodyCount"))
        .unwrap()
        .as_f64()
        .unwrap();
    assert_eq!(baseline_count, 1.0);
    let baseline_errors: Array = Reflect::get(&baseline, &JsValue::from_str("errors"))
        .unwrap()
        .dyn_into()
        .unwrap();
    assert_type_error(&baseline_errors);
    assert_eq!(
        Reflect::get(&baseline, &JsValue::from_str("failedNested"))
            .unwrap()
            .as_bool(),
        Some(true)
    );

    let name = common::names::unique_custom_element_name();
    let document = window().unwrap().document().unwrap();
    let element = document.create_element(&name).unwrap();
    document.body().unwrap().append_child(&element).unwrap();
    let count = Rc::new(Cell::new(0));
    let nested = Rc::new(RefCell::new(None));
    let recorded_count = count.clone();
    let recorded_nested = nested.clone();
    let nested_name = name.clone();
    let (closure, constructor) = create_custom_element(move |_element, _arguments| {
        let current = recorded_count.get() + 1;
        recorded_count.set(current);
        if current == 1 {
            let element = window()
                .unwrap()
                .document()
                .unwrap()
                .create_element(&nested_name)
                .unwrap();
            *recorded_nested.borrow_mut() = Some(element);
        }
        Empty
    });
    let (_, errors) = common::browser::capture_browser_errors(|| {
        window()
            .unwrap()
            .custom_elements()
            .define(&name, &constructor)
            .unwrap();
    });
    assert_type_error(&errors);
    assert_eq!(count.get(), 1);
    assert!(is_failed_element(
        nested.borrow().as_ref().unwrap().as_ref()
    ));
    common::browser::assert_no_browser_errors(|| element.remove());
    drop(closure);
}

fn assert_type_error(errors: &Array) {
    assert_eq!(errors.length(), 1);
    assert_eq!(
        Reflect::get(&errors.get(0), &JsValue::from_str("name"))
            .unwrap()
            .as_string()
            .as_deref(),
        Some("TypeError")
    );
}

#[wasm_bindgen_test]
fn factory_receives_the_element_as_html_element() {
    let observed_name = Rc::new(RefCell::new(String::new()));
    let recorded_name = observed_name.clone();
    let name = common::names::unique_custom_element_name();
    let (closure, constructor) = create_custom_element(move |element: HtmlElement, _arguments| {
        recorded_name.replace(element.local_name());
        Empty
    });
    closure.forget();
    window()
        .unwrap()
        .custom_elements()
        .define(&name, &constructor)
        .unwrap();
    window()
        .unwrap()
        .document()
        .unwrap()
        .create_element(&name)
        .unwrap();
    assert_eq!(*observed_name.borrow(), name);
}
