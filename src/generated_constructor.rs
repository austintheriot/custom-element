use std::ops::Deref;

use js_sys::{Array, Function, Reflect};
use wasm_bindgen::JsValue;

/// Wrapper type around the `GeneratedCustomElement`
/// class for easier/more ergonomic conversions
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeneratedConstructor(pub Function);

impl Deref for GeneratedConstructor {
    type Target = Function;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<Function> for GeneratedConstructor {
    fn as_ref(&self) -> &Function {
        &self.0
    }
}

impl GeneratedConstructor {
    /// Calls the `GeneratedCustomElement`'s constructor function,
    /// equivalent to calling `new GeneratedCustomElement()` in JavaScript
    pub fn construct(&self) -> Result<Function, JsValue> {
        Reflect::construct(self, &Array::new()).map(|js_value| js_value.into())
    }

    /// Calls the `GeneratedCustomElement`'s constructor function,
    /// equivalent to calling `new GeneratedCustomElement()` in JavaScript
    pub fn construct_with_arguments(&self, args: &Array) -> Result<Function, JsValue> {
        Reflect::construct(self, args).map(|js_value| js_value.into())
    }

    pub fn inner(&self) -> &Function {
        &self.0
    }

    pub fn into_inner(self) -> Function {
        self.0
    }
}
