# Shared Receiver Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development or superpowers:executing-plans to implement this plan task by task. Steps use checkbox syntax for tracking.

**Goal:** Support synchronous custom-element reentry through shared Rust callbacks and factories, with executable examples of attributes, `Cell`, and `RefCell` state.

**Architecture:** Keep the existing JavaScript wrapper and `Box<dyn CustomElement>` owner. Change both callback layers to `&self` and the factory to `Fn`. Applications select their state representation and release conflicting borrows before browser effects.

**Tech Stack:** Rust 2021, `wasm-bindgen` 0.2.100, `web-sys`, `js-sys`, `wasm-bindgen-test` 0.3.50, Trunk, Chromium, Firefox, and the existing cargo-make workflow.

**Spec:** `docs/superpowers/specs/2026-10-07-shared-receiver-design.md`

**Review:** `docs/superpowers/specs/2026-10-07-shared-receiver-review.md`

## Global Constraints

- Target version 0.2.0. A breaking receiver and factory change is accepted.
- Keep one `CustomElement` trait, all six method names, current arguments, unit returns, and default no-op implementations.
- Keep direct synchronous JavaScript forwarding. Add no callback queue, microtask, retry, or deduplication.
- Keep `Box<dyn CustomElement>` and caller-owned factory closure lifetime.
- Validate against the pinned `wasm-bindgen` 0.2.100 dependency. Align the CLI and preserve unrelated dependency versions.
- Provide runnable attribute, `Cell`, and scoped `RefCell` examples. Test their actual component code.
- Run the root and three state-example suites in development and release profiles on Chromium and Firefox.
- Treat unsupported native moves as a separate capability result. Never suppress portable regression tests.
- Read project instructions and the spec before execution. Never bypass hooks, disable tests, or commit code that does not compile.
- Use descriptive variable names. Add comments only for essential non-obvious constraints.
- Use test-driven development for new behavior. Compilation failures are not evidence of the original runtime borrow failure.
- No implementation starts until the user reviews this plan and selects its execution method. No publication is authorized.

## Review Focus

1. Events from inside a shadow root: pin retargeted `target`, listener-specific `currentTarget`, and the live path in Task 3.
2. Attribute changes while detached: callbacks and rendering still occur without a connection callback in Tasks 3 and 4.
3. Two registered factories: captured sequence state belongs to each factory, not a global dispatcher, in Task 3.
4. Cloned instances: cloning invokes construction for a new component, with separate state, in Task 3.
5. Out-of-range numeric input: the attribute counter preserves its current parse-failure defaults in Task 4.

## File and responsibility map

| Files | Responsibility |
| --- | --- |
| `src/custom_element.rs`, `src/bridge.rs` | Public receiver contract and exported callback boundary |
| `src/create_custom_element.rs` | Shared factory closure and bounds |
| `tests/common/contracts.rs`, `tests/common/browser.js` | Small recording and browser-observation helpers |
| `tests/forwarding.rs`, `tests/reentrancy.rs`, `tests/events.rs` | Root callback, state, and event contracts |
| `tests/factories.rs`, existing `tests/instances.rs`, `tests/arguments.rs` | Factory paths, lifetime, arguments, and upgrade boundaries |
| `examples/counter/src/counter.rs`, `src/main.rs`, `src/tests.rs` | Existing counter component, startup, and actual-component tests |
| `examples/cell_state/{Cargo.toml,index.html,README.md,src/main.rs,src/component.rs,src/tests.rs}` | Runnable scalar state example and tests |
| `examples/refcell_state/{Cargo.toml,index.html,README.md,src/main.rs,src/component.rs,src/tests.rs}` | Runnable compound state example and tests |
| Example manifests and lockfiles | Browser-test dependencies and reproducible resolution |
| `README.md`, `src/lib.rs`, example READMEs | API migration and state ownership guidance |
| `scripts/test-browser.sh`, `Makefile.toml`, `.github/workflows/tests.yml`, `SETUP.md` | Maintained execution of the required browser/profile matrix |

Use example-local modules with colocated tests. For the Cell and RefCell examples, declare the test module inside `component.rs`, not as a sibling in `main.rs`. Child modules can access private component state. User-visible behavior tests must still create the registered element and use its actual controls. Do not add an example abstraction to the library API.

Each task ends with a focused passing deliverable. Tasks 1–3 are sequential. Tasks 4–6 can proceed independently after Task 3 if file ownership is explicit. Task 7 consumes all three example suites. Task 8 closes documentation, packaging, and validation.

## Test command conventions

For a focused root browser test before the matrix script exists:

```sh
env GECKODRIVER="$(command -v geckodriver)" \
  cargo test --locked --target wasm32-unknown-unknown --test reentrancy
```

For Chromium, unset Firefox driver selection and supply the Chromium driver:

```sh
env -u GECKODRIVER -u GECKODRIVER_REMOTE \
  CHROMEDRIVER="$(command -v chromedriver)" \
  cargo test --locked --target wasm32-unknown-unknown --test reentrancy
```

The repository already sets `wasm-bindgen-test-runner` for this target. Check the installed CLI version before running. Run each test command once for the baseline and once after its relevant change. Broaden testing when the changed contract requires it.

## Task 1 Establish recording helpers and trustworthy baseline tests

**Files:** Create `tests/common/contracts.rs`, `tests/common/browser.js`, and `tests/forwarding.rs`. Modify `tests/common/mod.rs`, `tests/instances.rs`, `Cargo.toml`, and the root lockfile only as required for test features.

**Interfaces:**

