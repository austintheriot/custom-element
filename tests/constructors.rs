use wasm_bindgen_test::*;

mod common;

wasm_bindgen_test_configure!(run_in_browser);

pub mod autonomous_custom_elements {
    use crate::common::helpers;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn should_create_constructor() {
        let constructor = helpers::create_autonomous_custom_element_constructor();
        helpers::assert_is_autonomous_element_constructor(&constructor);
    }
}

pub mod customized_built_in_custom_elements {
    use crate::common::helpers;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn should_create_constructor() {
        let constructor = helpers::create_customized_built_in_element_constructor();
        helpers::assert_is_customized_built_in_constructor(&constructor);
    }
}
