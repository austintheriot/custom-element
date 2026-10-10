# Setup

Install Rust with [rustup](https://www.rust-lang.org/tools/install). Then install the WebAssembly target and browser test tools:

```sh
rustup update
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.100 --locked
cargo install cargo-make
```

The `wasm-bindgen` CLI and test runner must report version 0.2.100. The matrix script checks both versions before tests run.

Install Firefox, Chrome, `geckodriver`, and `chromedriver`. Both drivers must be on `PATH`. Check that each driver supports the installed browser version.

Run wasm tests from inside this repository. The root `.cargo/config.toml` sets the `wasm-bindgen-test-runner` runner. Outside the repository, `cargo test --target wasm32-unknown-unknown` has no runner.

If a browser binary is outside its driver's default search path, set `BROWSER_BINARY` to the binary path. The script then writes a `webdriver.json` file in the root directory and in each example directory that tests in a browser. The script deletes those files when it exits. The script stops if one of those files already exists. When `BROWSER_BINARY` is not set, the script writes no files.

The root browser suite reports whether the browser supports native move. When the browser reports `native move capability: unsupported`, the script fails. Set `ALLOW_MISSING_MOVE=1` to accept that result. The script prints which case applied.

Run the native tests and all four browser/profile pairs with:

```sh
cargo make test
```

Run one browser/profile pair during development with:

```sh
BROWSER_BINARY=/path/to/firefox bash scripts/test-browser.sh firefox debug
```

Install `cargo-watch` if you use the `cargo make watch` task.