- Consume `create_custom_element`, `create_custom_element_with_config`, and the current mutable trait.
- Produce `Call`, `Hook`, `register_recorder(Hook) -> (String, GeneratedConstructor)`, `create_recorder(Hook) -> HtmlElement`, and `invoke(&JsValue, &str, &Array) -> Result<JsValue, JsValue>` in test support.
- `Hook` is `Rc<dyn Fn(&HtmlElement, Call)>`. Factories clone the hook for each element. No hook borrow guard survives a callback.

- [ ] Add the recording types and implement every existing callback by forwarding its original data to the hook. Keep `&mut self` in this baseline fixture so it compiles before the API change.

```rust
pub enum Call {
    Connected,
    Disconnected,
    Adopted,
    Moved,
    Attribute { name: String, old: JsValue, new: JsValue },
    Event(web_sys::Event),
}

pub type Hook = Rc<dyn Fn(&HtmlElement, Call)>;

struct Recorder {
    element: HtmlElement,
    hook: Hook,
}

impl CustomElement for Recorder {
    fn connected_callback(&mut self) {
        (self.hook)(&self.element, Call::Connected);
    }
    fn disconnected_callback(&mut self) {
        (self.hook)(&self.element, Call::Disconnected);
    }
    fn adopted_callback(&mut self) {
        (self.hook)(&self.element, Call::Adopted);
    }
    fn connected_move_callback(&mut self) {
        (self.hook)(&self.element, Call::Moved);
    }
    fn handle_event(&mut self, event: web_sys::Event) {
        (self.hook)(&self.element, Call::Event(event));
    }
    fn attribute_changed_callback(&mut self, name: &str, old: JsValue, new: JsValue) {
        (self.hook)(&self.element, Call::Attribute {
            name: name.to_owned(), old, new,
        });
    }
}

pub fn register_recorder(hook: Hook) -> (String, GeneratedConstructor) {
    let name = super::helpers::get_random_custom_element_name();
    let (factory, constructor) = create_custom_element(
        move |element, _args| Recorder {
            element: element.into(),
            hook: hook.clone(),
        },
        vec!["observed".to_owned()],
    );
    factory.forget();
    web_sys::window().unwrap().custom_elements()
        .define(&name, &constructor).unwrap();
    (name, constructor)
}

pub fn create_recorder(hook: Hook) -> HtmlElement {
    let (name, _constructor) = register_recorder(hook);
    web_sys::window().unwrap().document().unwrap()
        .create_element(&name).unwrap().dyn_into().unwrap()
}

pub fn invoke(target: &JsValue, method: &str, args: &Array) -> Result<JsValue, JsValue> {
    let function = js_sys::Reflect::get(target, &JsValue::from_str(method))?
        .dyn_into::<js_sys::Function>()?;
    function.apply(target, args)
}
```

Import the existing crate types, `Rc`, `JsCast`, `JsValue`, `HtmlElement`, and `Array` in this test module. Export it from `tests/common/mod.rs`. Do not introduce a generic callback framework beyond these helpers.

- [ ] Add a narrow JavaScript error observer for expected-error cases. Bind it from Rust with `#[wasm_bindgen(module = "/tests/common/browser.js")]`. It accepts a synchronous `Function` and returns an `Array`.

```javascript
export function captureErrors(action) {
  const errors = [];
  const observe = event => {
    errors.push(event.error ?? new Error(event.message));
    event.preventDefault();
  };
  window.addEventListener('error', observe);
  try {
    action();
    return errors;
  } finally {
    window.removeEventListener('error', observe);
  }
}

export function nextTurn() {
  return new Promise(resolve => setTimeout(resolve, 0));
}
```

Bind `nextTurn` as returning `js_sys::Promise`. Add direct dev access to the already-locked `wasm-bindgen-futures` 0.4.50 for async tests. Add only needed `web-sys` test features: `HtmlParagraphElement`, `ElementCreationOptions`, `EventInit`, `EventTarget`, `ShadowRoot`, `ShadowRootInit`, and `ShadowRootMode`. Preserve the existing resolved package family.

- [ ] Strengthen the existing customized built-in document-creation test. Use `ElementCreationOptions` with `is` set to the registered name and create local name `p`. Assert `HtmlParagraphElement`, `local_name() == "p"`, one factory call, and connection delivery. Observe the current wrong-path fixture fail these assertions, then correct its element-creation call.

The equivalent JavaScript operation is:

```javascript
const element = document.createElement('p', { is: registeredName });
```

- [ ] Add table-driven forwarding tests using the method names and inputs below. Assert one matching `Call` and no other calls. Create a fresh element for each row.

| JavaScript method | Arguments | Expected call |
| --- | --- | --- |
| `connectedCallback` | `[]` | `Connected` |
| `disconnectedCallback` | `[]` | `Disconnected` |
| `adoptedCallback` | `[]` | `Adopted` |
| `connectedMoveCallback` | `[]` | `Moved` |
| `attributeChangedCallback` | `["observed", null, "value"]` | Exact attribute tuple |
| `handleEvent` | `[event]` | The same event object |

Use `invoke(element.as_ref(), method, &arguments)` and assert successful completion. Repeat the six invocations against a minimal `struct Empty; impl CustomElement for Empty {}` to establish default no-op behavior.

- [ ] Run `cargo test --locked` and browser tests `constructors`, `instances`, `arguments`, and `forwarding`. These characterization tests pass without changing production receiver semantics.
- [ ] Commit the focused baseline tests with `test: establish custom element callback contracts`.

## Task 2 Change both receiver boundaries and the factory atomically

