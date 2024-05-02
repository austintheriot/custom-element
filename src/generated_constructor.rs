use std::ops::Deref;

use js_sys::{Array, Function, Reflect};
use wasm_bindgen::JsValue;

/// Wrapper type around the JavaScript `GeneratedCustomElement`
/// class for easier/more ergonomic calling/conversions
#[derive(Clone, Eq, PartialEq, Debug, Default)]
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

impl AsMut<Function> for GeneratedConstructor {
    fn as_mut(&mut self) -> &mut Function {
        &mut self.0
    }
}

impl GeneratedConstructor {
    /// Calls the `GeneratedCustomElement`'s constructor function with arguments.
    /// Equivalent to calling `new GeneratedCustomElement()` in JavaScript
    pub fn construct(&self) -> Result<Function, JsValue> {
        Reflect::construct(self, &Array::new()).map(|js_value| js_value.into())
    }

    /// Calls the `GeneratedCustomElement`'s constructor function with arguments.
    /// Equivalent to calling `new GeneratedCustomElement()` in JavaScript
    pub fn construct_with_arguments(
        &self,
        args: impl AsRef<[JsValue]>,
    ) -> Result<Function, JsValue> {
        self.construct_with_array_arguments(&Array::from_iter(args.as_ref()))
    }

    /// Calls the `GeneratedCustomElement`'s constructor function with arguments.
    /// Equivalent to calling `new GeneratedCustomElement()` in JavaScript
    #[doc(hidden)]
    pub fn construct_with_array_arguments(&self, args: &Array) -> Result<Function, JsValue> {
        Reflect::construct(self, args).map(|js_value| js_value.into())
    }

    /// Get raw, inner JavaScript [`js_sys::Function`]
    pub fn inner(&self) -> &Function {
        &self.0
    }

    /// Clone and return the inner JavaScript [`js_sys::Function`]
    pub fn to_inner(&self) -> Function {
        self.0.clone()
    }

    /// Convert into the raw, inner JavaScript [`js_sys::Function`]
    pub fn into_inner(self) -> Function {
        self.0
    }
}
