use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Upload buildings, roads, and property boundaries as GeoJSON.
    pub type GeoDataApi;

    /// Upload GeoJSON data to Forma and add it to the library.
    /// Supports 2.5D buildings, roads, and property boundaries.
    #[wasm_bindgen(method)]
    pub fn upload(this: &GeoDataApi, request: &JsValue) -> js_sys::Promise;
}
