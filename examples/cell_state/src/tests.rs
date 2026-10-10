use super::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen_test::*;
use web_sys::{window, ShadowRoot};

wasm_bindgen_test_configure!(run_in_browser);

struct Fixture {
    element: HtmlElement,
    shadow: ShadowRoot,
}

impl Fixture {
    fn register(name: &str) -> Self {
        let (closure, constructor) = custom_element::create_custom_element(CellCounter::new);
        closure.forget();
        window()
            .unwrap()
            .custom_elements()
            .define(name, &constructor)
            .unwrap();
        Self::from_registration(name)
    }

    fn from_registration(name: &str) -> Self {
        let document = window().unwrap().document().unwrap();
        let element: HtmlElement = document.create_element(name).unwrap().dyn_into().unwrap();
        document.body().unwrap().append_child(&element).unwrap();
        let shadow = element.shadow_root().unwrap();
        Self { element, shadow }
    }

    fn click(&self) {
        let button: HtmlElement = self
            .shadow
            .query_selector("[data-role='increment']")
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
}

impl Drop for Fixture {
    fn drop(&mut self) {
        self.element.remove();
    }
}

struct ObservedCellCounter {
    inner: Rc<CellCounter>,
    seen: Rc<RefCell<Vec<u32>>>,
}

impl CustomElement for ObservedCellCounter {
    fn observed_attributes() -> &'static [&'static str] {
        CellCounter::observed_attributes()
    }

    fn connected_callback(&self) {
        self.inner.connected_callback();
    }
    fn disconnected_callback(&self) {
        self.inner.disconnected_callback();
    }
    fn handle_event(&self, event: Event) {
        self.inner.handle_event(event);
    }
    fn attribute_changed_callback(&self, name: &str, old: Option<String>, new: Option<String>) {
        self.seen.borrow_mut().push(self.inner.count.get());
        self.inner.attribute_changed_callback(name, old, new);
    }
}

#[wasm_bindgen_test]
fn click_commits_cell_before_nested_attribute_callback() {
    let components: Rc<RefCell<Vec<Rc<CellCounter>>>> = Rc::new(RefCell::new(Vec::new()));
    let seen = Rc::new(RefCell::new(Vec::new()));
    let retained_components = components.clone();
    let retained_seen = seen.clone();
    let (closure, constructor) = custom_element::create_custom_element(move |element, args| {
        let inner = Rc::new(CellCounter::new(element, args));
        retained_components.borrow_mut().push(inner.clone());
        ObservedCellCounter {
            inner,
            seen: retained_seen.clone(),
        }
    });
    closure.forget();
    let name = "cell-precommit-test";
    window()
        .unwrap()
        .custom_elements()
        .define(name, &constructor)
        .unwrap();
    let fixture = Fixture::from_registration(name);
    fixture.assert_count(None, "Count: 0");
    fixture.click();
    assert_eq!(&*seen.borrow(), &[1]);
    assert_eq!(components.borrow()[0].count.get(), 1);
    fixture.assert_count(Some("1"), "Count: 1");
}

#[wasm_bindgen_test]
fn clicks_render_immediately() {
    let fixture = Fixture::register("cell-clicks-test");
    fixture.assert_count(None, "Count: 0");
    fixture.click();
    fixture.assert_count(Some("1"), "Count: 1");
    fixture.click();
    fixture.assert_count(Some("2"), "Count: 2");
}

#[wasm_bindgen_test]
fn external_count_assignment_renders_immediately() {
    let fixture = Fixture::register("cell-external-test");
    fixture.element.set_attribute("count", "7").unwrap();
    fixture.assert_count(Some("7"), "Count: 7");
    fixture.click();
    fixture.assert_count(Some("8"), "Count: 8");
}

#[wasm_bindgen_test]
fn removed_and_invalid_attributes_reset_state() {
    for (suffix, raw) in [
        ("text", "bad"),
        ("negative", "-1"),
        ("overflow", "4294967296"),
    ] {
        let name = format!("cell-invalid-{suffix}-test");
        let fixture = Fixture::register(&name);
        fixture.element.set_attribute("count", "9").unwrap();
        fixture.element.set_attribute("count", raw).unwrap();
        fixture.assert_count(Some(raw), "Count: 0");
        fixture.click();
        fixture.assert_count(Some("1"), "Count: 1");
    }
    let fixture = Fixture::register("cell-removal-test");
    fixture.element.set_attribute("count", "9").unwrap();
    fixture.element.remove_attribute("count").unwrap();
    fixture.assert_count(None, "Count: 0");
    fixture.click();
    fixture.assert_count(Some("1"), "Count: 1");
}

#[wasm_bindgen_test]
fn increment_saturates_at_maximum() {
    let fixture = Fixture::register("cell-saturation-test");
    fixture
        .element
        .set_attribute("count", "4294967295")
        .unwrap();
    fixture.assert_count(Some("4294967295"), "Count: 4294967295");
    fixture.click();
    fixture.assert_count(Some("4294967295"), "Count: 4294967295");
}

#[wasm_bindgen_test]
fn instances_of_one_registration_have_independent_state() {
    let name = "cell-isolation-test";
    let first = Fixture::register(name);
    let second = Fixture::from_registration(name);
    first.click();
    first.assert_count(Some("1"), "Count: 1");
    second.assert_count(None, "Count: 0");
    second.element.set_attribute("count", "10").unwrap();
    second.click();
    first.assert_count(Some("1"), "Count: 1");
    second.assert_count(Some("11"), "Count: 11");
}

#[wasm_bindgen_test]
fn reconnection_restores_one_click_listener() {
    let fixture = Fixture::register("cell-reconnect-test");
    fixture.click();
    fixture.element.remove();
    fixture.click();
    fixture.assert_count(Some("1"), "Count: 1");
    window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&fixture.element)
        .unwrap();
    fixture.click();
    fixture.assert_count(Some("2"), "Count: 2");
    fixture.click();
    fixture.assert_count(Some("3"), "Count: 3");
}

#[wasm_bindgen_test]
fn equal_attribute_write_renders_again() {
    let fixture = Fixture::register("cell-equal-test");
    fixture.element.set_attribute("count", "5").unwrap();
    let display = fixture
        .shadow
        .query_selector("[data-role='display']")
        .unwrap()
        .unwrap();
    display.set_text_content(Some("stale"));
    fixture.element.set_attribute("count", "5").unwrap();
    fixture.assert_count(Some("5"), "Count: 5");
}

#[wasm_bindgen_test]
fn click_on_host_does_not_increment() {
    let fixture = Fixture::register("cell-host-click-test");
    fixture
        .element
        .add_event_listener_with_callback("click", &fixture.element.clone().unchecked_into())
        .unwrap();
    fixture.element.click();
    fixture.assert_count(None, "Count: 0");
}
