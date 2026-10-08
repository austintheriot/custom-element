# Shared receiver callbacks for custom elements

The current bridge borrows a custom element exclusively for each callback. A callback that changes an observed attribute can synchronously reenter that bridge and fail its borrow check. Version 0.2 will use shared receivers for callbacks and shared callable factories. Applications will control mutation through standard Rust types.

The intended outcome is synchronous browser integration without mandatory scheduling around ordinary element operations. The library will remain a thin wrapper. It will remove its own exclusive borrows rather than introduce a callback queue or a state framework.

## Agreed scope

A breaking change is acceptable. Keep the `CustomElement` name and replace its receiver contract. Do not maintain a second legacy trait.

Use `Cell` for independent scalar values and short `RefCell` borrows for compound state. The library does not manage application state. An application must release every conflicting state borrow before an external call that can reenter the component.

Shared receivers permit reentry. They do not promise that arbitrary application code is reentrant, that nested callbacks terminate, or that interrupted state transitions remain consistent.

## Public callback contract

All six trait methods use `&self`. Keep their names, argument types, unit return values, and default no-op implementations.

```rust
pub trait CustomElement {
    fn connected_callback(&self) {}

    fn disconnected_callback(&self) {}

    fn adopted_callback(&self) {}

    fn connected_move_callback(&self) {}

    fn handle_event(&self, _event: web_sys::Event) {}

    fn attribute_changed_callback(
        &self,
        _name: &str,
        _old_value: wasm_bindgen::JsValue,
        _new_value: wasm_bindgen::JsValue,
    ) {
    }
}
```

The corresponding exported `Bridge` methods also use `&self`. The bridge continues to own `Box<dyn CustomElement>`. Shared method receivers do not require `Rc`, a mutex, a registry, or an outer `RefCell`.

Keep the existing JavaScript method names and direct forwarding. Preserve support for autonomous custom elements and customized built-in elements. Keep `CustomElementConfiguration`, generated constructors, constructor arguments, and the returned factory/constructor tuple unchanged in shape.

The public `Bridge::new` and `From` construction paths retain their current meaning. Direct JavaScript calls through `element.struct` use the same shared bridge methods as browser callbacks.

## Factory contract and lifetime

Change the factory closure type from `Closure<dyn FnMut(JsValue, Array) -> JsValue>` to `Closure<dyn Fn(JsValue, Array) -> JsValue>`. Both creation functions require `Factory: Fn(JsValue, Array) -> Component + 'static`, with `Component: CustomElement + 'static`.

Update the closure type wherever it appears, including `CustomElementResult.closure`. Keep the existing closure ownership convention. Callers must retain the returned closure or explicitly forget it for the registered class lifetime.

The factory removes the Rust closure restriction on bounded recursive construction. During fresh construction, a nested invocation can create another instance of the same registered type. Each successful invocation produces a separate component and bridge. No mutable factory borrow remains active across a nested invocation.

Browser construction rules still apply. During an upgrade of an existing element, same-type recursive construction can fail before entering the Rust factory. The browser can reject its construction stack marker with a `TypeError`. The shared factory does not bypass that restriction. Test fresh construction and upgrade behavior separately.

Captured factory state follows the same borrowing discipline as component state. A captured `Cell` can supply a recursion bound or a sequence number. A captured `RefCell` guard must end before constructing another instance.

The receiver change does not relax browser constructor restrictions. It does not add support for callbacks that access the outer instance before its factory has returned and assigned `element.struct`.

## State ownership and browser effects

For independent scalar state, the application can use `Cell` without retaining a borrow guard:

```rust
struct Counter {
    count: std::cell::Cell<u32>,
    element: web_sys::HtmlElement,
}

impl CustomElement for Counter {
    fn connected_callback(&self) {
        let count = self.count.get() + 1;
        self.count.set(count);
        self.element
            .set_attribute("count", &count.to_string())
            .unwrap();
    }
}
```

This example demonstrates borrowing only. Applications choose their own initialization, overflow, and browser-error policies.

For compound state, compute owned output in an explicit scope before browser effects:

```rust
fn connected_callback(&self) {
    let label = {
        let mut state = self.state.borrow_mut();
        state.count += 1;
        format!("Count: {}", state.count)
    };

    self.element.set_text_content(Some(&label));
}
```

The returned `String` owns its data. No `Ref` or `RefMut` guard crosses the browser call. A shared state borrow can also conflict with a nested mutable borrow.

