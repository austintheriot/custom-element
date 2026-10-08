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
