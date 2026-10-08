# Scoped RefCell counter

This example keeps `count` and `updates` in one Rust `RefCell<Model>`. A click
changes both fields in one short mutable borrow. The click handler creates an
owned display label and releases the borrow before it changes the display or
the observed `count` attribute. The attribute change calls the same component
again before the click handler returns.

An external `count` change updates the model and display. Removing the
attribute, or setting an invalid number, gives a count of zero. A write that
keeps the same parsed count refreshes the display without increasing `updates`.

Do not keep a `RefCell` guard while you call browser code that can invoke a
component callback. A nested callback that borrows the model will fail if the
outer callback still holds a conflicting guard. The root contention tests
show that failure with `try_borrow` and `try_borrow_mut` without making this
example fail.

A live shared guard also conflicts with a nested mutable borrow. `Rc` can
share ownership of a model, but it does not remove either borrow conflict.
State can change in a nested callback. Read the model again after browser
code if later work needs the current value. Dropping a scope-local reference
to `self` does not release a borrow held by an exported wrapper. Browser
constructor upgrade rules independently restrict recursive element upgrades.
See the [Cell counter](../cell_state) for scalar state and the
[attribute-only counter](../counter) for state held in the document.

## Set up

Install Rust and the WebAssembly target:

```sh
rustup target add wasm32-unknown-unknown
```

Install [Trunk](https://trunkrs.dev/):

```sh
cargo install --locked trunk
```

## Run

From this directory, start the example:

```sh
trunk serve --open
```

The page creates one `sample-compound-counter` element. Click its button to
increment the count. Set its `count` attribute in browser tools to change the
model externally.
