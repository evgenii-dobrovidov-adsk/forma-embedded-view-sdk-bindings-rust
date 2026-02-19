use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Manage generators registered in Forma.
    pub type GeneratorsApi;

    /// Create or replace a generator.
    #[wasm_bindgen(method)]
    pub fn put(this: &GeneratorsApi, request: &JsValue) -> js_sys::Promise;

    /// List generators within the specified authcontext.
    #[wasm_bindgen(method)]
    pub fn list(this: &GeneratorsApi, request: Option<&JsValue>) -> js_sys::Promise;
}
