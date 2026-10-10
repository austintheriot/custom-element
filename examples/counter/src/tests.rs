use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{window, HtmlElement, ShadowRoot};

use crate::component::Counter;

wasm_bindgen_test_configure!(run_in_browser);

struct Fixture {
    element: HtmlElement,
    shadow: ShadowRoot,
}

impl Fixture {
    fn new(name: &str, count: Option<&str>, step: Option<&str>) -> Self {
        let (closure, constructor) =
            custom_element::create_custom_element(Counter::new, Counter::observed_attributes());
        closure.forget();
        let browser = window().unwrap();
        browser
            .custom_elements()
            .define(name, &constructor)
            .unwrap();
        Self::from_registration(name, count, step)
    }

    fn from_registration(name: &str, count: Option<&str>, step: Option<&str>) -> Self {
        let browser = window().unwrap();
        let document = browser.document().unwrap();
        let element: HtmlElement = document.create_element(name).unwrap().dyn_into().unwrap();
        if let Some(count) = count {
            element.set_attribute("count", count).unwrap();
        }
        if let Some(step) = step {
            element.set_attribute("step", step).unwrap();
        }
        document.body().unwrap().append_child(&element).unwrap();
        let shadow = element.shadow_root().unwrap();
        Self { element, shadow }
    }

    fn click(&self, role: &str) {
        let button: HtmlElement = self
            .shadow
            .query_selector(&format!("[data-role='{role}']"))
            .unwrap()
            .unwrap()
            .dyn_into()
            .unwrap();
        button.click();
    }

    fn assert_count(&self, raw: Option<&str>, display: &str) {
        assert_eq!(self.element.get_attribute("count").as_deref(), raw);
        assert_eq!(
            self.shadow
                .query_selector("[data-role='display']")
                .unwrap()
                .unwrap()
                .text_content()
                .as_deref(),
            Some(display)
        );
    }

    fn assert_step(&self, raw: Option<&str>, label: &str) {
        assert_eq!(self.element.get_attribute("step").as_deref(), raw);
        assert_eq!(
            self.shadow
                .query_selector("[data-role='increment']")
                .unwrap()
                .unwrap()
                .text_content()
                .as_deref(),
            Some(label)
        );
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        self.element.remove();
    }
}

#[wasm_bindgen_test]
fn missing_attributes_display_defaults_without_writing_attributes() {
    let counter = Fixture::new("test-counter-defaults", None, None);
    counter.assert_count(None, "Count is 0");
    counter.assert_step(None, "Increment by 1");
}

#[wasm_bindgen_test]
fn two_clicks_update_count_and_display_before_returning() {
    let counter = Fixture::new("test-counter-two-clicks", Some("4"), Some("2"));
    counter.assert_count(Some("4"), "Count is 4");
    counter.assert_step(Some("2"), "Increment by 2");
    counter.click("increment");
    counter.assert_count(Some("6"), "Count is 6");
    counter.click("increment");
    counter.assert_count(Some("8"), "Count is 8");
}

#[wasm_bindgen_test]
fn plus_and_minus_update_step_immediately() {
    let counter = Fixture::new("test-counter-step-buttons", None, Some("1"));
    counter.assert_step(Some("1"), "Increment by 1");
    counter.click("plus");
    counter.assert_step(Some("2"), "Increment by 2");
    counter.click("minus");
    counter.assert_step(Some("1"), "Increment by 1");
    counter.click("minus");
    counter.assert_step(Some("0"), "Increment by 0");
    counter.assert_count(None, "Count is 0");
}

#[wasm_bindgen_test]
fn zero_step_keeps_count_and_negative_step_decrements_it() {
    let zero_step = Fixture::new("test-counter-zero-step", Some("4"), Some("0"));
    zero_step.assert_count(Some("4"), "Count is 4");
    zero_step.click("increment");
    zero_step.assert_count(Some("4"), "Count is 4");

    let negative_step = Fixture::new("test-counter-negative-step", Some("4"), Some("-2"));
    negative_step.assert_count(Some("4"), "Count is 4");
    negative_step.click("increment");
    negative_step.assert_count(Some("2"), "Count is 2");
}

#[wasm_bindgen_test]
fn external_attribute_changes_render_without_clicks() {
    let counter = Fixture::new("test-counter-external-attributes", Some("4"), Some("3"));
    counter.assert_count(Some("4"), "Count is 4");
    counter.element.set_attribute("count", "9").unwrap();
    counter.assert_count(Some("9"), "Count is 9");
    counter.element.remove_attribute("count").unwrap();
    counter.assert_count(None, "Count is 0");
    counter.element.remove_attribute("step").unwrap();
    counter.assert_step(None, "Increment by 1");
    counter.click("increment");
    counter.assert_count(Some("1"), "Count is 1");
    counter.assert_step(None, "Increment by 1");
}

#[wasm_bindgen_test]
fn invalid_count_uses_display_default_and_preserves_raw_attribute() {
    for (name, raw_count) in [
        ("test-counter-invalid-count-text", "abc"),
        ("test-counter-invalid-count-overflow", "2147483648"),
    ] {
        let counter = Fixture::new(name, Some(raw_count), None);
        counter.assert_count(Some(raw_count), "Count is 0");
        counter.assert_step(None, "Increment by 1");
    }
}

#[wasm_bindgen_test]
fn invalid_step_uses_increment_default_and_preserves_raw_attribute() {
    for (name, raw_step) in [
        ("test-counter-invalid-step-text", "abc"),
        ("test-counter-invalid-step-overflow", "2147483648"),
    ] {
        let counter = Fixture::new(name, Some("0"), Some(raw_step));
        counter.assert_count(Some("0"), "Count is 0");
        counter.assert_step(Some(raw_step), "Increment by 1");
        counter.click("increment");
        counter.assert_count(Some("1"), "Count is 1");
        counter.assert_step(Some(raw_step), "Increment by 1");
    }
}

#[wasm_bindgen_test]
fn counter_instances_keep_independent_attributes_and_displays() {
    let name = "test-counter-isolation";
    let first = Fixture::new(name, Some("4"), Some("2"));
    let second = Fixture::from_registration(name, Some("9"), Some("5"));
    first.assert_count(Some("4"), "Count is 4");
    first.assert_step(Some("2"), "Increment by 2");
    second.assert_count(Some("9"), "Count is 9");
    second.assert_step(Some("5"), "Increment by 5");
    first.click("plus");
    first.assert_step(Some("3"), "Increment by 3");
    second.assert_step(Some("5"), "Increment by 5");
    first.click("increment");
    first.assert_count(Some("7"), "Count is 7");
    second.assert_count(Some("9"), "Count is 9");
    second.click("minus");
    second.assert_step(Some("4"), "Increment by 4");
    first.assert_step(Some("3"), "Increment by 3");
    second.click("increment");
    second.assert_count(Some("13"), "Count is 13");
    first.assert_count(Some("7"), "Count is 7");
}

#[wasm_bindgen_test]
fn detached_attribute_change_renders_and_reconnection_keeps_one_listener() {
    let counter = Fixture::new("test-counter-reconnect", Some("4"), Some("2"));
    counter.assert_count(Some("4"), "Count is 4");
    counter.element.remove();
    counter.element.set_attribute("count", "9").unwrap();
    counter.assert_count(Some("9"), "Count is 9");
    window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&counter.element)
        .unwrap();
    counter.assert_count(Some("9"), "Count is 9");
    counter.click("increment");
    counter.assert_count(Some("11"), "Count is 11");
    counter.click("increment");
    counter.assert_count(Some("13"), "Count is 13");
}
