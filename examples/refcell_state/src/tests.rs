use super::*;
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
        let (closure, constructor) = custom_element::create_custom_element(
            CompoundCounter::new,
            CompoundCounter::observed_attributes(),
        );
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

    fn assert_state(&self, raw: Option<&str>, label: &str) {
        assert_eq!(self.element.get_attribute("count").as_deref(), raw);
        assert_eq!(
            self.shadow
                .query_selector("[data-role='display']")
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

struct ObservedCompoundCounter {
    inner: Rc<CompoundCounter>,
    seen: Rc<RefCell<Vec<(u32, u32)>>>,
}

impl CustomElement for ObservedCompoundCounter {
    fn connected_callback(&self) {
        self.inner.connected_callback();
    }
    fn disconnected_callback(&self) {
        self.inner.disconnected_callback();
    }
    fn handle_event(&self, event: Event) {
        self.inner.handle_event(event);
    }
    fn attribute_changed_callback(&self, name: &str, old: JsValue, new: JsValue) {
        let snapshot = {
            let model = self
                .inner
                .model
                .try_borrow()
                .expect("model must be released before reflection");
            (model.count, model.updates)
        };
        self.seen.borrow_mut().push(snapshot);
        self.inner.attribute_changed_callback(name, old, new);
    }
}

#[wasm_bindgen_test]
fn one_click_updates_both_fields_before_return() {
    let components: Rc<RefCell<Vec<Rc<CompoundCounter>>>> = Rc::new(RefCell::new(Vec::new()));
    let seen = Rc::new(RefCell::new(Vec::new()));
    let retained_components = components.clone();
    let retained_seen = seen.clone();
    let (closure, constructor) = custom_element::create_custom_element(
        move |element, args| {
            let inner = Rc::new(CompoundCounter::new(element, args));
            retained_components.borrow_mut().push(inner.clone());
            ObservedCompoundCounter {
                inner,
                seen: retained_seen.clone(),
            }
        },
        CompoundCounter::observed_attributes(),
    );
    closure.forget();
    window()
        .unwrap()
        .custom_elements()
        .define("compound-observed-test", &constructor)
        .unwrap();
    let fixture = Fixture::from_registration("compound-observed-test");
    fixture.assert_state(None, "Count: 0; updates: 0");
    fixture.click();
    assert_eq!(&*seen.borrow(), &[(1, 1)]);
    let retained = components.borrow();
    let model = retained[0].model.borrow();
    assert_eq!((model.count, model.updates), (1, 1));
    fixture.assert_state(Some("1"), "Count: 1; updates: 1");
    drop(model);
    retained[0].model.borrow_mut().updates = u32::MAX - 1;
    fixture.click();
    fixture.assert_state(Some("2"), "Count: 2; updates: 4294967295");
    fixture.click();
    fixture.assert_state(Some("3"), "Count: 3; updates: 4294967295");
}

#[wasm_bindgen_test]
fn clicks_and_external_attributes_update_the_compound_model() {
    let fixture = Fixture::register("compound-changes-test");
    fixture.assert_state(None, "Count: 0; updates: 0");
    fixture.click();
    fixture.assert_state(Some("1"), "Count: 1; updates: 1");
    fixture.click();
    fixture.assert_state(Some("2"), "Count: 2; updates: 2");
    fixture.element.set_attribute("count", "7").unwrap();
    fixture.assert_state(Some("7"), "Count: 7; updates: 3");

    let display = fixture
        .shadow
        .query_selector("[data-role='display']")
        .unwrap()
        .unwrap();
    display.set_text_content(Some("stale"));
    fixture.element.set_attribute("count", "7").unwrap();
    fixture.assert_state(Some("7"), "Count: 7; updates: 3");

    fixture.element.remove_attribute("count").unwrap();
    fixture.assert_state(None, "Count: 0; updates: 4");
    fixture.element.set_attribute("count", "invalid").unwrap();
    fixture.assert_state(Some("invalid"), "Count: 0; updates: 4");
}

#[wasm_bindgen_test]
fn count_saturates_while_action_updates_advance() {
    let fixture = Fixture::register("compound-saturation-test");
    fixture
        .element
        .set_attribute("count", &u32::MAX.to_string())
        .unwrap();
    fixture.assert_state(Some("4294967295"), "Count: 4294967295; updates: 1");
    fixture.click();
    fixture.assert_state(Some("4294967295"), "Count: 4294967295; updates: 2");
    fixture.click();
    fixture.assert_state(Some("4294967295"), "Count: 4294967295; updates: 3");
}

#[wasm_bindgen_test]
fn two_instances_of_one_registration_have_independent_models() {
    let first = Fixture::register("compound-isolation-test");
    let second = Fixture::from_registration("compound-isolation-test");
    first.click();
    first.assert_state(Some("1"), "Count: 1; updates: 1");
    second.assert_state(None, "Count: 0; updates: 0");
    second.element.set_attribute("count", "10").unwrap();
    second.click();
    first.assert_state(Some("1"), "Count: 1; updates: 1");
    second.assert_state(Some("11"), "Count: 11; updates: 2");
}

#[wasm_bindgen_test]
fn reconnection_restores_one_listener_and_preserves_state() {
    let fixture = Fixture::register("compound-reconnect-test");
    fixture.click();
    fixture.element.remove();
    fixture.click();
    fixture.assert_state(Some("1"), "Count: 1; updates: 1");
    window()
        .unwrap()
        .document()
        .unwrap()
        .body()
        .unwrap()
        .append_child(&fixture.element)
        .unwrap();
    fixture.click();
    fixture.assert_state(Some("2"), "Count: 2; updates: 2");
    fixture.click();
    fixture.assert_state(Some("3"), "Count: 3; updates: 3");
}

#[wasm_bindgen_test]
fn click_on_host_does_not_increment() {
    let fixture = Fixture::register("refcell-host-click-test");
    fixture
        .element
        .add_event_listener_with_callback("click", &fixture.element.clone().unchecked_into())
        .unwrap();
    fixture.element.click();
    fixture.assert_state(None, "Count: 0; updates: 0");
}
