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