**Files:** Create `tests/reentrancy.rs` and the first `tests/factories.rs` cases. Modify `src/{custom_element.rs,bridge.rs,create_custom_element.rs}`, affected test fixtures, and affected example implementations. Extract the existing counter into `examples/counter/src/counter.rs` and add `examples/counter/src/tests.rs`.

**Interfaces:**

- Consume Task 1's recorder helpers and existing generated constructors.
- Produce all six `CustomElement` and `Bridge` methods with `&self`.
- Produce `Factory: Fn(JsValue, Array) -> Component + 'static` for both factory functions and `Closure<dyn Fn(JsValue, Array) -> JsValue>` throughout their exposed results.
- Preserve every JavaScript name, return type, and ownership convention.

- [ ] Write and run the real nested-attribute regression while fixtures still use `&mut self`.

```rust
#[wasm_bindgen_test]
fn connection_delivers_nested_attribute_before_resuming() {
    let trace = Rc::new(RefCell::new(Vec::new()));
    let recorded = trace.clone();
    let element = create_recorder(Rc::new(move |element, call| match call {
        Call::Connected => {
            recorded.borrow_mut().push("connected:start");
            element.set_attribute("observed", "value").unwrap();
            recorded.borrow_mut().push("connected:end");
        }
        Call::Attribute { .. } => recorded.borrow_mut().push("attribute"),
        _ => {}
    }));
    web_sys::window().unwrap().document().unwrap().body().unwrap()
        .append_child(&element).unwrap();
    assert_eq!(*trace.borrow(), vec!["connected:start", "attribute", "connected:end"]);
    element.remove();
}
```

Expected baseline: the inner callback does not run and the browser reports the recursive object error. A compile failure is not the baseline. Capture the error narrowly when necessary to keep a failed baseline run intelligible.

- [ ] Add `every_export_accepts_nested_shared_entry`. For each method in Task 1's six-row table, let the outer connection hook set a `Cell<bool>` guard and invoke that method on `element.struct`. Log entry and return. For nested connection, the guard prevents a second recursion. Assert the target callback runs before the invocation returns. Observe the existing bridge reject those calls.
- [ ] Add a bounded recursive factory test. Capture `Rc<Cell<u32>>` and `Rc<RefCell<Vec<JsValue>>>`. Increment the factory count, release any vector borrow, and on the first entry call `document.create_element` for the registered name. Assert two entries and distinct element identities. Run against `FnMut` and record the runtime closure rejection.
- [ ] Extract the counter's component without changing behavior, then add an immediate two-click test against it. Its current scheduled writes must fail the immediate attribute/display assertion. Keep this uncommitted until the whole task passes.
- [ ] Change the production boundary types exactly as follows, and update every trait implementation in tests and examples to match:

```rust
pub fn connected_callback(&self) {
    self.inner.connected_callback()
}

type MakeStruct = Closure<dyn Fn(JsValue, Array) -> JsValue>;
```

Apply the same receiver change to disconnection, adoption, move, attribute, and event methods. In both factory functions, replace `FnMut` with `Fn`. Remove the mutable factory binding. Keep `inner: Box<dyn CustomElement>` and `CustomElementResult.closure` consistent with the new alias.

- [ ] Make the counter compile under shared receivers by deleting its cached `count`, `step`, and scheduled-closure fields. Its three click helpers synchronously set attributes from the existing parsed getters. Render from those getters. Remove the imported microtask function and only the closure machinery that served it.

```rust
fn handle_increment_button_click(&self) {
    let count = self.get_count_attribute() + self.get_step_attribute();
    self.instance.set_attribute(Self::COUNT_ATTR_NAME, &count.to_string()).unwrap();
}

fn attribute_changed_callback(&self, _name: &str, _old: JsValue, _new: JsValue) {
    self.render();
}
```

For plus and minus, compute `get_step_attribute() + PLUS_AMOUNT` or `+ MINUS_AMOUNT` and synchronously set `STEP_ATTR_NAME`. Preserve signed arithmetic and existing defaults. `render` reads both getters and keeps the current display text.

Keep `Counter::new(JsValue, Array) -> Counter` and `Counter::get_observed_attributes() -> Vec<String>` accessible to the example startup and colocated tests. Make no root-library export for the example. Put `#[cfg(test)] mod tests;` beside the component module in `main.rs`.

- [ ] Migrate the Leptos and existing test trait receivers. Do not change unrelated Leptos signal APIs or lifetime handling. The hello-world implementation with default methods needs no receiver edits.
- [ ] Run the new root regressions, existing suites, and counter tests. Check every existing example manifest for `wasm32-unknown-unknown`. Assert nested timing and distinct factory invocation counts before committing.
- [ ] Commit the compiling transition with `feat!: support shared custom element callbacks`.

## Task 3 Pin the full root browser contract

**Files:** Expand `tests/{forwarding.rs,reentrancy.rs,factories.rs}`. Create `tests/events.rs`. Modify `tests/common/browser.js` only for browser operations that Rust bindings cannot express clearly.

**Interfaces:** Consume Task 2's shared callbacks, Task 1's `Call`/`Hook`/factory helpers, and browser objects. No production API changes.

- [ ] Add exact attribute transitions in `forwarding.rs`. Perform the following operations on a detached registered instance and assert the exact emitted tuples immediately:

```rust
let operations = [
    ("", JsValue::NULL, JsValue::from_str("")),
    ("null", JsValue::from_str(""), JsValue::from_str("null")),
    ("null", JsValue::from_str("null"), JsValue::from_str("null")),
];
for (value, expected_old, expected_new) in operations {
    element.set_attribute("observed", value).unwrap();
    assert_eq!(attributes.borrow().last().unwrap(), &(
        "observed".to_owned(), expected_old, expected_new,
    ));
}
element.remove_attribute("observed").unwrap();
assert_eq!(attributes.borrow().last().unwrap(), &(
    "observed".to_owned(), JsValue::from_str("null"), JsValue::NULL,
));
```

