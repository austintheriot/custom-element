# custom-element

## Overview

## The Problem

Extending a JS class from Rust/Wasm is not currently supported by `wasm-bindgen`: https://github.com/rustwasm/wasm-bindgen/issues/210, https://github.com/rustwasm/wasm-bindgen/issues/210. This functionality, however is essential for creating a custom element on the web, since custom elements [must inherit from `HTMLElement`](https://developer.mozilla.org/en-US/docs/Web/API/Web_Components/Using_custom_elements#implementing_a_custom_element) (or some other valid subclass of HTMLElement).

## The solution

This crates provide the JavaScript shim necessary for extending an arbitrary subclass of `HTMLElement` and forwards all custom element lifecycle method calls to the Rust struct you provide.

## Why this crate rather than the other available ones?

- Allows creating autonomous custom elements OR customized built-in elements

- Provides all the valid HtmlElement constructors (behind a feature flag) for creating customized built-in elements

- Comes with a comprehensive test suite

- Less opinionated approach to creating custom elements. This library

## Rough Edges

- It's possible to run into the following issue if you are calling your custom element with both `&mut` and `&` pointers: https://github.com/rustwasm/wasm-bindgen/issues/1578

## Set up

For steps for compiling this repo locally, see [./SETUP.md](./SETUP.md)

## Scripts

See available scripts at [./Makefile.toml](./Makefile.toml)
