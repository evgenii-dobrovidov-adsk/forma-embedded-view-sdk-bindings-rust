use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Result returned by subscription methods, containing an `unsubscribe` function.
    #[wasm_bindgen(js_name = Object)]
    pub type SubscriptionResult;

    #[wasm_bindgen(method, getter)]
    pub fn unsubscribe(this: &SubscriptionResult) -> js_sys::Function;
}
