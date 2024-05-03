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

### Current Rough Edges

- It's possible to run into the following issue if you are calling your custom element with both `&mut` and `&` pointers: https://github.com/rustwasm/wasm-bindgen/issues/1578. The ways that I know of to get around this issue are by either by scheduling mutable calls to the custom element instance to happen once the current function scope ends or by adding some indirection through `Rc<RefCell<T>>` If you have other ideas of how to reduce this issue, I'd be interested in a message or a PR.

## Other libraries you may be interested in

- [custom-elements](https://github.com/gbj/custom-elements) - the library that inspired this one
- [webcomponents](https://github.com/richardanaya/webcomponent) - a different implementation approach that doesn't use `wasm-bindgen`
