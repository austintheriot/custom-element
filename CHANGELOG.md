# Changelog

All notable changes to this crate are in this file. The format follows
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/). The crate follows
[Semantic Versioning](https://semver.org/).

## 0.2.0 - 2026-10-10

### Changed

- All six `CustomElement` callbacks take `&self` instead of `&mut self`.
  Use interior mutability when a callback changes Rust-owned state.
- The factory passed to `create_custom_element` and
  `create_custom_element_with_config` is `Fn(JsValue, Array) -> Component`.
  It was `FnMut`. The factory can capture shared configuration, but it must
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
    vec![],
);
```