Here, `attributes` is `Rc<RefCell<Vec<(String, JsValue, JsValue)>>>` captured by the recording hook for `Call::Attribute`. Also assert the count is four and an unobserved write adds nothing. Repeat the observed write after disconnection.

- [ ] Add a bounded equal-value recursion test. Its attribute hook records the tuple, then uses `Cell<bool>::replace(true)` to permit exactly one nested write of the same value. Assert the outer tuple is `(null, "value")`, the inner tuple is `("value", "value")`, and the trace is nested rather than queued.
- [ ] Add state probes using a captured `Rc<RefCell<u32>>`. Connection sets `1`, drops its guard, sets the observed attribute, then reads state. The attribute callback sets `2`. Assert the resumed read and final state both equal `2`.
- [ ] Add held-guard controls. For `RefMut`, nested `try_borrow` and `try_borrow_mut` both fail. For `Ref`, nested `try_borrow` succeeds and `try_borrow_mut` fails. After each outer guard ends, assert a later mutable borrow succeeds. Never panic the shared test module deliberately.
- [ ] Add two-instance independence and a bounded first-element → second-element → first-element chain. Capture the peer handle in `Rc<RefCell<Option<HtmlElement>>>`, clone the handle in a short scope, then call it. Assert exact entry/return order and separate final state.
- [ ] Add browser event helpers with deterministic dispatch observations:

```javascript
export function dispatchProbe(element, cancelable) {
  const event = new Event('probe', { bubbles: true, cancelable });
  const accepted = element.dispatchEvent(event);
  return { event, accepted, currentTarget: event.currentTarget,
    phase: event.eventPhase, path: event.composedPath() };
}
```

Register the generated element itself as the listener using the existing `handleEvent` object pattern. Add a second listener and an ancestor listener with separately owned counters. In the Rust `Call::Event` hook, record object identity, `current_target`, `event_phase`, and `composed_path` before applying the selected action.

- [ ] Implement `nested_event_cancellation_preserves_dispatch_context` and the controls below. Trigger `dispatchProbe` from the outer connection hook, not from an idle test body.

| Handler action | Dispatch accepted | Later same-target listener | Ancestor listener |
| --- | --- | --- | --- |
| Cancel + immediate stop | false | 0 | 0 |
| Ordinary stop, no cancel | true | 1 | 0 |
| No stop or cancel | true | 1 | 1 |
| Cancel noncancelable event | true | 1 | 1 |

Assert exact handler `currentTarget` and expected path membership. After dispatch, assert null `currentTarget`, phase zero, and an empty path. Add a composed event from a button inside an open shadow root to pin retargeting at the host.

- [ ] Add connection/disconnection/reconnection/adoption traces. Adopt a connected element into a second HTML document and compare the relevant callback order with a plain JavaScript control if platform sequencing is uncertain. Assert the disconnection callback never increments the connection counter.
- [ ] Add native move coverage with an explicit capability result. Always run the direct `connectedMoveCallback` forwarding test. When `Element.prototype.moveBefore` exists, perform a state-preserving move between connected parents in one document and assert a move callback instead of disconnect/reconnect. Report absence separately, not as a passing native-move test.
- [ ] Expand factory tests with concrete cases:

| Test | Setup and assertion |
| --- | --- |
| `arguments_preserve_order_and_identity` | Construct with `[17, null, object]`. Assert length, order, object identity, and that the factory element is the returned instance. Also construct with zero arguments. |
| `configured_paragraph_uses_correct_creation_path` | Register with `HTML_PARAGRAPH_ELEMENT_CONSTRUCTOR` and `extends: "p"`. Use the `is` option and assert paragraph identity, local name, and callback delivery. |
| `upgrade_initializes_bridge_before_callbacks` | Append an unregistered tag with an observed attribute, then define it. Assert factory, initial attribute callback, connection order and no missing-struct error. |
| `upgrade_recursion_preserves_browser_restriction` | Run the bounded recursive factory during upgrade and compare error category/body count with a plain JavaScript class. Do not match browser-specific text. |
| `factory_remains_callable_across_turns` | Retain its closure, construct once, await `JsFuture::from(nextTurn())`, construct again, and assert two instances. |
| `dropping_factory_fails_visibly` | On a unique class, drop its closure and invoke its constructor through `Reflect::construct`. Assert `Err` and zero further factory effects. Never reuse this class. |
| `factory_state_is_local_to_registration` | Two registrations each capture their own sequence cell. Two constructions per class must yield `1,2` independently. |
| `cloned_element_has_separate_component` | Clone a registered element. Assert a second factory entry, new element identity, and independent private state. |

Cover both factory functions, default configuration, explicit autonomous configuration, and customized built-ins. Use scoped captured `RefCell` state in an additional recursive-factory case.

- [ ] Test immutable `Bridge` values built through `Bridge::new` and `From`: invoke their shared callbacks and assert effects. Preserve a no-op component case for all methods.
- [ ] Inject a controlled JavaScript error into one bridge method, invoke it through the generated wrapper, and assert one reported failure and one attempted delivery. Restore the method in `finally`. Do not use a Rust panic to test browser error reporting.
- [ ] Run all root browser tests on both required browsers. Run native tests. Commit with `test: pin synchronous custom element behavior`.

## Task 4 Finish the attribute-only counter example tests

