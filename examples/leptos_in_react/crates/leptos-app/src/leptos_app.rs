use leptos::*;

use crate::State;

// The #[component] macro marks a function as a reusable component
// Components are the building blocks of your user interface
// They define a reusable unit of behavior
#[component]
pub fn App(state: RwSignal<State>) -> impl IntoView {
    provide_context(state);

    // the `view` macro is how we define the user interface
    // it uses an HTML-like format that can accept certain Rust values
    view! {
        <StateCounter />
    }
}

/// A component that updates the count in the global state.
#[component]
fn StateCounter() -> impl IntoView {
    let state = expect_context::<RwSignal<State>>();

    // `create_slice` lets us create a "lens" into the data
    let (count, set_count) = create_slice(
        // we take a slice *from* `state`
        state,
        // our getter returns a "slice" of the data
        |state| state.count,
        // our setter describes how to mutate that slice, given a new value
        |state, n| state.count = n,
    );

    let is_even = move || count.get() & 1 == 0;

    view! {
        <p>"Count is: " {count}</p>
        <p>
            "The number "
            <strong>{count}</strong>
            {move || if is_even() {
                " is"
            } else {
                " is not"
            }}
            " even."
        </p>
        <div>
            <button
                on:click=move |_| {
                    set_count.set(count.get() - 1);
                }
            >
                "Decrement Global Count"
            </button>
            <br/>
        </div>
    }
}
