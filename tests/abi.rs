use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn should_allow_interpreting_jsvalue_as_struct() {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    struct Test {
        example: u32,
    }

    let test = Test { example: 42 };

    let v: JsValue = test.into();

    #[wasm_bindgen(inline_js = "export function castInto(a) { return a }")]
    extern "C" {
        // This will let Rust regain ownership of `Foo`
        #[wasm_bindgen(js_name = castInto)]
        pub fn cast_into(value: JsValue) -> Test;
    }

    let test2: Test = cast_into(v);

    assert_eq!(test2.example, 42);
}