**Files:** `examples/counter/src/{counter.rs,main.rs,tests.rs}`, `examples/counter/Cargo.toml`, its lockfile, and `README.md`.

**Interfaces:** Consume the actual `Counter::new` and observed-attribute list extracted in Task 2. The browser controls and attributes remain the user-facing interface.

- [ ] Add `wasm-bindgen-test = "=0.3.50"` as an example dev dependency. Keep `wasm-bindgen` resolved to 0.2.100. Add only browser feature flags used by these tests.
- [ ] Give the existing increment, plus, minus, and display nodes stable `data-role` values for semantic test selection. Tests create the real registered counter and query its open shadow root. Do not create a second counter implementation.

```rust
let shadow = element.shadow_root().unwrap();
let increment: web_sys::HtmlElement = shadow
    .query_selector("[data-role='increment']").unwrap().unwrap()
    .dyn_into().unwrap();
increment.click();
assert_eq!(element.get_attribute("count").as_deref(), Some("1"));
increment.click();
assert_eq!(element.get_attribute("count").as_deref(), Some("2"));
assert_eq!(shadow.query_selector("[data-role='display']").unwrap().unwrap()
    .text_content().as_deref(), Some("Count is 2"));
```

- [ ] Add deterministic sequences with assertions after every action:

| Start/input | Action | Expected result |
| --- | --- | --- |
| No attributes | Connect | Display count 0 and step 1 |
| Count 4, step 2 | Increment twice | Count/display 8 before returning |
| Step 1 | Plus, minus, minus | Step 0 |
| Count 4, step 0 | Increment | Count remains 4 |
| Count 4, step -2 | Increment | Count/display 2 |
| Count changed externally to 9 | No click | Display immediately becomes 9 |
| Remove count | No click | Display becomes 0 |
| Remove step | Increment from count 0 | Count becomes 1 |
| Count `abc` or `2147483648` | Connect/render | Parsed count/display 0 |
| Step `abc` or `2147483648` | Increment from count 0 | Count becomes 1 |

Default parsing must not rewrite missing or invalid raw attributes during rendering. Assert raw attributes separately from their interpreted display values.

- [ ] Add two-instance isolation and connection → removal → reconnection coverage. A subsequent click changes the value once. Add a detached external attribute write and assert rendering updates without a new connection.
- [ ] Keep the example startup limited to registration, initial sample attributes, and attachment. Document attribute authority and immediate callback behavior.
- [ ] Run the actual counter browser suite in development and release profiles. Run `cargo check --manifest-path examples/counter/Cargo.toml --target wasm32-unknown-unknown`. Commit with `test: pin attribute counter behavior`.

## Task 5 Add the runnable Cell example

**Files:** Create `examples/cell_state/{Cargo.toml,index.html,README.md,src/main.rs,src/component.rs,src/tests.rs}` and its lockfile.

**Interfaces:** `component::CellCounter::new(JsValue, Array) -> CellCounter` and `CellCounter::observed_attributes() -> Vec<String>`. Keep this API local to the example. Tests run the same component.

- [ ] Use the existing counter manifest and Trunk layout as the template. Set the package name to `cell_state`. Keep the local `custom-element = { path = "../../" }` dependency. For new example manifests, use exact versions `wasm-bindgen = "=0.2.100"`, `web-sys = "=0.3.77"`, and `js-sys = "=0.3.77"`. Use `wasm-bindgen-test = "=0.3.50"` as a dev dependency. Do not add a runtime state library.
- [ ] Write tests for a registered component with one increment button, a display, and an observed `count` attribute. Establish the missing behavior using a compiling no-op shared implementation before adding event behavior. Assert the initial display is 0, one click gives 1 immediately, and two clicks give 2.
- [ ] Implement the scalar state boundary:

```rust
struct CellCounter {
    element: HtmlElement,
    button: HtmlElement,
    output: HtmlElement,
    count: Cell<u32>,
}

impl CellCounter {
    fn increment(&self) {
        let next = self.count.get().saturating_add(1);
        self.count.set(next);
        self.element.set_attribute("count", &next.to_string()).unwrap();
    }

    fn render(&self) {
        self.output.set_text_content(Some(&format!("Count: {}", self.count.get())));
    }
}
```

The constructor creates an open shadow root and appends a native button and output span to it. It does not add host attributes or light-DOM children. Use `data-role='increment'` and `data-role='display'`. Return `Cell::new(0)` state.

- [ ] Implement the callback contract without an equality-based early return:

```rust
fn attribute_changed_callback(&self, _name: &str, _old: JsValue, new: JsValue) {
    let count = new.as_string().and_then(|value| value.parse::<u32>().ok()).unwrap_or(0);
    self.count.set(count);
    self.render();
}
```

`connected_callback` renders and registers the generated element as the button's click listener. `disconnected_callback` removes that same listener. `handle_event` accepts a click only when `current_target` matches the button, then calls `increment`. Reuse the existing example's listener-object binding pattern.

- [ ] Declare the tests inside `src/component.rs` so they can inspect private state:

```rust
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
```

Start `src/tests.rs` with `use super::*;` and the test-specific imports. Define a local observation wrapper. The wrapper forwards to the real component and only records the state at nested entry:

```rust
struct ObservedCellCounter {
    inner: Rc<CellCounter>,
    seen: Rc<RefCell<Vec<u32>>>,
}
impl CustomElement for ObservedCellCounter {
    fn connected_callback(&self) { self.inner.connected_callback(); }
    fn disconnected_callback(&self) { self.inner.disconnected_callback(); }
    fn handle_event(&self, event: Event) { self.inner.handle_event(event); }
    fn attribute_changed_callback(&self, name: &str, old: JsValue, new: JsValue) {
        self.seen.borrow_mut().push(self.inner.count.get());
        self.inner.attribute_changed_callback(name, old, new);
    }
}
```

