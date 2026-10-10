use custom_element::{
    constructors::html_paragraph_element_constructor, create_custom_element,
    create_custom_element_extending, GeneratedConstructor,
};

use super::mock::MockCustomElement;

pub fn create_autonomous_custom_element_constructor() -> GeneratedConstructor {
    let (closure, constructor) =
        create_custom_element(|_element_instance, _args| MockCustomElement);
    closure.forget();
    constructor
}

pub fn create_customized_built_in_element_constructor() -> GeneratedConstructor {
    let (closure, constructor) = create_custom_element_extending(
        &html_paragraph_element_constructor(),
        |_element_instance, _args| MockCustomElement,
    );
    closure.forget();
    constructor
}
