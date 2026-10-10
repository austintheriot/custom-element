use custom_element::CustomElement;
use js_sys::{Array, Object};
use wasm_bindgen::JsCast;
use web_sys::{
    Event, HtmlButtonElement, HtmlElement, HtmlParagraphElement, ShadowRootInit, ShadowRootMode,
};

pub(crate) struct Counter {
    instance: HtmlElement,
    count_display_element: HtmlParagraphElement,
    increment_button: HtmlButtonElement,
    plus_button: HtmlButtonElement,
    minus_button: HtmlButtonElement,
}

impl CustomElement for Counter {
    fn observed_attributes() -> &'static [&'static str] {
        &[Self::COUNT_ATTR_NAME, Self::STEP_ATTR_NAME]
    }

    fn connected_callback(&self) {
        self.render();
        self.increment_button
            .add_event_listener_with_event_listener("click", self.instance.unchecked_ref())
            .unwrap();
        self.minus_button
            .add_event_listener_with_event_listener("click", self.instance.unchecked_ref())
            .unwrap();
        self.plus_button
            .add_event_listener_with_event_listener("click", self.instance.unchecked_ref())
            .unwrap();
    }

    fn attribute_changed_callback(
        &self,
        _name: &str,
        _old_value: Option<String>,
        _new_value: Option<String>,
    ) {
        self.render();
    }

    fn disconnected_callback(&self) {
        self.increment_button
            .remove_event_listener_with_event_listener("click", self.instance.unchecked_ref())
            .unwrap();
        self.minus_button
            .remove_event_listener_with_event_listener("click", self.instance.unchecked_ref())
            .unwrap();
        self.plus_button
            .remove_event_listener_with_event_listener("click", self.instance.unchecked_ref())
            .unwrap();
    }

    fn handle_event(&self, event: Event) {
        match event.type_().as_str() {
            "click" => {
                let Some(current_target) = event.current_target() else {
                    return;
                };

                if Object::is(&current_target, &self.increment_button) {
                    self.handle_increment_button_click();
                } else if Object::is(&current_target, &self.plus_button) {
                    self.handle_plus_button_click();
                } else if Object::is(&current_target, &self.minus_button) {
                    self.handle_minus_button_click();
                }
            }
            _ => {}
        }
    }
}

impl Counter {
    pub const DEFAULT_COUNT: i32 = 0;
    pub const DEFAULT_STEP: i32 = 1;
    pub const PLUS_AMOUNT: i32 = 1;
    pub const MINUS_AMOUNT: i32 = -1;
    pub const COUNT_ATTR_NAME: &'static str = "count";
    pub const STEP_ATTR_NAME: &'static str = "step";

    pub(crate) fn new(instance: HtmlElement, _args: Array) -> Self {
        let shadow_root_init = ShadowRootInit::new(ShadowRootMode::Open);
        let shadow_root = instance.attach_shadow(&shadow_root_init).unwrap();
        let document = web_sys::window().unwrap().document().unwrap();
        let count_element: HtmlParagraphElement =
            document.create_element("p").unwrap().unchecked_into();
        let click_button: HtmlButtonElement =
            document.create_element("button").unwrap().unchecked_into();
        let plus_button: HtmlButtonElement =
            document.create_element("button").unwrap().unchecked_into();
        let minus_button: HtmlButtonElement =
            document.create_element("button").unwrap().unchecked_into();

        count_element.set_attribute("data-role", "display").unwrap();
        click_button
            .set_attribute("data-role", "increment")
            .unwrap();
        plus_button.set_attribute("data-role", "plus").unwrap();
        minus_button.set_attribute("data-role", "minus").unwrap();

        shadow_root.append_child(&count_element).unwrap();
        shadow_root.append_child(&minus_button).unwrap();
        shadow_root.append_child(&click_button).unwrap();
        shadow_root.append_child(&plus_button).unwrap();

        Counter {
            instance,
            count_display_element: count_element,
            increment_button: click_button,
            plus_button,
            minus_button,
        }
    }

    fn render(&self) {
        let count = self.count_attribute();
        let step = self.step_attribute();

        self.count_display_element
            .set_inner_text(&format!("Count is {count}"));
        self.minus_button.set_inner_text("-");
        self.plus_button.set_inner_text("+");
        self.increment_button
            .set_inner_text(&format!("Increment by {step}"));
    }

    fn handle_increment_button_click(&self) {
        let count = self.count_attribute() + self.step_attribute();
        self.instance
            .set_attribute(Self::COUNT_ATTR_NAME, &count.to_string())
            .unwrap();
    }

    fn handle_minus_button_click(&self) {
        let step = self.step_attribute() + Self::MINUS_AMOUNT;
        self.instance
            .set_attribute(Self::STEP_ATTR_NAME, &step.to_string())
            .unwrap();
    }

    fn handle_plus_button_click(&self) {
        let step = self.step_attribute() + Self::PLUS_AMOUNT;
        self.instance
            .set_attribute(Self::STEP_ATTR_NAME, &step.to_string())
            .unwrap();
    }

    fn count_attribute(&self) -> i32 {
        let Some(new_count_string) = self.instance.get_attribute(Self::COUNT_ATTR_NAME) else {
            return Self::DEFAULT_COUNT;
        };

        new_count_string
            .parse::<i32>()
            .unwrap_or(Self::DEFAULT_COUNT)
    }

    fn step_attribute(&self) -> i32 {
        let Some(new_count_string) = self.instance.get_attribute(Self::STEP_ATTR_NAME) else {
            return Self::DEFAULT_STEP;
        };

        new_count_string
            .parse::<i32>()
            .unwrap_or(Self::DEFAULT_STEP)
    }
}
