use std::time::Duration;

use leptos::{
    leptos_dom::helpers::IntervalHandle, set_interval_with_handle, RwSignal, SignalUpdate,
};

#[derive(Clone, Debug, Default)]
pub(crate) struct State {
    pub count: i32,
    pub interval_handle: Option<IntervalHandle>,
    pub name: String,
}

pub(crate) fn init_state() -> RwSignal<State> {
    let state = RwSignal::new(State::default());

    let interval_handle = set_interval_with_handle(
        move || state.update(|state| state.count += 1),
        Duration::from_millis(1000),
    )
    .unwrap();

    state.update(|state| {
        state.interval_handle.replace(interval_handle);
    });

    state
}
