use std::{cell::RefCell, rc::Rc};

use web_sys::Element;

use super::recorder::{create_recorder, Call};

#[derive(Clone, Default)]
pub struct CallCounters {
    pub num_calls_connected: Rc<RefCell<u32>>,
    pub num_calls_disconnected: Rc<RefCell<u32>>,
    pub num_calls_adopted: Rc<RefCell<u32>>,
    pub num_calls_attribute_changed: Rc<RefCell<u32>>,
}

pub fn create_custom_element_with_call_counters() -> (Element, CallCounters) {
    let call_counters = CallCounters::default();
    let counted = call_counters.clone();
    let instance = create_recorder(Rc::new(move |_element, call| {
        let counter = match call {
            Call::Connected => &counted.num_calls_connected,
            Call::Disconnected => &counted.num_calls_disconnected,
            Call::Adopted => &counted.num_calls_adopted,
            Call::Attribute { .. } => &counted.num_calls_attribute_changed,
            Call::ConnectedMove | Call::Event(_) => return,
        };
        *counter.borrow_mut() += 1;
    }));
    (instance.into(), call_counters)
}
