use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Add and remove a colorbar to the scene view for analysis heatmaps.
    pub type ColorbarApi;

    /// Add a colorbar to the scene view. Only one colorbar at a time.
    #[wasm_bindgen(method)]
    pub fn add(this: &ColorbarApi, request: &JsValue) -> js_sys::Promise;

    /// Remove the colorbar added by this embedded view.
    #[wasm_bindgen(method)]
    pub fn remove(this: &ColorbarApi) -> js_sys::Promise;
}
