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
    use js_sys::{Array, Reflect};
    use wasm_bindgen_test::*;
    use web_sys::{window, ElementDefinitionOptions};

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
        let constructor = helpers::create_customized_built_in_element_constructor();
        let name = helpers::get_random_custom_element_name();
        let mut options = ElementDefinitionOptions::new();
        options.extends("p");
        window
            .custom_elements()
            .define_with_options(&name, &constructor, &options)
            .unwrap();

        let instance = document.create_element(&name).unwrap();

        helpers::assert_is_custom_element_instance(&instance)
    }
}

pub mod bridge {
    use std::{cell::RefCell, rc::Rc};

    use custom_element::{
        constructors::HTML_ELEMENT_CONSTRUCTOR, create_custom_element_with_config, CustomElement,
        CustomElementConfiguration,
    };

    use wasm_bindgen_test::wasm_bindgen_test;
    use web_sys::window;

    use crate::common::helpers;

    #[wasm_bindgen_test]
    fn should_forward_calls_to_the_underlying_struct() {
        pub struct CustomElementCallCounter {
            num_calls_connected: Rc<RefCell<u32>>,
            num_calls_disconnected: Rc<RefCell<u32>>,
            num_calls_adopted: Rc<RefCell<u32>>,
            num_calls_attributed_changed: Rc<RefCell<u32>>,
            num_calls_handle_event: Rc<RefCell<u32>>,
        }

        impl CustomElement for CustomElementCallCounter {
            fn connected_callback(&mut self) {
                *self.num_calls_connected.borrow_mut() += 1;
            }

            fn disconnected_callback(&mut self) {
                *self.num_calls_disconnected.borrow_mut() += 1;
            }

            fn adopted_callback(&mut self) {
                *self.num_calls_adopted.borrow_mut() += 1;
            }

            fn attribute_changed_callback(
                &mut self,
                _name: &str,
                _old_value: wasm_bindgen::JsValue,
                _new_value: wasm_bindgen::JsValue,
            ) {
                *self.num_calls_attributed_changed.borrow_mut() += 1;
            }

            fn handle_event(&mut self, _event: web_sys::Event) {
                *self.num_calls_handle_event.borrow_mut() += 1;
            }
        }

        impl CustomElementCallCounter {
            // allows to peek at implementation
            pub fn new(
                num_calls_connected: Rc<RefCell<u32>>,
                num_calls_disconnected: Rc<RefCell<u32>>,
                num_calls_adopted: Rc<RefCell<u32>>,
                num_calls_attributed_changed: Rc<RefCell<u32>>,
                num_calls_handle_event: Rc<RefCell<u32>>,
            ) -> Self {
                Self {
                    num_calls_connected,
                    num_calls_disconnected,
                    num_calls_adopted,
                    num_calls_attributed_changed,
                    num_calls_handle_event,
                }
            }
        }

        let window = window().unwrap();
        let document = window.document().unwrap();

        let num_calls_connected = Rc::new(RefCell::new(0u32));
        let num_calls_disconnected = Rc::new(RefCell::new(0u32));
        let num_calls_adopted = Rc::new(RefCell::new(0u32));
        let num_calls_attributed_changed = Rc::new(RefCell::new(0u32));
        let num_calls_handle_event = Rc::new(RefCell::new(0u32));

        // create constructor
        let (closure, constructor) = {
            let num_calls_connected = Rc::clone(&num_calls_connected);
            let num_calls_disconnected = Rc::clone(&num_calls_disconnected);
            let num_calls_adopted = Rc::clone(&num_calls_adopted);
            let num_calls_attribute_changed = Rc::clone(&num_calls_attributed_changed);
            let num_calls_handle_event = Rc::clone(&num_calls_handle_event);

            create_custom_element_with_config(
                move |_element_instance, _args| {
                    CustomElementCallCounter::new(
                        Rc::clone(&num_calls_connected),
                        Rc::clone(&num_calls_disconnected),
                        Rc::clone(&num_calls_adopted),
                        Rc::clone(&num_calls_attribute_changed),
                        Rc::clone(&num_calls_handle_event),
                    )
                },
                vec![String::from("observed")],
                CustomElementConfiguration {
                    element_constructor: &HTML_ELEMENT_CONSTRUCTOR,
                },
            )
        };
        closure.forget();

        let name = helpers::get_random_custom_element_name();
        window
            .custom_elements()
            .define(&name, &constructor)
            .unwrap();

        let instance = document.create_element(&name).unwrap();

        // CONNECTED

        assert_eq!(*num_calls_connected.borrow(), 0);

        document.body().unwrap().append_child(&instance).unwrap();

        assert_eq!(*num_calls_connected.borrow(), 1);

        // DISCONNECTED

        assert_eq!(*num_calls_disconnected.borrow(), 0);

        document.body().unwrap().remove_child(&instance).unwrap();

        assert_eq!(*num_calls_disconnected.borrow(), 1);

        // ADOPTED

        assert_eq!(*num_calls_adopted.borrow(), 0);

        let new_document = document
            .implementation()
            .unwrap()
            .create_html_document()
            .unwrap();

        new_document.adopt_node(&instance).unwrap();

        assert_eq!(*num_calls_adopted.borrow(), 1);

        // ATTRIBUTE CHANGED
        // does not increment unobserved attributes

        assert_eq!(*num_calls_attributed_changed.borrow(), 0);

        instance
            .set_attribute("unobserved", "Hello, world")
            .unwrap();

        assert_eq!(*num_calls_attributed_changed.borrow(), 0);

        // does increment unobserved attributes

        assert_eq!(*num_calls_attributed_changed.borrow(), 0);

        instance.set_attribute("observed", "Hello, world").unwrap();

        assert_eq!(*num_calls_attributed_changed.borrow(), 1);
    }
}
