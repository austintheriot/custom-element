mod counter;
#[cfg(test)]
mod tests;

use counter::Counter;

fn main() {
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::default());

    let component_name = "my-counter";
    let (closure, constructor) =
        custom_element::create_custom_element(Counter::new, Counter::get_observed_attributes());
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
