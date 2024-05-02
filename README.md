# custom-element

[![build](https://img.shields.io/github/actions/workflow/status/austintheriot/custom-element/tests.yml)](https://github.com/austintheriot/custom-element/actions)
[![crates.io](https://img.shields.io/crates/v/custom_element.svg)](https://crates.io/crates/custom_element)
[![docs.rs](https://docs.rs/custom_element/badge.svg)](https://docs.rs/custom_element)

## Overview

### The Problem

Extending a JS class from Rust/Wasm is not currently supported by `wasm-bindgen`: https://github.com/rustwasm/wasm-bindgen/issues/210, https://github.com/rustwasm/wasm-bindgen/issues/210. This functionality, however is essential for creating a custom element on the web, since custom elements [must inherit from `HTMLElement`](https://developer.mozilla.org/en-US/docs/Web/API/Web_Components/Using_custom_elements#implementing_a_custom_element) (or some other valid subclass of HTMLElement).

### The solution

This crates provide the JavaScript shim necessary for extending an arbitrary subclass of `HTMLElement` and forwards all custom element lifecycle method calls to the Rust struct you provide.

### Why this crate rather than the other available ones?

- Allows creating both autonomous custom elements AND customized built-in elements

- Provides all the valid HtmlElement constructors (behind a feature flag) for creating customized built-in elements

- Comes with a comprehensive test suite

- Extensive examples showing how to use this library to wrap Leptos components and call them from JS

- Less opinionated approach to creating custom elements. This library

### Current Rough Edges

- It's possible to run into the following issue if you are calling your custom element with both `&mut` and `&` pointers: https://github.com/rustwasm/wasm-bindgen/issues/1578. The ways that I know of to get around this issue are by either by scheduling mutable calls to the custom element instance to happen once the current function scope ends or by adding some indirection through `Rc<RefCell<T>>` If you have other ideas of how to reduce this issue, I'd be interested in a message or a PR.

## Other libraries you may be interested in

- [custom-elements](https://github.com/gbj/custom-elements)

- [webcomponents](https://github.com/richardanaya/webcomponent)
