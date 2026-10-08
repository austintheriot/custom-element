# Shared receiver design review

The design review found no blocking defects in shared receivers or shared callable factories. Four minor requirements gaps have been corrected. One additional test insight has also been incorporated. The remaining alternatives concern ergonomics and example design, not the soundness of the chosen trait.

Scope: `2026-10-07-shared-receiver-design.md`, with current Rust code, JavaScript forwarding, examples, and tests as context. This is a specification review. Production implementation has not started.

Five reviewers covered Rust/WebAssembly, API/browser contracts, bug patterns, state ownership, and first-principles/functional-effects alternatives. The final reviewer also explored unconventional APIs explicitly.

## Findings and dispositions

| ID | Severity | Confidence | Finding | Disposition |
| --- | --- | --- | --- | --- |
| R1 | Minor | 100 | The initial factory promise included same-type recursion during browser upgrades. | Narrowed the promise to browser-permitted paths. Added separate fresh-construction and upgrade tests. Confirmed the browser restriction experimentally. |
| R2 | Minor | 94 | Counter acceptance tests omitted existing input boundaries. | Added plus/minus controls, zero and negative steps, removal, invalid numbers, and default-value requirements. |
| R3 | Minor | 90 | Direct forwarding had no equal-value attribute regression case. | Added a guarded second same-value write. Require both invocations with original arguments and bounded termination. |
| R4 | Minor | 88 | The counter migration did not establish state authority or reflection direction. | Cells drive arithmetic and rendering. Clicks commit before reflection. Attribute callbacks update and render without writing attributes back. |
| R5 | Insight | 94 | Borrow success alone does not detect stale writes after reentry. | Require outer state `1`, nested state `2`, resumed observation `2`, and final state `2`. |
| R6 | Insight | 75 | The counter can use attributes as its only state store. | Worth discussion. Keep the currently agreed cell-based example until that example choice changes. No trait change is required. |
| R7 | Insight | 65 | Shared closure handlers can remove component-struct boilerplate. | Record as a future additive adapter. No demonstrated application need justifies including it in version 0.2. |

The reviewers of R1 through R5 checked the revised requirements. All five items are resolved in the specification.

## Construction experiment

A focused Chromium experiment compared four cases with the experimental shared factory on `wasm-bindgen` 0.2.129:

| Case | Result |
| --- | --- |
| Fresh Rust-backed construction with bounded recursion | Both factory bodies ran. No browser error. |
| Upgrade of an existing Rust-backed element with the same recursion | Only the first factory body ran. The browser reported `TypeError: This instance is already constructed`. |
| Fresh plain JavaScript construction with bounded recursion | Both constructor bodies ran. |
| Upgrade of an existing plain JavaScript element with the same recursion | Only the first constructor body ran. The browser threw the same `TypeError`. |

The matched JavaScript control establishes that the upgrade restriction belongs to the browser constructor path. Shared Rust receivers do not remove that restriction.

The experiment and results remain in `/tmp/custom-element-options/upgrade-check.cjs` and `/tmp/custom-element-options/upgrade-results.json`. These temporary artifacts are not required implementation inputs. The permanent acceptance criteria describe the cases independently.

The earlier eighteen prototype scenarios remain supporting evidence for callback timing, event cancellation, short state borrows, factory reentry, and delayed-write behavior. Implementation must still test the pinned `wasm-bindgen` 0.2.100 dependency. A source inspection confirmed that its closure implementation supports `Fn`; the final design has not yet been compiled against that version.

## Alternative one Use attributes as counter state

The counter already has helpers that parse its public count and step attributes. It can reuse those helpers for both arithmetic and rendering, then delete the cached fields:

```rust
fn handle_increment_button_click(&self) {
    let count = self.get_count_attribute() + self.get_step_attribute();
    self.instance
        .set_attribute("count", &count.to_string())
        .unwrap();
}

fn render(&self) {
    let count = self.get_count_attribute();
    self.count_display_element
        .set_inner_text(&format!("Count is {count}"));
}
```

The attribute callback renders without reflecting attributes again. No new library API is necessary, and no Rust cache needs synchronization.

The costs are repeated parsing and browser access. Private state and multi-field invariants still need an application-owned representation. This choice also stops the counter from demonstrating private scalar state with `Cell`.

Recommendation: discuss this as an example simplification. Retain a separate `Cell` example if the counter adopts attributes as its state store. The sketch has not been compiled as a migrated counter.

## Alternative two Shared handler bundles

An optional adapter can implement `CustomElement` through per-instance `Fn` closures:

```rust
Handlers::new()
    .on_connected(move || render_count(&element, count.get()))
    .on_event(move |event| {
        event_count.set(event_count.get() + 1);
        event.prevent_default();
    })
```

The factory creates and shares the captured state handles. This removes the need to declare a component struct for small integrations or existing framework state.

The adapter provides no stronger borrowing guarantee than the trait. Captures can obscure ownership, and multiple handlers need cloned handles. Handler storage and cleanup still need an explicit owner.

Recommendation: defer. This is an additive convenience API and does not need a decision before the shared trait ships. The illustrative API does not exist and the sketch is uncompiled.

## Alternative three A scoped host capability

A context can expose state and browser operations through mutually exclusive borrows of the same context:

```rust
let count = {
    let mut state = context.state();
    state.count += 1;
    state.count
};
context.host().set_attribute("count", count)?;
```

If both accessors borrow the context mutably, Rust can reject simultaneous state and host access through that context. One common error becomes a compile-time error.

Captured browser handles or global browser access bypass the restriction. Complete enforcement requires wrapping browser operations and restricting escape routes. That changes the crate from a bridge into a constrained programming environment.

Recommendation: do not include this API in version 0.2. Reconsider only if enforced browser capabilities become an explicit product goal. The capability API is an uncompiled design sketch.

## Alternative four Synchronous continuations

A state callback can yield a browser operation and a continuation:

```rust
Step::set_attribute(
    "count",
    count.to_string(),
    |model, result| {
        model.last_write_succeeded = result.is_ok();
        Step::done()
    },
)
```

An interpreter releases its state borrow, executes the operation, then gives the continuation a fresh state borrow. Nested callbacks execute during the operation. The continuation observes their resulting state and receives the browser operation's result.

This handles results and fresh state more explicitly than a list of commands. It also requires an interpreter, effect definitions, continuation storage, error handling, and ordering rules. Arbitrary external calls inside state callbacks remain outside the guarantee.

Recommendation: keep this as a separate experimental architecture. The current crate does not justify that runtime. The illustrative API does not exist and the sketch is uncompiled.

## Review conclusion

The chosen trait remains the smallest change that removes the library's exclusive callback and factory borrows while preserving synchronous forwarding. No reviewer identified a need for a library-owned state helper, scheduler, new dependency, or different component owner.

The immediate discussion item is the counter example's representation. The specification currently retains `Cell` fields. Handler bundles remain a later ergonomic option. Scoped capabilities and continuations have stronger local structure but exceed the agreed thin-wrapper scope.

The implementation plan must turn the specified browser behavior into failing and passing tests. No implementation, release, publication, or claim of cross-browser completion follows from this review.
