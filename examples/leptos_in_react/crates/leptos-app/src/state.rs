use std::{ops::Deref, time::Duration};

use leptos::{
    leptos_dom::helpers::IntervalHandle, set_interval_with_handle, RwSignal, SignalUpdate,
    SignalWith,
};
use wasm_bindgen::prelude::*;

#[derive(Debug, Default)]
pub struct InnerState {
    pub count: i32,
    pub interval_handle: Option<IntervalHandle>,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
pub struct State(RwSignal<InnerState>);

impl Default for State {
    fn default() -> Self {
        Self(RwSignal::new(InnerState::default()))
    }
}

/// Public API, callable from React
#[wasm_bindgen]
impl State {
    #[wasm_bindgen]
    pub fn count(&self) -> i32 {
        self.0.with(|state| state.count)
    }

    #[wasm_bindgen(js_name = incrementCountBy)]
    pub fn increment_count_by(&mut self, increment_amount: i32) {
        self.update(|state| state.count += increment_amount);
    }
}

impl Deref for State {
    type Target = RwSignal<InnerState>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub(crate) fn init_state() -> State {
    let state = State::default();

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
