use custom_element::{
    constructors::html_paragraph_element_constructor, create_custom_element,
    create_custom_element_with_config, CustomElementConfiguration, GeneratedConstructor,
};

use super::mock::MockCustomElement;

pub fn create_autonomous_custom_element_constructor() -> GeneratedConstructor {
    let (closure, constructor) =
        create_custom_element(|_element_instance, _args| MockCustomElement);
    closure.forget();
    constructor
}

pub fn create_customized_built_in_element_constructor() -> GeneratedConstructor {
    let (closure, constructor) = create_custom_element_with_config(
        |_element_instance, _args| MockCustomElement,
        CustomElementConfiguration {
            element_constructor: html_paragraph_element_constructor(),
        },
    );
    closure.forget();
    constructor
}
