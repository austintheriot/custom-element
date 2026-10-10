# Shared receiver callbacks for custom elements

The current bridge borrows a custom element exclusively for each callback. A callback that changes an observed attribute can synchronously reenter that bridge and fail its borrow check. Version 0.2 will use shared receivers for callbacks and shared callable factories. Applications will control mutation through standard Rust types.

The intended outcome is synchronous browser integration without mandatory scheduling around ordinary element operations. The library will remain a thin wrapper. It will remove its own exclusive borrows rather than introduce a callback queue or a state framework.

## Agreed scope

A breaking change is acceptable. Keep the `CustomElement` name and replace its receiver contract. Do not maintain a second legacy trait.

Provide runnable examples of attribute-owned state, `Cell` scalar state, and scoped `RefCell` compound state. These application patterns do not add library API surface. Use `Cell` for independent scalar values and short `RefCell` borrows for compound state. The library does not manage application state. An application must release every conflicting state borrow before an external call that can reenter the component.

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
struct ScalarCounter {
    count: std::cell::Cell<u32>,
    element: web_sys::HtmlElement,
}

impl CustomElement for ScalarCounter {
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
- `examples/cell_state` and `examples/refcell_state`: add runnable examples of scalar and compound Rust state. Both examples use the local crate and include behavior tests.
- Root package metadata and lockfile: record the incompatible version change without an unrelated dependency upgrade.

Use attributes as the existing counter example's sole state source. Remove its cached `count` and `step` fields. Reuse the attribute parsing helpers for arithmetic and rendering. A click reads the current attributes and writes the changed attribute synchronously.

An attribute callback renders from current attributes. It must not write attributes back or maintain another copy of the values. Remove microtask scheduling and closure storage used solely to work around bridge borrowing.

Preserve defaults of count `0` and step `1` when attributes are absent, removed, or invalid. Preserve signed step values, including zero and negative values. Process two immediate increment events without losing an increment. Preserve plus/minus controls and externally assigned attributes.

The attribute-only counter is an example choice. It does not restrict applications to browser-owned state or change any factory or trait signature.

## Required examples

Each example must run in a browser and include setup instructions, its state authority, and the relevant borrowing rule. Compile and test the same component code that the example executes. Do not substitute a copied test fixture for the actual example behavior.

| Example | State authority | Required demonstration |
| --- | --- | --- |
| `examples/counter` | Parsed `count` and `step` attributes | No Rust value cache. Clicks update attributes directly. Attribute callbacks render without reflection loops. |
| `examples/cell_state` | `Cell` scalar state | A handler commits a value before a browser operation. A synchronous nested callback reads the committed value without a borrow guard. |
| `examples/refcell_state` | One compound `RefCell` model | Update related fields in one short scope and return an owned label. Release the guard before rendering or changing an observed attribute. A nested callback can borrow the model. |

The scalar example retains the reviewed precommit pattern. An application action updates its cell, then reflects the value to an observed attribute. The attribute callback parses external input, updates the cell, and renders without writing the attribute back. Render even when the parsed value equals the precommitted cell value.

The compound example must make both ownership and effect boundaries visible. Use related fields, such as a count and its update count, and produce an owned `String` while borrowing. Execute browser effects after the explicit scope ends. Include an observed-attribute change or event dispatch that actually reenters the same component. Its nested callback reads the complete committed state. A text update alone does not establish reentry. Do not render an earlier owned snapshot over a newer nested update. Keep the demonstration small and avoid introducing a state helper or framework.

The normal compound example must succeed. Put deliberately held-borrow cases in tests and explain their failure mode in the example documentation. Do not make the example panic to demonstrate the limitation.

All examples must support removal and reconnection without accumulating listeners or performing duplicate actions. Attributes, `Cell`, and `RefCell` remain standard application choices. No extra public trait, helper, or dependency follows from the three examples.

Changing an implementation method from `&mut self` to `&self` is source-incompatible. Factory closures that require mutable captured access also need migration. Document `Cell` and scoped `RefCell` examples for both cases.

Do not change unrelated framework state APIs or constructor ownership. Do not add a dependency to replace standard interior mutability types.

## Acceptance tests

Use the existing `wasm-bindgen-test` browser harness. New behavior tests must first demonstrate the missing runtime behavior, then pass after implementation. A compiler error from changing a fixture signature is not the required failing behavior. Adapt fixtures in stages to establish the runtime regression before changing receiver contracts. Existing contract tests can pass before and after the change. Only new behavior requires a failing baseline.


Pin observable behavior with ordered traces, exact arguments, state values, rendered output, and event outcomes. Test successful paths and bounded negative cases. Avoid assertions about internal storage, generated binding text, incidental error wording, or private helper structure.

### Callback forwarding and attribute semantics

| Case | Required observation |
| --- | --- |
| Shared receiver surface | A component implements all six callbacks with shared receivers and works through an immutable `Bridge` reference. Exercise `Bridge::new` and `From` through behavior. |
| Every nested export | While one callback is active, enter each of the six exported callbacks once through `element.struct`, with bounded behavior. All six must permit the shared reentry contract. |
| Default callbacks | A component that overrides nothing accepts all six callbacks as no-ops. |
| Forwarding names | Exercise all six generated JavaScript methods and assert exactly the intended callback runs once with its original arguments. Disconnection must not call connection. |
| Browser lifecycle | Connect, disconnect, reconnect, and adopt elements through browser operations. Pin the relevant callback order and counts. |
| Move callback | Test unconditional forwarding through the generated method. Test native move behavior separately where the browser supports that operation. |
| Attribute transitions | Cover absent-to-value, replacement, equal-value assignment, and removal. Assert exact names and old/new values, including `JsValue::NULL` at the absent boundary. Distinguish absence, an empty string, and the literal string `"null"`. |
| Observed attributes | Observed names produce callbacks. Unobserved names do not. Preserve the supplied observed-attribute list. |
| Synchronous timing | In connection, change an observed attribute. Assert `connected:start`, `attribute_changed`, `connected:end` before the outer browser call returns. |
| Equal-value recursion | Let the nested attribute callback perform exactly one guarded equal-value write. Assert both deliveries and original arguments, with no silent deduplication. |
| Direct bridge access | Invoke shared methods through `element.struct` as well as the generated wrapper. The supported entry paths have the same callback behavior. |

### State and reentry

| Case | Required observation |
| --- | --- |
| Attribute-only counter | Two immediate increments update the count attribute and display twice before returning. There is no delayed update needed for correctness. |
| Counter boundaries | Exercise plus/minus controls, zero and negative steps, valid external changes, removal, and invalid numbers. Assert attributes, parsed defaults, and display. Preserve count `0` and step `1` defaults. |
| Scalar state example | Commit the cell before reflection. The nested callback observes the committed value and renders even when the incoming value equals the cell. External accepted input updates the cell. |
| Compound state example | Update both model fields before the browser call. The nested callback sees a consistent pair. Rendering uses the owned label after the state guard ends. |
| Nested state mutation | The outer callback commits `1` and releases its guard. The nested callback changes state to `2`. The resumed outer callback reads `2`, and the final value stays `2`. |
| Held mutable guard | Hold a `RefMut` deliberately. The nested callback detects contention with `try_borrow_mut`. Release the guard and demonstrate that a later borrow succeeds. |
| Held shared guard | Hold a `Ref` deliberately. A nested read succeeds, but a nested `try_borrow_mut` fails. Mutation succeeds after the guard ends. |
| Independent instances | Updating one component leaves another component's private state unchanged. Cover the scalar and compound examples. |
| Cross-element reentry | Use a bounded chain where element A calls B, then B calls A. Assert the synchronous trace and final state without a global queue. |
| Repeated use | Exercise several completed callback invocations. State accumulates correctly, and no borrow remains held after return. |
| Reconnection | For each runnable state example, remove and reconnect the element. A subsequent action runs once, retains the intended state, and leaves no duplicate listener. |

### Events

| Case | Required observation |
| --- | --- |
| Nested cancelable event | Dispatch from inside another callback. The handler sees the same event object, the exact expected `currentTarget`, and the expected path members. `preventDefault` makes dispatch return false. |
| Immediate propagation stop | A nested handler calls `stopImmediatePropagation`. A second listener does not run. |
| Ordinary propagation stop | A handler calls `stopPropagation`. Later listeners on the same target still run, but the ancestor listener does not. |
| Noncancelable event | Calling `preventDefault` does not cancel it. Dispatch returns true. |
| Uncanceled event | Without cancellation, dispatch returns true and ordinary listeners run. |
| Context lifetime | Record `currentTarget`, phase, and path inside the handler. After dispatch, verify the expected browser cleanup. Do not use retained context as proof of live delivery. |

Check the dispatch return value and listener counts, not only a retained event's eventual `defaultPrevented` flag. Use explicit bubbling and cancelability options so the tests establish the intended event contract.

### Construction and lifetime

| Case | Required observation |
| --- | --- |
| Creation paths | Exercise the default factory and configured factory with autonomous and customized built-in elements. Preserve superclass selection and argument order. |
| Customized built-in identity | Create the configured paragraph through `createElement("p", { is: registeredName })`. Assert paragraph type, local name, factory invocation, and lifecycle delivery. An `HtmlElement` assertion alone is insufficient. |
| Constructor arguments | Cover zero and several arguments, including object identity. Each factory receives its own element and argument array. |
| Recursive factory | In fresh construction, a bounded `Fn` factory creates one further same-type instance. Both invocations complete with distinct elements and component state. |
| Factory state | A captured cell supplies distinct sequence values. A scoped compound-state borrow ends before nested construction. |
| Ordinary upgrade | A preexisting element with observed attributes upgrades successfully. Initial attribute callbacks precede connection and see an initialized bridge. |
| Upgrade restriction | A separate bounded same-type recursion case preserves the browser construction failure. Compare against a plain JavaScript control and distinguish that failure from a Rust closure error. |
| Retained factory | Retain the returned closure while constructing multiple instances across browser turns and verify continued construction. |
| Released factory | In an isolated case, drop the closure and assert subsequent invocation fails visibly. Do not pin an incidental error message or require recovery of that registered class. |

Keep factory lifetime distinct from component lifetime. Removing an element does not unregister its class. The new API introduces no automatic disposal promise.

### Errors and test isolation

Capture unexpected browser errors and fail the affected case. A successful `append_child` or `set_attribute` result alone does not establish callback success. For expected browser construction errors, assert the error category, the callback trace, and the absence of the intended nested effect.

Use a controlled browser-level callback error case to check that forwarding does not retry or swallow delivery failures. Do not deliberately panic or trap a shared WebAssembly instance, and do not promise recovery after such failures. Contention tests use fallible borrows and assert the state after releasing their guards.

Every recursive fixture needs a finite bound. Give expected-error cases narrow observers instead of suppressing all browser errors. Restore temporary error handlers, remove listeners and elements, and release retained closures when safe. Registered custom elements cannot be unregistered. Keep each successful class factory callable until its isolated browser test realm ends. A deliberately released-factory test uses a separate realm or unique class that no later test touches.

Share setup helpers, not component implementations copied from examples. Put testable example logic behind a module or library target when the existing binary structure requires it. Browser tests must run the same example components that users run.

## Validation requirements

Run the existing native and browser suites, plus the example behavior suites. Compile all local example crates for the WebAssembly target. Include the new state examples in maintained test commands so they cannot silently fall outside repository checks.

Run the root browser suite and all three state-example suites in development and release builds. Require Chromium and Firefox for APIs supported by both. Keep native state-preserving moves separate from portable forwarding tests, and report platform capability limits explicitly. Do not disable portable tests because an unrelated native browser feature is absent. Safari results are additional evidence, not an unverified compatibility claim.

Run documentation tests for the trait and state examples that can execute in the native harness. Compile browser-only snippets for the WebAssembly target. Use existing testing tools where possible and add tooling only if the required behavior cannot be checked with the current harness.

Keep `wasm-bindgen` library and CLI versions aligned. The existing lockfile uses 0.2.100. Validate the final design against that version without an unrelated dependency upgrade. Temporary experiments on 0.2.129 support the design but do not replace repository tests.

Record the commands, browser and driver versions, build profiles, and unsupported optional capabilities. Any missing required execution remains an explicit validation gap. Specify narrow test-task or continuous integration changes needed to run this matrix as part of the implementation plan.

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
