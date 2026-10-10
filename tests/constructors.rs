use js_sys::{Object, Reflect};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

mod common {
    pub mod constructors;
    pub mod mock;
}

wasm_bindgen_test_configure!(run_in_browser);

fn assert_is_autonomous_element_constructor(value: &JsValue) {
    assert!(value.is_function());
    let prototype = Object::get_prototype_of(value);
    assert!(prototype.is_function());
    let name = Reflect::get(&prototype, &JsValue::from_str("name")).unwrap();
    assert_eq!(name, "HTMLElement");
}

fn assert_is_customized_built_in_constructor(value: &JsValue) {
    assert!(value.is_function());
    let prototype = Object::get_prototype_of(value);
    assert!(prototype.is_function());
    let name = Reflect::get(&prototype, &JsValue::from_str("name")).unwrap();
    assert_ne!(name, "HTMLElement");
    assert!(name.as_string().unwrap().contains("Element"))
}

pub mod autonomous_custom_elements {
    use super::assert_is_autonomous_element_constructor;
    use crate::common::constructors;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn should_create_constructor() {
        let constructor = constructors::create_autonomous_custom_element_constructor();
        assert_is_autonomous_element_constructor(&constructor);
    }
}

pub mod customized_built_in_custom_elements {
    use super::assert_is_customized_built_in_constructor;
    use crate::common::constructors;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn should_create_constructor() {
        let constructor = constructors::create_customized_built_in_element_constructor();
        assert_is_customized_built_in_constructor(&constructor);
    }
}
