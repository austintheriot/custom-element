use custom_element::{CustomElement, GeneratedConstructor};

use leptos::RwSignal;
use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{HtmlElement, ShadowRootInit, ShadowRootMode};

use crate::leptos_app::{App, AppProps};
use crate::State;

pub struct CustomElementApp;

impl CustomElement for CustomElementApp {
    fn connected_callback(&mut self) {}

    fn disconnected_callback(&mut self) {}
}

#[wasm_bindgen(js_name = getAppTagName)]
pub fn get_app_tag_name() -> String {
    String::from("my-rust-app")
}

impl CustomElementApp {
    pub(crate) fn register(state: RwSignal<State>) {
        let constructor = CustomElementApp::create_app_element(state);
        CustomElementApp::register_app_element(&constructor);
    }

    fn create_app_element(state: RwSignal<State>) -> GeneratedConstructor {
        let (closure, constructor) = custom_element::create_custom_element(
            move |instance, _args| CustomElementApp::new(instance, state),
            vec![],
        );
        closure.forget();
        constructor
    }

    fn register_app_element(constructor: &GeneratedConstructor) {
        let window = web_sys::window().unwrap();
        window
            .custom_elements()
            .define(&get_app_tag_name(), constructor)
            .unwrap();
    }

    // is called every time this component is created fresh
    fn new(instance: JsValue, state: RwSignal<State>) -> Self {
        let instance: HtmlElement = instance.into();
        let shadow_root_init = ShadowRootInit::new(ShadowRootMode::Open);
        let shadow_root = instance.attach_shadow(&shadow_root_init).unwrap();
        leptos::mount_to(shadow_root.unchecked_into(), move || {
            App(AppProps { state })
        });

        CustomElementApp
    }
}
