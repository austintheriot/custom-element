# PR #7 review fixes: implementation plan

Spec: `docs/superpowers/reviews/2026-10-10-pr-7-review.md`. Finding numbers
below refer to that file. Its "Rulings" section is binding: do not
implement a rejected finding.

## Global constraints

- Branch `feat/shared-receivers`. Commit in small conventional commits that
  match the history (`ci:`, `docs:`, `test:`, `refactor:`, `build:`,
  `fix:`). End every commit message with:

  ```
  Co-Authored-By: Claude Opus 5.5 <noreply@anthropic.com>
  Claude-Session: https://claude.ai/code/session_0135We8Th3zUySHXJYvXcmAs
  ```

- Never use `--no-verify`. Never push.
- Test-driven development (`superpowers:test-driven-development`) for every
  behavior test you add. A test for behavior that already works must be
  shown red first by a temporary mutation of the code under test. Revert
  the mutation, then show it green. Record both runs in the report.
- Pure refactors and CI or documentation edits skip TDD. Say so in the
  report.
- No code comments unless the reason is not visible in the code.
- No single-letter names except numeric loop indices. No em dashes in any
  prose or commit message.
- Prose in Markdown and rustdoc: short sentences, active voice, no
  semicolons, no "e.g." or "etc.".
- Native tests: `cargo test` (root). Browser tests on this machine run in
  Firefox only (no chromedriver):

  ```
  bash scripts/test-browser.sh firefox debug
  ```

  Run it with a directory that holds wasm-bindgen CLI 0.2.100 on `PATH`.
  The script requires that version. The script runs the root suite and
  all three state examples.
- Leave `git status --porcelain` empty when done, except for intended
  commits.

## Task 1: CI and scripts

Findings 1-9 and the status-check finding.

- `.github/workflows/tests.yml`:
  - Add a job named exactly `build-and-test` that `needs:
    [native-tests, browser-tests]`, runs `if: always()`, and fails unless
    both needed jobs report `success`. The `main` ruleset requires this
    check name.
  - Re-pin `browser-actions/setup-chrome` to
    `48ad923757ca74d66703209fe939badbdf80f2f4` and
    `browser-actions/setup-firefox` to
    `0bc507ddf224827e3b1af68e014d5e42ab93e795`. Add a trailing
    `# vX.Y.Z` comment on every pinned `uses:` line. Look up the tag for
    `actions/checkout` (`d23441a…`, v6.1.0) and `setup-geckodriver`
    (`fac5b04…`) with `gh api`.
  - Add `concurrency: { group: ${{ github.workflow }}-${{ github.ref }},
    cancel-in-progress: true }`.
  - Rename the matrix value `chromium` to `chrome` everywhere it appears
    (workflow, script, `Makefile.toml`, `SETUP.md`, any README).
  - Remove the inline Python step that writes `webdriver.json`. The
    script owns that file now (next bullet).
- `scripts/test-browser.sh`:
  - Derive the example manifest list in one place. Glob
    `examples/*/Cargo.toml` and keep the crates whose `Cargo.toml` lists
    `wasm-bindgen-test` under `[dev-dependencies]`. The root manifest
    runs first.
  - When `BROWSER_BINARY` is set, write `webdriver.json` with the
    browser capabilities (the same keys and flags the removed Python step
    wrote) into the repository root and each selected example directory,
    and delete them on exit. When it is not set, write nothing.
  - Fail when the root suite reports `native move capability:
    unsupported`, unless `ALLOW_MISSING_MOVE=1` is set. Print which case
    applied.
- `Makefile.toml`: delete `[tasks.test-wasm]`. Rename `chromium` entries.
- `.github/dependabot.yml`: weekly `github-actions` updates.
- `CONTRIBUTING.md`: say that `scripts/test-browser.sh` holds the browser
  runner and that `cargo make` tasks call it.
- `SETUP.md`: drop the `test-wasm` sentence. Describe `BROWSER_BINARY`
  and `ALLOW_MISSING_MOVE`. Say that wasm tests must run from inside the
  repository, because the runner comes from the root
  `.cargo/config.toml`.
- Verify: `bash -n scripts/test-browser.sh`, a YAML parse of the workflow
  (`ruby -ryaml -e 'YAML.load_file(ARGV[0])'` or Python), and the Firefox
  debug run above.

## Task 2: Packaging, rustdoc, and changelog

Findings 12-18, 25 (library and README part), and 26.

- `Cargo.toml`: add `exclude = ["docs/", "scripts/", ".github/",
  ".cargo-husky/"]`. Verify with `cargo package --list --allow-dirty`
  that none of those paths remain.
