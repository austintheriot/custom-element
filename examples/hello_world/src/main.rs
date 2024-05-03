use custom_element::CustomElement;
use js_sys::Array;
use wasm_bindgen::JsValue;
use web_sys::HtmlElement;

struct HelloWorld;

// all custom element lifecycle hooks have a default
// no-op implementation
impl CustomElement for HelloWorld {}

impl HelloWorld {
    // called from the JavaScript custom element's `constructor`
    fn new(instance: JsValue, _args: Array) -> Self {
        let instance: HtmlElement = instance.into();
        instance.set_text_content(Some("Hello, world!"));
        HelloWorld
    }
}

fn main() {
    // create custom element constructor
    let component_name = "hello-world";
    let (closure, constructor) = custom_element::create_custom_element(HelloWorld::new, vec![]);
    // we want our custom element to live forever
    closure.forget();

    // define the element
    let window = web_sys::window().unwrap();
    window
        .custom_elements()
        .define(component_name, &constructor)
        .unwrap();

    // render in the DOM
    let body = window.document().unwrap().body().unwrap();
    body.set_inner_html(&format!("<{0}></{0}>", component_name));
}
