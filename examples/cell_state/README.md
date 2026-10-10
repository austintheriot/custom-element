# Cell state counter

This example keeps the counter value in a Rust `Cell<u32>`. A click updates the
cell before it writes the reflected `count` attribute. The attribute callback
handles reflected writes and external writes. The callback also updates the
display before the attribute operation returns.

Each callback takes `&self`. `Cell<u32>` lets the click handler store the
new count before `set_attribute` synchronously invokes the attribute
callback. `Cell` fits a copyable scalar. Use the
[scoped RefCell example](../refcell_state) for compound state. The
[attribute-only example](../counter) stores no Rust counter value.

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

The page creates one `sample-cell-counter` element. Click its button to
increment the display. Set its `count` attribute from browser tools to update
the cell and display. Invalid values and attribute removal reset the value to
zero. The next click writes the new value back to the attribute.