- `src/lib.rs` `Shared callbacks` doctest: stop importing `Bridge`. Call
  the trait method on the component directly
  (`CustomElement::connected_callback(&visits)`), and keep the assertion.
- `src/create_custom_element.rs` rustdoc:
  - Replace "fails with a browser `TypeError`" with the README's
    wording: the browser reports a `TypeError` and the nested call returns
    an element with no Rust component.
  - Add one sentence: the factory must not add attributes to the element,
    because the browser rejects a constructor that adds attributes.
  - In the doc example, bind the returned value as `closure`, not
    `factory`. Keep the parameter name `factory`.
- `src/bridge.rs`: rename generic `C` to `Component` in `Bridge::new` and
  `From<C> for Bridge`.
- `README.md`: rename returned-value bindings from `factory` to `closure`.
  Add one sentence near the reentry section: the shared receiver removes
  the nested-call error that wasm-bindgen issue #1578 caused for mixed
  `&mut self` and `&self` calls. Link
  https://github.com/rustwasm/wasm-bindgen/issues/1578.
- `CHANGELOG.md` (new, Keep a Changelog format) with a `0.2.0` entry dated
  2026-10-10. List under "Changed": callbacks take `&self`, factories are
  `Fn`, and the returned closure type is `Closure<dyn Fn(JsValue, Array)
  -> JsValue>`. Include one before/after snippet that moves a mutable
  capture into `Cell`. Link it from the README migration section.
- Verify: `cargo test` (runs the doctests) and the Firefox debug run.

## Task 3: Test infrastructure

Findings 15, 19, 20, 21, 25 (tests part), and 31.

- One Rust module binds `tests/common/browser.js`. Move the `extern`
  blocks from `tests/events.rs`, `tests/factories.rs`, and
  `tests/forwarding.rs` into `tests/common/browser.rs` (declared from
  `tests/common/mod.rs`). Test files call those functions through
  `common::browser`.
- Rename the Rust wrapper `observe_errors` to `capture_browser_errors`.
  Remove the dead-code warnings without `#[allow]`: a function that a
  test target does not use moves behind the module that uses it, or the
  common module is organized so each item is used by every target that
  compiles it. Report how you did it.
- Remove the duplication between `Recorder` (`tests/common/contracts.rs`)
  and `CustomElementCallCounter` (`tests/common/helpers.rs`). Express the
  call counters through a `Recorder` hook and delete the duplicate
  `CustomElement` implementation. `tests/instances.rs` keeps its
  assertions unchanged.
- Rename `Call::Moved` to `Call::ConnectedMove`.
- Rename returned-value bindings from `factory` to `closure` in `tests/`.
- TDD: add a browser test in `tests/factories.rs` for finding 15. A
  factory that creates another element of its own registered type with a
  fresh `document.create_element`, outside an upgrade, gets a fully
  constructed element (it has its component), and no browser error is
  reported. Show it red with a temporary mutation (for example, make the
  JS shim's constructor skip `makeStruct` when a module-level flag says a
  construction is already running), then green.
- Verify: `cargo test` and the Firefox debug run.

## Task 4: Examples

Findings 22, 23, 24, 25 (examples part), 27, 28, 29, and 30.

- Rename `examples/counter/src/counter.rs` to `component.rs` and update
  its module path.
- Drop the `get_` prefix from the counter's accessors
  (`get_count_attribute`, `get_step_attribute`,
  `get_observed_attributes`) and update callers.
- Delete the `[[bin]]` tables from `examples/cell_state/Cargo.toml` and
  `examples/refcell_state/Cargo.toml`. Run `trunk build` in each example
  to confirm it still builds, and check that `Cargo.lock` does not change
  beyond the package entry.
- Fixture names: one fixture type named `Fixture` per example, one
  constructor named `from_registration`.
- Rename returned-value bindings from `factory` to `closure` in each
  example's `main.rs` and tests.
- TDD, each red by temporary mutation first:
  - Counter: click each of the three buttons while the element is
    detached, and assert that `count` and `step` do not change. Mutation:
    delete one `remove_event_listener_with_callback` call. Rename
    `detached_attribute_change_renders_and_reconnection_keeps_one_listener`
    so the name claims only what it checks.
  - `cell_state` and `refcell_state`: register the element as a `click`
    listener on its own host as well, dispatch a click on the host, and
    assert the count does not change. Mutation: replace the
    `Object::is(&target, &self.button)` check with `.is_some()`.
  - Counter: dispatch a non-click event (`new Event("input")`) on each
    button after registering the element as a listener for that event
    type, and assert nothing changes. Mutation: change the `"click"` arm
    to `_`.
- Verify: the Firefox debug run (it runs each example's tests).