Register this test wrapper through a factory that creates `Rc::new(CellCounter::new(element, args))`. Retain a cloned `Rc` in a test-owned vector, with no vector borrow across a browser call. Retain the `seen` handle separately. After one click, assert the observation is `[1]`, the real component's count is 1, and the real display reads `Count: 1`. Run ordinary UI cases directly against `CellCounter` as well.

Use a unique literal tag per test, such as `cell-precommit-test`, and a generated suffix for table cases. Keep factory closures alive for the browser realm. No copied mutation implementation or root-library adapter is allowed.

- [ ] Test external count assignment, removal/invalid input defaults, `u32::MAX` saturation, two-instance isolation, reconnection, and reflected equal-value rendering. A direct `count` write updates state and display immediately.
- [ ] Startup registers a distinct sample tag, retains the factory through `forget`, creates the element, and appends it. Add setup and `trunk serve --open` instructions. Explain that cells own the scalar state and reflected input is handled by the attribute callback.
- [ ] Run focused browser tests and compile the example. Commit with `feat: add shared callback Cell example`.

## Task 6 Add the runnable scoped RefCell example

**Files:** Create `examples/refcell_state/{Cargo.toml,index.html,README.md,src/main.rs,src/component.rs,src/tests.rs}` and its lockfile.

**Interfaces:** `component::CompoundCounter::new(JsValue, Array) -> CompoundCounter` and `CompoundCounter::observed_attributes() -> Vec<String>`. Keep both local to the example. Model fields are `count: u32` and `updates: u32`.

- [ ] Set the package name to `refcell_state`. Use `custom-element = { path = "../../" }`, `wasm-bindgen = "=0.2.100"`, `web-sys = "=0.3.77"`, and `js-sys = "=0.3.77"`. Add dev dependency `wasm-bindgen-test = "=0.3.50"`. Create the Trunk entry page. Write a browser test expecting one click to render `Count: 1; updates: 1` before implementing the increment behavior.
- [ ] Implement the compound mutation scope and owned output:

```rust
struct Model {
    count: u32,
    updates: u32,
}

impl CompoundCounter {
    fn increment(&self) {
        let (count, label) = {
            let mut model = self.model.borrow_mut();
            model.count = model.count.saturating_add(1);
            model.updates = model.updates.saturating_add(1);
            (model.count, format!("Count: {}; updates: {}", model.count, model.updates))
        };
        self.output.set_text_content(Some(&label));
        self.element.set_attribute("count", &count.to_string()).unwrap();
    }
}
```

`CompoundCounter` owns `element`, `button`, and `output` as `HtmlElement` handles and `model: RefCell<Model>`. Its constructor creates the same small shadow-root controls as the scalar example, with state `(0, 0)` and no host mutation during construction.

- [ ] Implement the observed-attribute callback with a new short borrow. Parse `u32` with default 0. If the parsed count differs, update `count` and increment `updates` once with saturation. Always compute a fresh owned label, release the guard, and render it. Do not reflect the attribute again.

```rust
fn attribute_changed_callback(&self, _name: &str, _old: JsValue, new: JsValue) {
    let count = new.as_string().and_then(|value| value.parse::<u32>().ok()).unwrap_or(0);
    let label = {
        let mut model = self.model.borrow_mut();
        if model.count != count {
            model.count = count;
            model.updates = model.updates.saturating_add(1);
        }
        format!("Count: {}; updates: {}", model.count, model.updates)
    };
    self.output.set_text_content(Some(&label));
}
```

The output update before reflection demonstrates owned output after the guard ends. The observed attribute then provides real same-element reentry. The outer method performs no stale render after that nested callback returns.

- [ ] Implement connection/listener registration, disconnection/removal, event-target checking, and startup as in the specified scalar contract. These are separate small example implementations, not a new generic example framework.
- [ ] Add browser tests with these exact observations:

| Input | Expected model and output |
| --- | --- |
| Connect | `(0,0)`, `Count: 0; updates: 0` |
| One click | `(1,1)`, matching output before click returns |
| Two clicks | `(2,2)` |
| External count 7 after two clicks | `(7,3)` |
| Equal external count 7 | `(7,3)`, callback still renders |
| Remove count | `(0,4)` |
| Invalid count when already 0 | `(0,4)` |
| Click at max count | Count saturates, action update count advances until its own saturation |

Declare the test module inside `src/component.rs`:

```rust
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
```

Start `src/tests.rs` with `use super::*;` and test-specific imports. Use a local forwarding wrapper to inspect the actual model before the actual callback:

```rust
struct ObservedCompoundCounter {
    inner: Rc<CompoundCounter>,
    seen: Rc<RefCell<Vec<(u32, u32)>>>,
}
impl CustomElement for ObservedCompoundCounter {
    fn connected_callback(&self) { self.inner.connected_callback(); }
    fn disconnected_callback(&self) { self.inner.disconnected_callback(); }
    fn handle_event(&self, event: Event) { self.inner.handle_event(event); }
    fn attribute_changed_callback(&self, name: &str, old: JsValue, new: JsValue) {
        let snapshot = {
            let model = self.inner.model.try_borrow().expect("model must be released before reflection");
            (model.count, model.updates)
        };
        self.seen.borrow_mut().push(snapshot);
        self.inner.attribute_changed_callback(name, old, new);
    }
}
```

