use std::fmt::format;

use custom_element::CustomElement;
use js_sys::{Array, Object};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    Event, HtmlButtonElement, HtmlElement, HtmlParagraphElement, ShadowRoot, ShadowRootInit,
    ShadowRootMode,
};

struct Counter {
    instance: HtmlElement,
    count_display_element: HtmlParagraphElement,
    increment_button: HtmlButtonElement,
    plus_button: HtmlButtonElement,
    minus_button: HtmlButtonElement,
    count: i32,
    step: i32,
    closures: Vec<JsValue>,
}

impl CustomElement for Counter {
    fn connected_callback(&mut self) {
        self.render();
        self.increment_button
            .add_event_listener_with_callback("click", &self.instance.clone().unchecked_into())
            .unwrap();
        self.minus_button
            .add_event_listener_with_callback("click", &self.instance.clone().unchecked_into())
            .unwrap();
        self.plus_button
            .add_event_listener_with_callback("click", &self.instance.clone().unchecked_into())
            .unwrap();
    }

    fn attribute_changed_callback(&mut self, name: &str, _old_value: JsValue, new_value: JsValue) {
        match name {
            Self::COUNT_ATTR_NAME => self.handle_count_change(new_value),
            Self::STEP_ATTR_NAME => self.handle_step_change(new_value),
            _ => {}
        }

        self.render();
    }

    fn disconnected_callback(&mut self) {
        self.increment_button
            .remove_event_listener_with_callback("click", &self.instance.clone().unchecked_into())
            .unwrap();
        self.minus_button
            .remove_event_listener_with_callback("click", &self.instance.clone().unchecked_into())
            .unwrap();
        self.plus_button
            .remove_event_listener_with_callback("click", &self.instance.clone().unchecked_into())
            .unwrap();
        self.closures.clear();
    }

    fn handle_event(&mut self, event: Event) {
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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = queueMicrotask)]
    pub fn queue_microtask(micro_task: &JsValue);
}

impl Counter {
    pub const DEFAULT_COUNT: i32 = 0;
    pub const DEFAULT_STEP: i32 = 1;
    pub const PLUS_AMOUNT: i32 = 1;
    pub const MINUS_AMOUNT: i32 = -1;
    pub const COUNT_ATTR_NAME: &'static str = "count";
    pub const STEP_ATTR_NAME: &'static str = "step";

    fn new(instance: JsValue, _args: Array) -> Self {
        let instance: HtmlElement = instance.into();
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
            count: Self::DEFAULT_COUNT,
            step: Self::DEFAULT_STEP,
            closures: Vec::new(),
        }
    }

    fn render(&mut self) {
        let count = self.count;
        let step = self.step;

        self.count_display_element
            .set_inner_text(&format!("Count is {count}"));
        self.minus_button.set_inner_text("-");
        self.plus_button.set_inner_text("+");
        self.increment_button
            .set_inner_text(&format!("Increment by {step}"));
    }

    fn handle_increment_button_click(&mut self) {
        let instance = self.instance.clone();
        let new_count = self.get_count_attribute() + self.get_step_attribute();

        // Rust doesn't like calling this directly,
        // since it complains about recursive function
        // calls causing unsafe behavior--
        // so we schedule to run after this function
        let cb = Closure::wrap(Box::new(move || {
            instance
                .set_attribute(Self::COUNT_ATTR_NAME, &new_count.to_string())
                .unwrap();
        }) as Box<dyn FnMut()>);

        queue_microtask(&cb.as_ref());
        self.closures.push(cb.into_js_value());
    }

    fn handle_minus_button_click(&mut self) {
        let instance = self.instance.clone();
        let new_step = self.get_step_attribute() + Self::MINUS_AMOUNT;

        let cb = Closure::wrap(Box::new(move || {
            instance
                .set_attribute(Self::STEP_ATTR_NAME, &new_step.to_string())
                .unwrap();
        }) as Box<dyn FnMut()>);

        queue_microtask(&cb.as_ref());
        self.closures.push(cb.into_js_value());
    }

    fn handle_plus_button_click(&mut self) {
        let instance = self.instance.clone();
        let new_step = self.get_step_attribute() + Self::PLUS_AMOUNT;

        let cb = Closure::wrap(Box::new(move || {
            instance
                .set_attribute(Self::STEP_ATTR_NAME, &new_step.to_string())
                .unwrap();
        }) as Box<dyn FnMut()>);

        queue_microtask(&cb.as_ref());
        self.closures.push(cb.into_js_value());
    }

    fn handle_count_change(&mut self, new_value: JsValue) {
        self.count = new_value
            .as_string()
            .unwrap_or_else(|| Self::DEFAULT_COUNT.to_string())
            .parse::<i32>()
            .unwrap_or(Self::DEFAULT_COUNT);
    }

    fn handle_step_change(&mut self, new_value: JsValue) {
        self.step = new_value
            .as_string()
            .unwrap_or_else(|| Self::DEFAULT_STEP.to_string())
            .parse::<i32>()
            .unwrap_or(Self::DEFAULT_STEP);
    }

    fn get_count_attribute(&self) -> i32 {
        let Some(new_count_string) = self.instance.get_attribute(Self::COUNT_ATTR_NAME) else {
            return Self::DEFAULT_COUNT;
        };

        new_count_string
            .parse::<i32>()
            .unwrap_or(Self::DEFAULT_COUNT)
    }

    fn get_step_attribute(&self) -> i32 {
        let Some(new_count_string) = self.instance.get_attribute(Self::STEP_ATTR_NAME) else {
            return Self::DEFAULT_STEP;
        };

        new_count_string
            .parse::<i32>()
            .unwrap_or(Self::DEFAULT_STEP)
    }

    fn get_observed_attributes() -> Vec<String> {
        vec![
            Self::COUNT_ATTR_NAME.to_string(),
            Self::STEP_ATTR_NAME.to_string(),
        ]
    }
}

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    let component_name = "my-counter";
    let (closure, constructor) = codex_custom_elements::create_custom_element(
        Counter::new,
        Counter::get_observed_attributes(),
    );
    closure.forget();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    window
        .custom_elements()
        .define(component_name, &constructor)
        .unwrap();

    let counter = document.create_element(component_name).unwrap();
    counter.set_attribute("count", "0").unwrap();
    counter.set_attribute("step", "1").unwrap();

    body.append_child(&counter).unwrap();
}
