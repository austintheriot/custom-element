use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use codex_custom_elements::{CustomElement, GeneratedConstructor};

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{
    Event, HtmlElement, HtmlInputElement, HtmlLabelElement, HtmlParagraphElement, ShadowRootInit,
    ShadowRootMode,
};

use crate::{State, StateListener, StateListenerHandle};

struct InnerApp {
    instance: HtmlElement,
    name_display_element: HtmlParagraphElement,
    input_element: HtmlInputElement,
    closures: Vec<JsValue>,
    state: State,
    state_listener_handle: Option<StateListenerHandle>,
}

#[derive(Clone)]
pub struct App {
    inner: Arc<Mutex<InnerApp>>,
}

impl CustomElement for App {
    fn connected_callback(&mut self) {
        self.render();
    }

    fn disconnected_callback(&mut self) {
        self.clear_closures();
    }

    fn handle_event(&mut self, event: Event) {
        if event.type_().as_str() == "input" {
            self.handle_input_event()
        }
    }
}

// state calls
impl App {
    fn instance(&self) -> HtmlElement {
        self.inner.lock().unwrap().instance.clone()
    }

    fn set_state_listener_handle(&mut self, handle: StateListenerHandle) {
        self.inner.lock().unwrap().state_listener_handle = Some(handle)
    }

    fn state(&self) -> State {
        self.inner.lock().unwrap().state.clone()
    }

    fn name_display_element(&self) -> HtmlParagraphElement {
        self.inner.lock().unwrap().name_display_element.clone()
    }

    fn input_element(&self) -> HtmlInputElement {
        self.inner.lock().unwrap().input_element.clone()
    }

    fn clear_closures(&mut self) {
        self.inner.lock().unwrap().closures.clear();
    }
}

impl App {
    const COMPONENT_NAME: &'static str = "my-app";

    pub fn run() {
        let constructor = App::create_app_element();
        App::register_app_element(&constructor);
        App::mount();
    }

    fn handle_state_change(&mut self) {
        self.render();
    }

    fn handle_input_event(&mut self) {
        let new_name = self.input_element().value();
        self.state().set_name(new_name);
    }

    fn attach_listeners(&mut self) {
        let state_listener = {
            let app = self.clone();
            Box::new(move || {
                let mut app = app.clone();
                app.handle_state_change()
            }) as Box<StateListener>
        };
        let handle = self.state().add_listener(state_listener);
        self.set_state_listener_handle(handle);

        self.inner
            .lock()
            .unwrap()
            .input_element
            .add_event_listener_with_callback("input", &self.instance().unchecked_into())
            .unwrap();
    }

    fn create_app_element() -> GeneratedConstructor {
        let state = State::new(String::from("Austin"));
        let (closure, constructor) = codex_custom_elements::create_custom_element(
            move |instance, _args| App::new_with_state(instance, state.clone()),
            vec![],
        );
        closure.forget();
        constructor
    }

    fn register_app_element(constructor: &GeneratedConstructor) {
        let window = web_sys::window().unwrap();
        window
            .custom_elements()
            .define(Self::COMPONENT_NAME, constructor)
            .unwrap();
    }

    fn mount() {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        let body_html = format!("<{} />", Self::COMPONENT_NAME);
        body.set_inner_html(&body_html);
    }

    fn render(&mut self) {
        let name = self.state().name().to_uppercase();
        self.name_display_element().set_inner_text(&name);
    }

    fn new_with_state(instance: JsValue, state: State) -> Self {
        let instance: HtmlElement = instance.into();
        let shadow_root_init = ShadowRootInit::new(ShadowRootMode::Open);
        let shadow_root = instance.attach_shadow(&shadow_root_init).unwrap();
        let document = web_sys::window().unwrap().document().unwrap();
        let name_display_element: HtmlParagraphElement =
            document.create_element("p").unwrap().unchecked_into();

        let input_id = "name-input";
        let label_element: HtmlLabelElement =
            document.create_element("label").unwrap().unchecked_into();
        label_element.set_inner_text("Name");
        label_element.set_html_for(input_id);

        let input_element: HtmlInputElement =
            document.create_element("input").unwrap().unchecked_into();
        input_element.set_default_value(&state.name());
        input_element.set_id(input_id);

        shadow_root.append_child(&name_display_element).unwrap();
        shadow_root.append_child(&input_element).unwrap();

        let inner_app = InnerApp {
            instance,
            name_display_element,
            input_element,
            closures: Vec::new(),
            state,
            state_listener_handle: None,
        };

        let mut app = App {
            inner: Rc::new(RefCell::new(inner_app)),
        };

        app.attach_listeners();

        app
    }
}
