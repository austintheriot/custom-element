# Setup

Install Rust with [rustup](https://www.rust-lang.org/tools/install). Then install the WebAssembly target and browser test tools:

```sh
rustup update
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --version 0.2.100 --locked
cargo install cargo-make
```

The `wasm-bindgen` CLI and test runner must report version 0.2.100. The matrix script checks both versions before tests run.

Install Firefox, Chromium, `geckodriver`, and `chromedriver`. Both drivers must be on `PATH`. Check that each driver supports the installed browser version.

If a browser binary is outside its driver's default search path, create `webdriver.json` in the root directory and each state example directory. Those directories are `examples/counter`, `examples/cell_state`, and `examples/refcell_state`.

The test runner reads each package's file. The matrix script does not change the files. For Chromium, set `goog:chromeOptions.binary`. For Firefox, set `moz:firefoxOptions.binary`. The repository ignores these machine-specific files.

Run the native tests and all four browser/profile pairs with:

```sh
cargo make test
```

Run one browser/profile pair during development with:

```sh
bash scripts/test-browser.sh firefox debug
```

The focused root browser task remains `cargo make test-wasm`. Install `cargo-watch` if you use the `cargo make watch` task.
