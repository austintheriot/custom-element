use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

mod common;

pub mod autonomous_custom_elements {
    use crate::common::helpers;
    use js_sys::{Array, Reflect};
    use wasm_bindgen_test::*;
    use web_sys::{window, ElementDefinitionOptions};

    #[wasm_bindgen_test]
    fn should_create_instance_from_constructor() {
        let window = window().unwrap();
        let constructor = helpers::create_autonomous_custom_element_constructor();
        let name = helpers::get_random_custom_element_name();
        window
            .custom_elements()
            .define(&name, &constructor)
            .unwrap();

        let element_from_constructor = Reflect::construct(&constructor, &Array::new()).unwrap();

        helpers::assert_is_custom_element_instance(&element_from_constructor)
    }

    #[wasm_bindgen_test]
    fn should_create_instance_from_document_create() {
        let window = window().unwrap();
        let constructor = helpers::create_autonomous_custom_element_constructor();
        let document = window.document().unwrap();
        let name = helpers::get_random_custom_element_name();
        let mut options = ElementDefinitionOptions::new();
        options.extends("p");
        window
            .custom_elements()
            .define(&name, &constructor)
            .unwrap();

        let instance = document.create_element(&name).unwrap();

        helpers::assert_is_custom_element_instance(&instance)
    }
}

pub mod customized_built_in_custom_elements {
    use crate::common::helpers;
    use custom_element::{
        constructors::HTML_PARAGRAPH_ELEMENT_CONSTRUCTOR, create_custom_element_with_config,
        CustomElement, CustomElementConfiguration,
    };
    use js_sys::{Array, Reflect};
    use std::{cell::Cell, rc::Rc};
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;
    use web_sys::{window, ElementCreationOptions, ElementDefinitionOptions, HtmlParagraphElement};

    struct ConnectedElement {
        connections: Rc<Cell<u32>>,
    }

    impl CustomElement for ConnectedElement {
        fn connected_callback(&mut self) {
            self.connections.set(self.connections.get() + 1);
        }
    }

    #[wasm_bindgen_test]
    fn should_create_instance_from_constructor() {
        let window = window().unwrap();
        let constructor = helpers::create_customized_built_in_element_constructor();
        let name = helpers::get_random_custom_element_name();
        let mut options = ElementDefinitionOptions::new();
        options.extends("p");
        window
            .custom_elements()
            .define_with_options(&name, &constructor, &options)
            .unwrap();

        let element_from_constructor = Reflect::construct(&constructor, &Array::new()).unwrap();

        helpers::assert_is_custom_element_instance(&element_from_constructor)
    }

    #[wasm_bindgen_test]
    fn should_create_instance_from_document_create() {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let factory_calls = Rc::new(Cell::new(0));
        let connections = Rc::new(Cell::new(0));
        let factory_calls_for_constructor = factory_calls.clone();
        let connections_for_constructor = connections.clone();
        let (factory, constructor) = create_custom_element_with_config(
            move |_element, _arguments| {
                factory_calls_for_constructor.set(factory_calls_for_constructor.get() + 1);
                ConnectedElement {
                    connections: connections_for_constructor.clone(),
                }
            },
            vec![],
            CustomElementConfiguration {
                element_constructor: &HTML_PARAGRAPH_ELEMENT_CONSTRUCTOR,
            },
        );
        factory.forget();
        let name = helpers::get_random_custom_element_name();
        let mut options = ElementDefinitionOptions::new();
        options.extends("p");
        window
            .custom_elements()
            .define_with_options(&name, &constructor, &options)
            .unwrap();

        let creation_options = ElementCreationOptions::new();
        creation_options.set_is(&name);
        let instance = document
            .create_element_with_element_creation_options("p", &creation_options)
            .unwrap();

        assert!(instance.is_instance_of::<HtmlParagraphElement>());
        assert_eq!(instance.local_name(), "p");
        assert_eq!(factory_calls.get(), 1);
        document.body().unwrap().append_child(&instance).unwrap();
        assert_eq!(connections.get(), 1);
    }
}

pub mod bridge {

    mod call_forwarding_to_struct {
        use wasm_bindgen_test::wasm_bindgen_test;
        use web_sys::window;

        use crate::common::helpers;

        #[wasm_bindgen_test]
        fn should_forward_connected_calls() {
            let window = window().unwrap();
            let document = window.document().unwrap();
            let (instance, call_counters) = helpers::create_custom_element_with_call_counters();

            assert_eq!(*call_counters.num_calls_connected.borrow(), 0);

            document.body().unwrap().append_child(&instance).unwrap();

            assert_eq!(*call_counters.num_calls_connected.borrow(), 1);
        }

        #[wasm_bindgen_test]
        fn should_forward_disconnected_calls() {
            let window = window().unwrap();
            let document = window.document().unwrap();
            let (instance, call_counters) = helpers::create_custom_element_with_call_counters();
            document.body().unwrap().append_child(&instance).unwrap();

            assert_eq!(*call_counters.num_calls_disconnected.borrow(), 0);

            document.body().unwrap().remove_child(&instance).unwrap();

            assert_eq!(*call_counters.num_calls_disconnected.borrow(), 1);
        }

        #[wasm_bindgen_test]
        fn should_forward_adopted_calls() {
            let window = window().unwrap();
            let document = window.document().unwrap();
            let (instance, call_counters) = helpers::create_custom_element_with_call_counters();
            document.body().unwrap().append_child(&instance).unwrap();

            assert_eq!(*call_counters.num_calls_adopted.borrow(), 0);

            let new_document = document
                .implementation()
                .unwrap()
                .create_html_document()
                .unwrap();

            new_document.adopt_node(&instance).unwrap();

            assert_eq!(*call_counters.num_calls_adopted.borrow(), 1);
        }

        #[wasm_bindgen_test]
        fn should_forward_attribute_changed_calls() {
            let window = window().unwrap();
            let document = window.document().unwrap();
            let (instance, call_counters) = helpers::create_custom_element_with_call_counters();

            // does NOT increment unobserved attributes

            assert_eq!(*call_counters.num_calls_attribute_changed.borrow(), 0);

            instance
                .set_attribute("unobserved", "Hello, world")
                .unwrap();

            assert_eq!(*call_counters.num_calls_attribute_changed.borrow(), 0);

            // DOES increment observed attributes

            assert_eq!(*call_counters.num_calls_attribute_changed.borrow(), 0);

            instance.set_attribute("observed", "Hello, world").unwrap();

            assert_eq!(*call_counters.num_calls_attribute_changed.borrow(), 1);
        }
    }
}