The factory creates the real `CompoundCounter` in an `Rc` and retains a clone for test assertions. Keep observation and component handles in test-local ownership, and give each test a unique registered name. After one click, assert `seen == [(1, 1)]`, model `(1,1)`, and matching output. Test ordinary UI behavior directly against the component too. Do not reproduce its mutation logic in a fixture.

- [ ] Add reconnection and independent-instance tests. Root contention tests from Task 3 cover deliberately retained guards. Document the failing pattern without making the runnable example fail.
- [ ] Run browser tests and compile the example. Commit with `feat: add scoped RefCell state example`.

## Task 7 Maintain the browser and profile matrix

**Files:** Create `scripts/test-browser.sh`. Modify `Makefile.toml`, `.github/workflows/tests.yml`, `.gitignore`, and `SETUP.md`. Do not alter callback code.

**Interfaces:** The script accepts `firefox|chromium` and `debug|release`. It runs the root and all three state-example suites against the same pinned binding family. Missing required tooling exits nonzero.

- [ ] Add the matrix runner with explicit manifest ownership:

```bash
#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$repo_root"
unset NO_HEADLESS SAFARIDRIVER SAFARIDRIVER_REMOTE MSEDGEDRIVER MSEDGEDRIVER_REMOTE
unset CHROMEDRIVER_REMOTE GECKODRIVER_REMOTE
[[ "$(wasm-bindgen --version)" == "wasm-bindgen 0.2.100"* ]]
[[ "$(wasm-bindgen-test-runner --version)" == "wasm-bindgen-test-runner 0.2.100"* ]]
browser="${1:?Specify firefox or chromium}"
profile="${2:?Specify debug or release}"
case "$browser" in
  firefox)
    unset CHROMEDRIVER CHROMEDRIVER_REMOTE
    driver_path="$(command -v geckodriver)"
    export GECKODRIVER="$driver_path"
    ;;
  chromium)
    unset GECKODRIVER GECKODRIVER_REMOTE
    driver_path="$(command -v chromedriver)"
    export CHROMEDRIVER="$driver_path"
    ;;
  *) exit 2 ;;
esac
profile_args=()
case "$profile" in
  debug) ;;
  release) profile_args+=(--release) ;;
  *) exit 2 ;;
esac
manifests=(
  Cargo.toml
  examples/counter/Cargo.toml
  examples/cell_state/Cargo.toml
  examples/refcell_state/Cargo.toml
)
for manifest in "${manifests[@]}"; do
  cargo test --locked --manifest-path "$repo_root/$manifest" \
    --target wasm32-unknown-unknown "${profile_args[@]}"
done
```

Run the script from the repository root so `.cargo/config.toml` selects the runner consistently. Fail immediately if a driver lookup returns an empty path. Confirm `wasm-bindgen --version` and `wasm-bindgen-test-runner --version` match the locked family before running suites. Print browser and driver versions in the invoking task.

- [ ] Add `test-browser-matrix` to cargo-make to run the four pairs. Keep `test` as the documented aggregate command, including native tests and the matrix. Avoid an inherited `GECKODRIVER` forcing Chromium jobs onto Firefox.

```toml
[tasks.test-browser-matrix]
script = [
  "bash scripts/test-browser.sh firefox debug",
  "bash scripts/test-browser.sh firefox release",
  "bash scripts/test-browser.sh chromium debug",
  "bash scripts/test-browser.sh chromium release",
]
```

Update the existing aggregate dependency/run-task list to include `test-native` and `test-browser-matrix` exactly once. Retain a fast focused browser task for development. Record the changed local setup requirements.

- [ ] Split the existing workflow into native checks and the browser matrix. Keep existing checkout/toolchain conventions. Install the browser binaries explicitly and direct WebDriver to those binaries. Add this browser job, retaining native tests in a separate job:

```yaml
browser-tests:
  runs-on: ubuntu-latest
  permissions:
    contents: read
  strategy:
    fail-fast: false
    matrix:
      browser: [firefox, chromium]
      profile: [debug, release]
  steps:
    - uses: actions/checkout@v2
    - run: |
        rustup target add wasm32-unknown-unknown
        cargo install wasm-bindgen-cli --version 0.2.100 --locked
    - id: chrome
      if: matrix.browser == 'chromium'
      uses: browser-actions/setup-chrome@48483d551c22a1fe6154c6a195ccc6d5773dc8c4
      with:
        chrome-version: stable
        install-chromedriver: true
    - id: firefox
      if: matrix.browser == 'firefox'
      uses: browser-actions/setup-firefox@2ddc1281882c66dd6a359947714eed0a4b91afc1
      with:
        firefox-version: latest
    - if: matrix.browser == 'firefox'
      uses: browser-actions/setup-geckodriver@fac5b0424c584257f32cf12c5eb4ba86014c34f8
    - name: Configure browser binary
      env:
        TEST_BROWSER: ${{ matrix.browser }}
        BROWSER_BINARY: ${{ steps.chrome.outputs.chrome-path || steps.firefox.outputs.firefox-path }}
      run: |
        "$BROWSER_BINARY" --version
        python3 - <<'PYTHON'
        import json
        import os
        from pathlib import Path
        key = "goog:chromeOptions" if os.environ["TEST_BROWSER"] == "chromium" else "moz:firefoxOptions"
        capabilities = {key: {"binary": os.environ["BROWSER_BINARY"]}}
        for directory in [".", "examples/counter", "examples/cell_state", "examples/refcell_state"]:
            (Path(directory) / "webdriver.json").write_text(json.dumps(capabilities))
        PYTHON
    - name: Run browser tests
      run: |
        if [ "${{ matrix.browser }}" = firefox ]; then geckodriver --version; else chromedriver --version; fi
        bash scripts/test-browser.sh "${{ matrix.browser }}" "${{ matrix.profile }}"
```

