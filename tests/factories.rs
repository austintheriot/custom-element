use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};

use custom_element::{
    constructors::{HTML_ELEMENT_CONSTRUCTOR, HTML_PARAGRAPH_ELEMENT_CONSTRUCTOR},
    create_custom_element, create_custom_element_with_config, CustomElement,
    CustomElementConfiguration,
};
use js_sys::{Array, Object, Reflect};
use wasm_bindgen::{prelude::*, JsCast};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;
use web_sys::{
    window, ElementCreationOptions, ElementDefinitionOptions, HtmlElement, HtmlParagraphElement,
};

#[allow(dead_code)]
mod common;

use common::next_turn;

#[wasm_bindgen(module = "/tests/common/browser.js")]
extern "C" {
    #[wasm_bindgen(js_name = plainUpgradeRecursion)]
    fn plain_upgrade_recursion(name: &str) -> JsValue;
}

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

#[wasm_bindgen_test]
fn recursive_factory_releases_state_before_nested_construction() {
    let state = Rc::new(RefCell::new(Vec::new()));
    let recorded = state.clone();
    let name = common::helpers::get_random_custom_element_name();
    let nested_name = name.clone();
    let (factory, constructor) = create_custom_element(
        move |_element, _arguments| {
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
        },
        vec![],
    );
    factory.forget();
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
    assert_eq!(*state.borrow(), vec!["enter", "enter", "return", "return"]);
}

#[wasm_bindgen_test]
fn arguments_preserve_order_and_identity() {
    let seen = Rc::new(RefCell::new(Vec::<(JsValue, Array)>::new()));
    let recorded = seen.clone();
    let (factory, constructor) = create_custom_element(
        move |element, arguments| {
            recorded.borrow_mut().push((element, arguments));
            Empty
        },
        vec![],
    );
    let name = common::helpers::get_random_custom_element_name();
    window()
        .unwrap()
        .custom_elements()
        .define(&name, &constructor)
        .unwrap();
    let object = Object::new();
    let arguments = Array::new();
    arguments.push(&JsValue::from_f64(17.0));
    arguments.push(&JsValue::NULL);
    arguments.push(&object);
    let element = Reflect::construct(&constructor, &arguments).unwrap();
    let empty = Reflect::construct(&constructor, &Array::new()).unwrap();
    let seen = seen.borrow();
    assert_eq!(seen.len(), 2);
    assert!(Object::is(&seen[0].0, &element));
    assert_eq!(seen[0].1.length(), 3);
    assert_eq!(seen[0].1.get(0).as_f64(), Some(17.0));
    assert!(seen[0].1.get(1).is_null());
    assert!(Object::is(&seen[0].1.get(2), &object));
    assert!(Object::is(&seen[1].0, &empty));
    assert_eq!(seen[1].1.length(), 0);
    drop(factory);
}

#[wasm_bindgen_test]
fn explicit_autonomous_configuration_constructs() {
    let (factory, constructor) = create_custom_element_with_config(
        |_element, _arguments| Empty,
        vec![],
        CustomElementConfiguration {
            element_constructor: &HTML_ELEMENT_CONSTRUCTOR,
        },
    );
    let name = common::helpers::get_random_custom_element_name();
    window()
        .unwrap()
        .custom_elements()
        .define(&name, &constructor)
        .unwrap();
    let element = Reflect::construct(&constructor, &Array::new()).unwrap();
    assert!(element.is_instance_of::<HtmlElement>());
    drop(factory);
}

#[wasm_bindgen_test]
fn configured_paragraph_uses_correct_creation_path() {
    let connections = Rc::new(Cell::new(0));
    let recorded = connections.clone();
    struct ParagraphState(Rc<Cell<u32>>);
    impl CustomElement for ParagraphState {
        fn connected_callback(&self) {
            self.0.set(self.0.get() + 1);
        }
    }
    let (factory, constructor) = create_custom_element_with_config(
        move |_element, _arguments| ParagraphState(recorded.clone()),
        vec![],
        CustomElementConfiguration {
            element_constructor: &HTML_PARAGRAPH_ELEMENT_CONSTRUCTOR,
        },
    );
    factory.forget();
    let name = common::helpers::get_random_custom_element_name();
    let options = ElementDefinitionOptions::new();
    options.set_extends("p");
    window()
        .unwrap()
        .custom_elements()
        .define_with_options(&name, &constructor, &options)
        .unwrap();
    let creation_options = ElementCreationOptions::new();
    creation_options.set_is(&name);
    let element = window()
        .unwrap()
        .document()
        .unwrap()
        .create_element_with_element_creation_options("p", &creation_options)
        .unwrap();
    assert!(element.is_instance_of::<HtmlParagraphElement>());
    assert_eq!(element.local_name(), "p");
    window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&element)
        .unwrap();
    assert_eq!(connections.get(), 1);
    element.remove();
}

#[wasm_bindgen_test]
async fn factory_remains_callable_across_turns() {
    let calls = Rc::new(Cell::new(0));
    let recorded = calls.clone();
    let (factory, constructor) = create_custom_element(
        move |_element, _arguments| {
            recorded.set(recorded.get() + 1);
            Empty
        },
        vec![],
    );
    let name = common::helpers::get_random_custom_element_name();
    window()
        .unwrap()
        .custom_elements()
        .define(&name, &constructor)
        .unwrap();
    let first = Reflect::construct(&constructor, &Array::new()).unwrap();
    JsFuture::from(next_turn()).await.unwrap();
    let second = Reflect::construct(&constructor, &Array::new()).unwrap();
    assert_eq!(calls.get(), 2);
    assert!(!Object::is(&first, &second));
    drop(factory);
}

