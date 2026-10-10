use std::cell::Cell;

thread_local! {
    static NEXT_ELEMENT_NUMBER: Cell<u32> = const { Cell::new(0) };
}

pub fn unique_custom_element_name() -> String {
    let number = NEXT_ELEMENT_NUMBER.with(|next| {
        let current = next.get();
        next.set(current + 1);
        current
    });
    format!("test-element-{number}")
}
