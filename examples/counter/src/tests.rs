use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;
use web_sys::{window, HtmlElement, ShadowRoot};

use crate::counter::Counter;

wasm_bindgen_test_configure!(run_in_browser);

struct CounterFixture {
    element: HtmlElement,
    shadow: ShadowRoot,
}

impl CounterFixture {
    fn new(name: &str, count: Option<&str>, step: Option<&str>) -> Self {
        let (factory, constructor) =
            custom_element::create_custom_element(Counter::new, Counter::get_observed_attributes());
        factory.forget();
        let browser = window().unwrap();
        browser
            .custom_elements()
            .define(name, &constructor)
            .unwrap();
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

impl Drop for CounterFixture {
    fn drop(&mut self) {
        self.element.remove();
    }
}

#[wasm_bindgen_test]
fn missing_attributes_display_defaults_without_writing_attributes() {
    let counter = CounterFixture::new("test-counter-defaults", None, None);
    counter.assert_count(None, "Count is 0");
    counter.assert_step(None, "Increment by 1");
}

#[wasm_bindgen_test]
fn two_clicks_update_count_and_display_before_returning() {
    let counter = CounterFixture::new("test-counter-two-clicks", Some("4"), Some("2"));
    counter.assert_count(Some("4"), "Count is 4");
    counter.assert_step(Some("2"), "Increment by 2");
    counter.click("increment");
    counter.assert_count(Some("6"), "Count is 6");
    counter.click("increment");
    counter.assert_count(Some("8"), "Count is 8");
}

#[wasm_bindgen_test]
fn plus_and_minus_update_step_immediately() {
    let counter = CounterFixture::new("test-counter-step-buttons", None, Some("1"));
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
    let zero_step = CounterFixture::new("test-counter-zero-step", Some("4"), Some("0"));
    zero_step.assert_count(Some("4"), "Count is 4");
    zero_step.click("increment");
    zero_step.assert_count(Some("4"), "Count is 4");

    let negative_step = CounterFixture::new("test-counter-negative-step", Some("4"), Some("-2"));
    negative_step.assert_count(Some("4"), "Count is 4");
    negative_step.click("increment");
    negative_step.assert_count(Some("2"), "Count is 2");
}

#[wasm_bindgen_test]
fn external_attribute_changes_render_without_clicks() {
    let counter = CounterFixture::new("test-counter-external-attributes", Some("4"), Some("3"));
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
        let counter = CounterFixture::new(name, Some(raw_count), None);
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
        let counter = CounterFixture::new(name, Some("0"), Some(raw_step));
        counter.assert_count(Some("0"), "Count is 0");
        counter.assert_step(Some(raw_step), "Increment by 1");
        counter.click("increment");
        counter.assert_count(Some("1"), "Count is 1");
        counter.assert_step(Some(raw_step), "Increment by 1");
    }
}

#[wasm_bindgen_test]
fn counter_instances_keep_independent_attributes_and_displays() {
    let first = CounterFixture::new("test-counter-isolation-first", Some("4"), Some("2"));
    let second = CounterFixture::new("test-counter-isolation-second", Some("9"), Some("3"));
    first.assert_count(Some("4"), "Count is 4");
    second.assert_count(Some("9"), "Count is 9");
    first.click("increment");
    first.assert_count(Some("6"), "Count is 6");
    second.assert_count(Some("9"), "Count is 9");
    second.click("minus");
    second.assert_step(Some("2"), "Increment by 2");
    first.assert_step(Some("2"), "Increment by 2");
    second.click("increment");
    second.assert_count(Some("11"), "Count is 11");
    first.assert_count(Some("6"), "Count is 6");
}

#[wasm_bindgen_test]
fn detached_attribute_change_renders_and_reconnection_keeps_one_listener() {
    let counter = CounterFixture::new("test-counter-reconnect", Some("4"), Some("2"));
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