#[wasm_bindgen_test]
fn dropping_factory_fails_visibly() {
    let calls = Rc::new(Cell::new(0));
    let recorded = calls.clone();
    let (factory, constructor) = create_custom_element(
        move |_element, _arguments| {
            recorded.set(recorded.get() + 1);
            Empty
        },
        vec![],
    );
    let name = common::helpers::get_random_custom_element_name();
    window()
        .unwrap()
        .custom_elements()
        .define(&name, &constructor)
        .unwrap();
    Reflect::construct(&constructor, &Array::new()).unwrap();
    assert_eq!(calls.get(), 1);
    drop(factory);
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
        let (factory, constructor) = create_custom_element(
            move |_element, _arguments| {
                next.set(next.get() + 1);
                output.borrow_mut().push(next.get());
                Empty
            },
            vec![],
        );
        let name = common::helpers::get_random_custom_element_name();
        window()
            .unwrap()
            .custom_elements()
            .define(&name, &constructor)
            .unwrap();
        (factory, constructor, recorded)
    };
    let (first_factory, first, first_calls) = register();
    let (second_factory, second, second_calls) = register();
    for _ in 0..2 {
        Reflect::construct(&first, &Array::new()).unwrap();
        Reflect::construct(&second, &Array::new()).unwrap();
    }
    assert_eq!(*first_calls.borrow(), vec![1, 2]);
    assert_eq!(*second_calls.borrow(), vec![1, 2]);
    drop((first_factory, second_factory));
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
    let (factory, constructor) = create_custom_element(
        move |_element, _arguments| {
            recorded_calls.set(recorded_calls.get() + 1);
            PrivateState {
                value: Cell::new(0),
                observed: recorded.clone(),
            }
        },
        vec![],
    );
    factory.forget();
    let name = common::helpers::get_random_custom_element_name();
    window()
        .unwrap()
        .custom_elements()
        .define(&name, &constructor)
        .unwrap();
    let document = window().unwrap().document().unwrap();
    let original = document.create_element(&name).unwrap();
    let clone = original
        .clone_node()
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();
    assert_eq!(calls.get(), 2);
    assert!(!Object::is(original.as_ref(), clone.as_ref()));
    document.body().unwrap().append_child(&original).unwrap();
    document.body().unwrap().append_child(&clone).unwrap();
    original.remove();
    document.body().unwrap().append_child(&original).unwrap();
    assert_eq!(*observed.borrow(), vec![1, 1, 2]);
    original.remove();
    clone.remove();
}

struct UpgradeRecorder(Rc<RefCell<Vec<&'static str>>>);

impl CustomElement for UpgradeRecorder {
    fn attribute_changed_callback(&self, _name: &str, _old: JsValue, _new: JsValue) {
        self.0.borrow_mut().push("attribute");
    }
    fn connected_callback(&self) {
        self.0.borrow_mut().push("connected");
    }
}

#[wasm_bindgen_test]
fn upgrade_initializes_bridge_before_callbacks() {
    let document = window().unwrap().document().unwrap();
    let name = common::helpers::get_random_custom_element_name();
    let element = document.create_element(&name).unwrap();
    element.set_attribute("observed", "existing").unwrap();
    document.body().unwrap().append_child(&element).unwrap();
    let trace = Rc::new(RefCell::new(Vec::new()));
    let recorded = trace.clone();
    let (factory, constructor) = create_custom_element(
        move |_element, _arguments| {
            recorded.borrow_mut().push("factory");
            UpgradeRecorder(recorded.clone())
        },
        vec!["observed".to_owned()],
    );
    window()
        .unwrap()
        .custom_elements()
        .define(&name, &constructor)
        .unwrap();
    assert_eq!(*trace.borrow(), vec!["factory", "attribute", "connected"]);
    element.remove();
    drop(factory);
}

#[wasm_bindgen_test]
fn upgrade_recursion_preserves_browser_restriction() {
    let baseline_name = common::helpers::get_random_custom_element_name();
    let baseline = plain_upgrade_recursion(&baseline_name);
    let baseline_count = Reflect::get(&baseline, &JsValue::from_str("bodyCount"))
        .unwrap()
        .as_f64()
        .unwrap();
    let baseline_error = Reflect::get(&baseline, &JsValue::from_str("errorName")).unwrap();

    let name = common::helpers::get_random_custom_element_name();
    let document = window().unwrap().document().unwrap();
    let element = document.create_element(&name).unwrap();
    document.body().unwrap().append_child(&element).unwrap();
    let count = Rc::new(RefCell::new(0));
    let error_name = Rc::new(RefCell::new(JsValue::NULL));
    let recorded_count = count.clone();
    let recorded_error = error_name.clone();
    let nested_name = name.clone();
    let (factory, constructor) = create_custom_element(
        move |_element, _arguments| {
            let current = {
                let mut guard = recorded_count.borrow_mut();
                *guard += 1;
                *guard
            };
            if current == 1 {
                if let Err(error) = window()
                    .unwrap()
                    .document()
                    .unwrap()
                    .create_element(&nested_name)
                {
                    *recorded_error.borrow_mut() =
                        Reflect::get(&error, &JsValue::from_str("name")).unwrap();
                }
            }
            Empty
        },
        vec![],
    );
    window()
        .unwrap()
        .custom_elements()
        .define(&name, &constructor)
        .unwrap();
    assert_eq!(f64::from(*count.borrow()), baseline_count);
    assert!(Object::is(&error_name.borrow(), &baseline_error));
    element.remove();
    drop(factory);
}
