# Counter

## Set up

Install rust using [rustup](https://www.rust-lang.org/tools/install)

If Rust is already installed, update to the latest version

```sh
rustup update
```

Install the wasm32-unknown-unknown target (for compiling Rust code to .wasm)

```sh
rustup target add wasm32-unknown-unknown
```

Install [trunk](https://trunkrs.dev/)

```sh
cargo install --locked trunk
```

## Run

To run the web app, run the following in the root directory of this crate

```sh
trunk serve --open
```

## Counter attributes

The `count` and `step` attributes hold the counter state. The example starts
with `count="0"` and `step="1"`. The increment, plus, and minus buttons update
those attributes. An external attribute change updates the display before the
attribute operation returns, including when the element is detached.

If an attribute is missing or is not a signed 32-bit integer, the counter uses
0 for `count` or 1 for `step`. Rendering leaves the original attribute value
unchanged. Removing an attribute also leaves it absent until a button changes
that attribute.

The callbacks use shared `&self` receivers. The component reads its current
values from attributes instead of keeping Rust state. A nested attribute
callback sees the latest attribute value without a `RefCell` borrow. The
example does not cover compound Rust state. See the
[Cell counter](../cell_state) and [scoped RefCell counter](../refcell_state)
for those strategies.