The action commits resolve to the inspected Chrome v2, Firefox v1, and Geckodriver distribution revisions. Keep their role limited to installing browsers and drivers. The installed browser versions remain visible in job output.

Version 0.2.100 reads `webdriver.json` from the test process working directory. Generate it for each package in the disposable job checkout. Add `**/webdriver.json` to `.gitignore`. Do not overwrite an existing developer configuration when running the local matrix script.

Implementation references: [Chrome setup](https://github.com/browser-actions/setup-chrome), [Firefox setup](https://github.com/browser-actions/setup-firefox), [Geckodriver setup](https://github.com/browser-actions/setup-geckodriver), and [the pinned test runner](https://github.com/wasm-bindgen/wasm-bindgen/blob/0.2.100/crates/cli/src/bin/wasm-bindgen-test-runner/headless.rs).
- [ ] Run `bash -n scripts/test-browser.sh`. Execute all four combinations. A missing browser is a reported validation gap, never a passing matrix job.
- [ ] Commit with `ci: test shared callbacks across browser profiles`.

## Task 8 Complete migration documentation and final verification

**Files:** `README.md`, `src/lib.rs`, public trait/factory documentation, all affected example READMEs, root `Cargo.toml`, root and example lockfiles, and `SETUP.md` if the matrix setup needs clarification.

**Interfaces:** Publish no new state abstractions. Documentation describes the shared trait, shared factory, three state strategies, and their limits.

- [ ] Replace the old warning with the precise contract: shared callbacks allow reentry only when application state has no conflicting borrow. Explain both shared and mutable `RefCell` guards, state changes during nested calls, and the independent factory restriction during browser upgrades.
- [ ] Add migration snippets for scalar `Cell`, scoped compound `RefCell`, and `Fn` factory captures. Link all three executable examples. Avoid claiming `Rc` itself fixes reentry or that scope-local `drop(self)` releases an exported borrow.
- [ ] Add a native documentation test proving the trait is usable with a shared bridge and standard interior mutability:

```rust
use std::{cell::Cell, rc::Rc};
use custom_element::{Bridge, CustomElement};

struct Visits(Rc<Cell<u32>>);
impl CustomElement for Visits {
    fn connected_callback(&self) {
        self.0.set(self.0.get() + 1);
    }
}
let count = Rc::new(Cell::new(0));
let bridge = Bridge::new(Visits(count.clone()));
bridge.connected_callback();
assert_eq!(count.get(), 1);
```

Compile browser-only examples for the WebAssembly target rather than executing browser globals in native documentation tests. A `no_run` browser snippet must still typecheck where possible.

- [ ] Change the package version to 0.2.0. Update local package entries in lockfiles without refreshing unrelated dependencies. Preserve `wasm-bindgen` 0.2.100, `web-sys` 0.3.77, `js-sys` 0.3.77, `wasm-bindgen-test` 0.3.50, and `wasm-bindgen-futures` 0.4.50 in the validated resolution. Existing manifest minimums can remain compatible.
- [ ] Run the complete verification set and record actual outcomes:

```sh
cargo fmt --all -- --check
cargo test --locked
cargo test --locked --doc
cargo make test
cargo check --locked --target wasm32-unknown-unknown --manifest-path examples/hello_world/Cargo.toml
cargo check --locked --target wasm32-unknown-unknown --manifest-path examples/counter/Cargo.toml
cargo check --locked --target wasm32-unknown-unknown --manifest-path examples/cell_state/Cargo.toml
cargo check --locked --target wasm32-unknown-unknown --manifest-path examples/refcell_state/Cargo.toml
cargo check --locked --target wasm32-unknown-unknown --manifest-path examples/leptos_in_react/crates/leptos-app/Cargo.toml
```

Run formatting checks in each non-workspace example manifest as well. `cargo fmt --all` at the root does not cover these separate crates. Do not describe the root suite as testing examples unless the maintained matrix actually ran them.

- [ ] Check the spec-to-task mapping below and review the final diff. Fix any missing required test or report a specific external execution blocker. Preserve all baseline behavior tests that still apply.
- [ ] Commit documentation and version changes with `docs: document shared receiver migration`. Follow the selected execution workflow's final independent review. Do not publish the crate or push changes without an explicit request.

## Spec coverage and execution notes

| Spec area | Owning tasks |
| --- | --- |
| All six receiver contracts, defaults, immutable Bridge paths | 1, 2, 3 |
| Fn factory, captured state, inputs, configuration, lifetime, upgrades | 2, 3 |
| Synchronous nesting, bounded cycles, stale-state avoidance, borrow conflicts | 2, 3 |
| Attribute identity, null/empty/value distinctions, equal-value recursion | 3 |
| Cancelability, propagation, event identity/context, shadow retargeting | 3 |
| Attribute-only example and parse/control boundaries | 2, 4 |
| Cell example and precommit rendering | 5 |
| Scoped RefCell example and actual nested borrow | 6 |
| Reconnection, instance isolation, direct example tests | 4, 5, 6 |
| Browser errors, trap limits, constructor limitations | 3, 8 |
| Native tests, example checks, browser/profile matrix, documentation tests | 7, 8 |
| Version change, ownership documentation, migration, no new root APIs | 2, 8 |

The example snippets and matrix are implementation instructions, not claims of code already running. The design experiments remain supporting evidence. Completion requires the repository and example tests described here.
