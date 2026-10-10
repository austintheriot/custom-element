#![deny(missing_docs)]

//! # About custom-element
//!
//! `custom-element` provides a `CustomElement` trait that, when implemented,
//! allows your Rust struct to be rendered as a web component on any html page.
//!
//! Web components are a great way to [eliminate frontend framework lock-in](https://jakelazaroff.com/words/web-components-eliminate-javascript-framework-lock-in/).
//! They are also a great way to bridge the divide between UI components written in JavaScript
//! and UI elements written in Rust.
//!
//! # Learn By Example
//! If you want to see some examples of writing web components in Rust, check out
//! [examples](https://github.com/austintheriot/custom-element/tree/main/examples) directory:
//! - [`hello_world`](https://github.com/austintheriot/custom-element/tree/main/examples/hello_world) is the most basic web component you can create
//! - [`counter`](https://github.com/austintheriot/custom-element/tree/main/examples/counter) is the standard
//! counter example, with attributes as its state.
//! - [`cell_state`](https://github.com/austintheriot/custom-element/tree/main/examples/cell_state)
//! stores one value in a `Cell`.
//! - [`refcell_state`](https://github.com/austintheriot/custom-element/tree/main/examples/refcell_state)
//! stores compound state in a scoped `RefCell` borrow.
//! - [`leptos_in_react`](https://github.com/austintheriot/custom-element/tree/main/examples/leptos_in_react)
//! shows how you can mix Rust and JavaScript in the same codebase, and even mix Rust/JavaScript
//! frameworks.
//!
//! # A Simple Hello World
//!
//! ```rust,no_run
//! # use custom_element::CustomElement;
//! # use js_sys::Array;
//! # use web_sys::{HtmlElement, ShadowRootInit, ShadowRootMode};
//! #
//! struct HelloWorld;
//!
//! // all custom element lifecycle hooks have a default no-op implementation
//! impl CustomElement for HelloWorld {}
//!
//! impl HelloWorld {
//!     // called from the JavaScript custom element's `constructor`
//!     fn new(instance: HtmlElement, _args: Array) -> Self {
//!         // a constructor must not add children to the element itself,
//!         // so the text goes into a shadow root
//!         let shadow_root = instance
//!             .attach_shadow(&ShadowRootInit::new(ShadowRootMode::Open))
//!             .unwrap();
//!         shadow_root.set_text_content(Some("Hello, world!"));
//!         HelloWorld
//!     }
//! }
//!
//! fn main() {
//!     // create custom element constructor
//!     let component_name = "hello-world";
//!     let (closure, constructor) = custom_element::create_custom_element(HelloWorld::new, vec![]);
//!
//!     // we want the Rust code for our custom element to live forever
//!     closure.forget();
//!
//!     // define the element
//!     let window = web_sys::window().unwrap();
//!     window
//!         .custom_elements()
//!         .define(component_name, &constructor)
//!         .unwrap();
//!
//!     // render in the DOM
//!     let body = window.document().unwrap().body().unwrap();
//!     body.set_inner_html(&format!("<{0}></{0}>", component_name));
//! }
//! ```
//!
//! # Shared callbacks
//!
//! Each [`CustomElement`] callback takes a shared receiver. A component can
//! change its state through standard interior mutability. This example runs
//! without browser globals:
//!
//! ```
//! use std::{cell::Cell, rc::Rc};
//! use custom_element::CustomElement;
//!
//! struct Visits(Rc<Cell<u32>>);
//! impl CustomElement for Visits {
//!     fn connected_callback(&self) {
//!         self.0.set(self.0.get() + 1);
//!     }
//! }
//! let count = Rc::new(Cell::new(0));
//! let visits = Visits(count.clone());
//! CustomElement::connected_callback(&visits);
//! assert_eq!(count.get(), 1);
//! ```
//!
//! A callback can call browser code that synchronously invokes another
//! callback. End any conflicting `RefCell` guard before that call. A shared
//! guard conflicts with a nested mutable borrow, and a mutable guard
//! conflicts with every nested borrow. Read state again after a nested call
//! if later work needs its current value.

mod bridge;
mod create_custom_element;
mod custom_element;
mod generated_constructor;
mod html_constructors;

pub use bridge::*;
pub use create_custom_element::*;
pub use custom_element::*;
pub use generated_constructor::*;
pub use html_constructors::*;
