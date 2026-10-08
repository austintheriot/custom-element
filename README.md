# custom-element

[![build](https://img.shields.io/github/actions/workflow/status/austintheriot/custom-element/tests.yml?label=tests)](https://github.com/austintheriot/custom-element/actions)
[![crates.io](https://img.shields.io/crates/v/custom_element.svg)](https://crates.io/crates/custom_element)
[![docs.rs](https://img.shields.io/docsrs/custom-element)](https://docs.rs/custom_element)

## Overview

### What is a custom element / what is a web component?

First off "web components" are just another name for custom elements, and, in the words of [Jake Lazaroff](https://jakelazaroff.com/words/web-components-will-outlive-your-javascript-framework/)

> They’re a set of W3C standards for building reusable HTML elements. You use them by writing a class for a custom element, registering a tag name and using it in your markup.

For us, this means that we can write HTML elements that run in the browser using Rust, and no other elements on the page have to know! It's a nice way to bridge the gap between HTML/JS/CSS and Rust/Wasm on the web. As a UI/library author, you can leverage all the flexibility of JS/HTML and still get all the tooling/performance/safety of Rust whenever and wherever you need it.

### The Problem

Extending a JS class from Rust/Wasm is not currently supported by `wasm-bindgen`: https://github.com/rustwasm/wasm-bindgen/issues/210, https://github.com/rustwasm/wasm-bindgen/issues/210. This functionality, however is essential for creating a custom element on the web, since custom elements [must inherit from `HTMLElement`](https://developer.mozilla.org/en-US/docs/Web/API/Web_Components/Using_custom_elements#implementing_a_custom_element) (or some other valid subclass of HTMLElement).

### The solution

This crates provide the JavaScript shim necessary for extending an arbitrary subclass of `HTMLElement` and forwards all custom element lifecycle method calls to the Rust struct you provide.

### Why this crate rather than the other available ones?

- Allows creating both [autonomous custom elements AND customized built-in elements](https://developer.mozilla.org/en-US/docs/Web/API/Web_Components/Using_custom_elements#types_of_custom_element)

- Provides all the valid HtmlElement constructors for creating customized built-in elements--these are not provided by `wasm-bindgen` out-of-the-box, and it's not obvious how to access them otherwise.

- Contains both simple examples for copying into your own project, and also advanced examples showing how to use this library to wrap Leptos components (for example) and render them from JavaScript/React

- Less opinionated approach to creating custom elements. This library gives you the JavaScript constructor for the wrapper class around your struct, and you decide what to do with it.

### Migrating to 0.2.0

All six `CustomElement` callbacks now take `&self`. Use interior mutability
when a callback changes Rust-owned state. A component can also keep its
state in element attributes. The factory passed to
`create_custom_element` now implements `Fn(JsValue, Array) -> Component`.
It can capture shared configuration, but each call must create a component
for the new element. Keep the returned closure alive while the registered
constructor can create elements.

For a scalar value, use `Cell`. Store the new value before a browser operation
that can call another lifecycle method:

```rust
use std::cell::Cell;
use custom_element::CustomElement;
use web_sys::HtmlElement;

struct Counter { element: HtmlElement, count: Cell<u32> }

impl CustomElement for Counter {
    fn connected_callback(&self) {
        let next = self.count.get() + 1;
        self.count.set(next);
        self.element.set_attribute("count", &next.to_string()).unwrap();
    }
}
```

For compound state, use a short `RefCell` borrow. Copy or build the data
needed for the browser operation before the borrow ends:

```rust
use std::cell::RefCell;
use custom_element::CustomElement;
use web_sys::HtmlElement;

struct Model { count: u32, updates: u32 }
struct Counter { output: HtmlElement, model: RefCell<Model> }

impl CustomElement for Counter {
    fn connected_callback(&self) {
        let label = {
            let mut model = self.model.borrow_mut();
            model.count += 1;
            model.updates += 1;
            format!("Count: {}; updates: {}", model.count, model.updates)
        };
        self.output.set_text_content(Some(&label));
    }
}
```

For shared factory inputs, capture an owned value. The closure must not
consume its capture, because the browser calls the factory for every element:

```rust,no_run
use std::rc::Rc;
use js_sys::Array;
use wasm_bindgen::JsValue;
use custom_element::{create_custom_element, CustomElement};

struct Greeting { text: Rc<String> }
impl CustomElement for Greeting {}

let shared_text = Rc::new(String::from("Hello"));
let (factory, constructor) = create_custom_element(
    move |_element: JsValue, _args: Array| Greeting {
        text: Rc::clone(&shared_text),
    },
    vec![],
);
factory.forget();
let _ = constructor;
```

The runnable examples show [attribute-only state](examples/counter),
[scalar `Cell` state](examples/cell_state), and
[scoped `RefCell` state](examples/refcell_state).

Shared callbacks permit synchronous reentry only when application state has
no conflicting borrow. A live `RefCell` mutable guard conflicts with every
nested borrow. A live shared guard conflicts with a nested mutable borrow.
End each guard before changing an attribute, dispatching an event, or calling
browser code that can run a callback. `Rc` shares ownership but does not
remove these borrow conflicts. State can change during a nested call, so
read it again after the browser operation when later work needs the current
value. Dropping a scope-local reference to `self` does not release a borrow
held by an exported wrapper.

Browser custom-element upgrade rules also apply independently of Rust
borrowing. A constructor cannot perform recursive upgrades of the same
element. Keep constructor work within browser custom-element restrictions.

## Other libraries you may be interested in

- [custom-elements](https://github.com/gbj/custom-elements) - the library that inspired this one
- [webcomponents](https://github.com/richardanaya/webcomponent) - a different implementation approach that doesn't use `wasm-bindgen`
