use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Calculate area metrics / key figures for elements.
    pub type AreaMetricsApi;

    /// Calculate area metrics for the given paths. If no paths are given,
    /// metrics are calculated for all elements.
    #[wasm_bindgen(method)]
    pub fn calculate(this: &AreaMetricsApi, request: &JsValue) -> js_sys::Promise;
}
