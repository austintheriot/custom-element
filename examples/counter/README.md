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

```sh
trunk serve --open
```
