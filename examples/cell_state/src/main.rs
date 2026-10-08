mod component;

use component::CellCounter;

fn main() {
    let (factory, constructor) =
        custom_element::create_custom_element(CellCounter::new, CellCounter::observed_attributes());
    factory.forget();

    let browser = web_sys::window().unwrap();
    let document = browser.document().unwrap();
    let name = "sample-cell-counter";
    browser
        .custom_elements()
        .define(name, &constructor)
        .unwrap();
    let element = document.create_element(name).unwrap();
    document.body().unwrap().append_child(&element).unwrap();
}
