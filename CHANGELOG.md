# Changelog

All notable changes to this crate are in this file. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/). The crate follows
[Semantic Versioning](https://semver.org/).

## 0.2.0 - 2026-10-10

### Changed

- All six `CustomElement` callbacks take `&self` instead of `&mut self`.
  Use interior mutability when a callback changes Rust-owned state.
- The factory passed to `create_custom_element` and
  `create_custom_element_with_config` is `Fn(HtmlElement, Array) -> Component`.
  It was `FnMut(JsValue, Array) -> Component`. The factory receives the
  element as `HtmlElement`, so it no longer converts a `JsValue`. The factory can capture shared configuration, but it must
  create a component for each call.
- The returned closure type is `Closure<dyn Fn(JsValue, Array) -> JsValue>`.
  It was a `dyn FnMut` closure. Keep the closure alive while the registered
  constructor can create elements.
- The default `connected_move_callback` calls `disconnected_callback` and
  then `connected_callback`. It did nothing before. The generated element
  class always defines `connectedMoveCallback`, so the browser skips its own
  disconnect and reconnect for an `Element.moveBefore()` move. A component
  that relied on the default lost both callbacks on such a move. Override
  `connected_move_callback` to keep state across a move.
- The crate requires `wasm-bindgen` 0.2.129 and `js-sys` and `web-sys`
  0.3.106 or later. Install the `wasm-bindgen` CLI version that your
  `Cargo.lock` resolves.
- The `constructors` module exposes functions instead of statics. Replace
  `&HTML_PARAGRAPH_ELEMENT_CONSTRUCTOR` with
  `html_paragraph_element_constructor()`, and the same for every other
  element. The statics used a `wasm-bindgen` form that is deprecated. The
  `HTMLDocument` entry is gone, because a document is not an element.
- `CustomElementConfiguration` owns its `element_constructor` as a
  `Function` instead of borrowing it, so the struct no longer implements
  `Copy` and has no lifetime parameter.
- `create_custom_element` and `create_custom_element_with_config` no longer
  take an attribute list. Implement `CustomElement::observed_attributes` on
  the component type instead. It returns `&'static [&'static str]` and
  defaults to an empty list.

### Migrating a mutable capture

A factory that changed a captured value no longer compiles. Move the value
into a `Cell`.

Before:

```rust,ignore
let mut sequence = 0;
let (closure, constructor) = create_custom_element(
    move |_element, _arguments| {
        sequence += 1;
        Item { sequence }
    },
    vec![],
);
```

After:

```rust,ignore
let sequence = Cell::new(0);
let (closure, constructor) = create_custom_element(
    move |_element, _arguments| {
        let next = sequence.get() + 1;
        sequence.set(next);
        Item { sequence: next }
    },
);
```
