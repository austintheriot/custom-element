use custom_element::CustomElement;

pub struct MockCustomElement;

impl CustomElement for MockCustomElement {
    fn observed_attributes() -> &'static [&'static str] {
        &["count"]
    }
}