Commit consistent state before calling external code. Nested callbacks can change that state before the external call returns. Read state again after the call if subsequent work depends on its current value. Avoid restoring an earlier snapshot over a nested update.

Use `Rc` only when state actually has multiple owners. Do not replace every component with `Rc<RefCell<Component>>`, and do not use `RefCell::get_mut` to assume exclusive access from a shared receiver.

Application-created `FnMut` closures retain their own reentry restrictions. Changing this crate's callback receivers does not change independent event listeners or framework closures.

## Timing and event behavior

The wrapper invokes each Rust callback immediately when the browser invokes the corresponding JavaScript method. The library adds no microtask, queue, batching, coalescing, or retry.

For a synchronous observed-attribute update during connection, the required trace is:

```text
connected:start
attribute_changed
connected:end
```

For a synchronous cancelable event dispatched during a callback, the event handler runs during that dispatch. Calls to `preventDefault` and `stopImmediatePropagation` retain their dispatch-time effects. The handler receives the live event context, including `currentTarget` and `composedPath`.

The library does not impose a new global order across elements. It forwards the browser's invocations in place. Existing lifecycle forwarding remains intact, including the prior fix that routes disconnection to `disconnectedCallback`.

## Errors and limits

Keep the existing callback return types and browser error behavior. Do not add a catch-and-retry layer or convert an application borrow failure into a skipped callback.

An overlapping application `RefCell` borrow remains an application error. A callback can use `try_borrow_mut` when it has an explicit way to handle contention. The library provides no recovery guarantee after a panic or a WebAssembly trap.

Browser callback errors can be reported globally without making `append_child` or `set_attribute` return an error. Tests must observe callback effects and browser errors. A successful browser operation alone does not establish successful callback delivery.

Equal-value attribute writes can still trigger application callbacks. Applications must avoid unbounded reflection loops. Shared receivers remove the borrow barrier that previously stopped some loops.

## Migration and repository changes

Target version 0.2.0. This design authorizes no publication or external announcement.

Implementation scope includes:

- `src/custom_element.rs`: shared trait receivers and borrowing documentation.
- `src/bridge.rs`: shared exported receivers, with current ownership preserved.
- `src/create_custom_element.rs`: `Fn` closure type and bounds throughout the factory API.
- `src/lib.rs` and `README.md`: migration guidance, examples, and a precise replacement for the existing borrow warning.
- `tests/common/helpers.rs` and affected tests: update receivers and add browser regression coverage.
- The hello-world, counter, and Leptos examples: migrate every affected trait implementation and check each example against the local crate.
- Root package metadata and lockfile: record the incompatible version change without an unrelated dependency upgrade.

Convert mutable scalar fields in the counter example to `Cell`, and convert its internal methods to shared receivers where appropriate. Remove the microtask scheduling used solely to work around bridge borrowing. Remove closure storage used solely by that scheduling.

Keep the counter's public attributes and interaction behavior. Use its `Cell<i32>` values for arithmetic and rendering. A click reads those cells, commits the changed value, then reflects that value to the corresponding attribute.

An attribute callback parses the incoming value, updates the corresponding cell, and renders. It must not write the attribute back. Render even when the parsed value equals the cell value, because a click can have precommitted that value before the callback runs.

Preserve defaults of count `0` and step `1` when attributes are absent, removed, or invalid. Preserve signed step values, including zero and negative values. Process two immediate increment events without losing an increment. Preserve plus/minus controls and externally assigned attributes.

A future simplification can remove the cached fields and derive arithmetic and rendering directly from attributes. That option requires no new library API. The current specification retains cells to demonstrate private mutable state and keeps this alternative separate from the accepted callback contract.

Changing an implementation method from `&mut self` to `&self` is source-incompatible. Factory closures that require mutable captured access also need migration. Document `Cell` and scoped `RefCell` examples for both cases.

Do not change unrelated framework state APIs or constructor ownership. Do not add a dependency to replace standard interior mutability types.

## Acceptance tests

Use the existing `wasm-bindgen-test` browser harness. New behavioral tests must fail against the current implementation for the missing behavior, then pass after the change. Adapt fixture signatures as needed to demonstrate the runtime failure before changing receiver contracts.

