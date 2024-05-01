custom-elements-rs

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

Install wasm-bindgen-cli (for generating the JS bindings to use the .wasm files in the browser)

Note: must match version of `wasm-bindgen` installed locally.

```sh
cargo install wasm-bindgen-cli --vers "0.2.92"
```

Install Firefox's `geckodriver` to your $PATH (for running wasm tests in headless mode): https://github.com/mozilla/geckodriver/releases

`geckodriver` must be in your $PATH, so that it can be executed from `cargo make`.

Build binaryen: https://github.com/WebAssembly/binaryen (for optimizing the generated wasm)

After building, install to /usr/local/bin by running the following in the `binaryen` directory

```sh
make install
```

Install cargo-make

```sh
cargo install cargo-make
```

Install cargo-watch (for rebuilding on edits to Rust source files)

```sh
cargo install cargo-watch
```

Install nvm (for running the codex-web app)

- Unix: https://github.com/nvm-sh/nvm
- Windows: https://github.com/coreybutler/nvm-windows

Install the node version listed in the ./package.json file

Install Yarn

```sh
npm install --global yarn
```

Install all JS dependencies

```sh
yarn
```

## Run

To run the web app, run the following in the root directory

```sh
# compiles Rust code on file changes (with --profile=dev)
cargo make watch-dev

# runs Vite development server for web app
yarn web-dev
```

## Test

```sh
# tests codex-custom-elements package tests in a headless browser
cargo make test-custom-elements
```
