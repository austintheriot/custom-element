use custom_element::{
    constructors::HTML_PARAGRAPH_ELEMENT_CONSTRUCTOR, create_custom_element,
    create_custom_element_with_config, CustomElementConfiguration, GeneratedConstructor,
};

use super::mock::MockCustomElement;

pub fn create_autonomous_custom_element_constructor() -> GeneratedConstructor {
    let (closure, constructor) = create_custom_element(
        |_element_instance, _args| MockCustomElement,
        vec![String::from("count")],
    );
    closure.forget();
    constructor
}

pub fn create_customized_built_in_element_constructor() -> GeneratedConstructor {
    let (closure, constructor) = create_custom_element_with_config(
        |_element_instance, _args| MockCustomElement,
        vec![String::from("count")],
        CustomElementConfiguration {
            element_constructor: &HTML_PARAGRAPH_ELEMENT_CONSTRUCTOR,
        },
    );
    closure.forget();
    constructor
}