| Case | Required observation |
| --- | --- |
| Nested observed-attribute update | Connection changes an observed attribute. The attribute callback completes before connection resumes. Capture zero unexpected browser errors. |
| Nested mutable application state | The outer callback commits `1` and releases its guard. The nested callback changes that state to `2`. The resumed outer callback reads `2`, and the final state remains `2`. |
| Equal-value attribute writes | A nested callback performs one guarded write of the same observed value. Assert both callback deliveries and their original arguments. Terminate after the bounded second delivery. |
| Borrow discipline boundary | Hold a guard deliberately and use `try_borrow_mut` in the nested callback. Detect contention without panicking or poisoning the browser suite. |
| Nested cancelable event | The handler sees a non-null `currentTarget` and a nonempty event path. Dispatch returns false after cancellation. A second listener does not run after immediate propagation stops. |
| Recursive same-type construction | During fresh construction, a bounded factory creates one further instance. Both invocations complete, with distinct component state and no recursive-closure error. |
| Upgrade construction boundary | An ordinary preexisting element upgrades successfully. A separate bounded same-type recursion case preserves the browser construction failure. Compare that failure with a plain JavaScript control and distinguish it from a Rust closure error. |
| Ordinary factory inputs | Constructor arguments and selected element constructors still reach the factory unchanged. |
| Lifecycle forwarding | Preserve existing connection, disconnection, adoption, attribute, and instance tests. Cover move-callback forwarding without requiring a browser that implements native moves. |
| Immediate counter updates | Two synchronous increments produce two increments and an updated display before returning. A reflected value equal to its precommitted cell value still renders. |
| Counter input boundaries | Exercise plus/minus controls, zero and negative steps, valid external changes, attribute removal, and invalid numeric values. Assert state and display, including defaults of count `0` and step `1`. |
| Independent instances | Updating one instance does not mutate another instance's private state. |

Capture event properties inside the handler, rather than inspecting the retained event after dispatch. Check the dispatch return value and second-listener count, not only the eventual `defaultPrevented` flag.

Test fixtures must bound recursion, release listeners and retained closures when safe, and remove their elements. A registered custom element cannot be unregistered. Keep its factory callable for the duration of each browser test realm.

Run the existing native and browser suites. Check all local example crates for the WebAssembly target. Run the new browser regressions in development and release builds. Keep `wasm-bindgen` library and CLI versions aligned. Do not upgrade the dependency solely to implement shared receivers.

The existing lockfile uses `wasm-bindgen` 0.2.100. Validate the final design against that version. Keep the temporary experiments on 0.2.129 as supporting evidence rather than a replacement for repository tests.

## Prior experimental evidence

The original failure reproduced in Chromium 153 with `wasm-bindgen` 0.2.100 and 0.2.129. Eighteen later prototype scenarios ran on 0.2.129 in development builds.

Shared trait and bridge receivers allowed synchronous nested callbacks after state borrows ended. A held inner borrow still produced a conflict. Callback serialization avoided the bridge error but changed event cancellation and context. An `Fn` factory allowed bounded recursive construction where the current `FnMut` factory failed.

Four further construction scenarios compared fresh creation and upgrades in Rust-backed and plain JavaScript elements. Fresh same-type recursion succeeded in both controls. Upgrade-time recursion failed in both controls with a browser `TypeError` before the second factory body ran.

These experiments establish feasibility and delimit the factory guarantee. They do not replace the acceptance tests above. Firefox, Safari, release builds, and panic recovery were not covered by those prototypes.

## Alternatives considered

Callback serialization preserves mutable implementations but changes nested timing and event behavior. It is not part of this design.

A generic state-update helper cannot stop its closure from calling browser code through captured handles. An update/effect helper can guarantee that its own borrow ends between phases, but it cannot make arbitrary update closures effect-free.

A required command/effect architecture introduces a broader component framework. Keep that pattern available to applications without imposing it through this trait.

Do not add an automatic state wrapper, a second trait, a proc macro, or a callback policy field in this version. Reconsider a helper only when migrated examples demonstrate repeated application code that standard types do not express clearly.

## References

- [Rust interior mutability](https://doc.rust-lang.org/std/cell/index.html)
- [HTML element constructor algorithm](https://html.spec.whatwg.org/multipage/dom.html#htmlconstructor)
- [HTML custom element reactions](https://html.spec.whatwg.org/multipage/custom-elements.html#custom-element-reactions)
- [Document Object Model event dispatch](https://dom.spec.whatwg.org/#concept-event-dispatch)
