use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{window, HtmlElement};

use crate::counter::Counter;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn two_clicks_update_attribute_and_display_immediately() {
    let (factory, constructor) =
        custom_element::create_custom_element(Counter::new, Counter::get_observed_attributes());
    factory.forget();
    let name = "test-counter-immediate";
    let document = window().unwrap().document().unwrap();
    window()
        .unwrap()
        .custom_elements()
        .define(name, &constructor)
        .unwrap();
    let element: HtmlElement = document.create_element(name).unwrap().dyn_into().unwrap();
    document.body().unwrap().append_child(&element).unwrap();
    let shadow_root = element.shadow_root().unwrap();
    let button: HtmlElement = shadow_root
        .query_selector("button:nth-of-type(2)")
        .unwrap()
        .unwrap()
        .dyn_into()
        .unwrap();
    let display = shadow_root.query_selector("p").unwrap().unwrap();

    button.click();
    assert_eq!(element.get_attribute("count").as_deref(), Some("1"));
    assert_eq!(display.text_content().as_deref(), Some("Count is 1"));
    button.click();
    assert_eq!(element.get_attribute("count").as_deref(), Some("2"));
    assert_eq!(display.text_content().as_deref(), Some("Count is 2"));

    element.remove();
}
