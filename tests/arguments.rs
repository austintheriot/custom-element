use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

mod common;

use common::helpers;

pub mod autonomous_custom_elements {
    use std::{cell::RefCell, rc::Rc};

    use custom_element::create_custom_element;

    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;
    use web_sys::window;

    use crate::helpers::{self, MockCustomElement};

    #[wasm_bindgen_test]
    fn should_pass_constructor_args_to_wasm() {
        let window = window().unwrap();

        // HACK: peek into call to see what the Rust constructor has
        // available when called from JS
        let arg_passed_to_constructor = JsValue::from_f64(1.234);
        let arg_peeked_from_constructor = Rc::new(RefCell::new(None));
        {
            // create custom element class
            let arg_peeked_from_constructor = Rc::clone(&arg_peeked_from_constructor);
            let (closure, constructor) = create_custom_element(
                move |element, args| {
                    arg_peeked_from_constructor
                        .borrow_mut()
                        .replace(args.get(0));
                    MockCustomElement::new_with_args(element, args)
                },
                vec![String::from("count")],
            );
            closure.forget();

            // define custom element
            let name = helpers::get_random_custom_element_name();
            window
                .custom_elements()
                .define(&name, &constructor)
                .unwrap();

            // instantiate custom element
            constructor
                .construct_with_arguments(vec![arg_passed_to_constructor.clone()])
                .unwrap();
        }

        assert_eq!(
            &arg_passed_to_constructor,
            arg_peeked_from_constructor.borrow().as_ref().unwrap()
        );
    }
}

pub mod customized_built_in_components {
    use std::{cell::RefCell, rc::Rc};

    use custom_element::{
        constructors::HTML_PARAGRAPH_ELEMENT_CONSTRUCTOR, create_custom_element_with_config,
        CustomElementConfiguration,
    };

    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;
    use web_sys::{window, ElementDefinitionOptions};

    use crate::helpers::{self, MockCustomElement};

    #[wasm_bindgen_test]
    fn should_pass_constructor_args_to_wasm() {
        let window = window().unwrap();

        // HACK: peek into call to see what the Rust constructor has
        // available when called from JS
        let arg_passed_to_constructor = JsValue::from_f64(1.234);
        let arg_peeked_from_constructor = Rc::new(RefCell::new(None));
        {
            // create custom element class
            let arg_peeked_from_constructor = Rc::clone(&arg_peeked_from_constructor);
            let (closure, constructor) = create_custom_element_with_config(
                move |element, args| {
                    arg_peeked_from_constructor
                        .borrow_mut()
                        .replace(args.get(0));
                    MockCustomElement::new_with_args(element, args)
                },
                vec![String::from("count")],
                CustomElementConfiguration {
                    element_constructor: &HTML_PARAGRAPH_ELEMENT_CONSTRUCTOR,
                },
            );
            closure.forget();

            // define custom element
            let name = helpers::get_random_custom_element_name();
            let mut options = ElementDefinitionOptions::new();
            options.extends("p");
            window
                .custom_elements()
                .define_with_options(&name, &constructor, &options)
                .unwrap();

            // instantiate custom element
            constructor
                .construct_with_arguments(vec![arg_passed_to_constructor.clone()])
                .unwrap();
        }

        assert_eq!(
            &arg_passed_to_constructor,
            arg_peeked_from_constructor.borrow().as_ref().unwrap()
        );
    }
}
