use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Access project-level metadata.
    pub type ProjectApi;

    /// Fetch all project metadata. Resolves to `Project`.
    #[wasm_bindgen(method)]
    pub fn get(this: &ProjectApi) -> js_sys::Promise;

    /// Fetch project location. Resolves to `[latitude, longitude] | undefined`.
    #[wasm_bindgen(method, js_name = "getGeoLocation")]
    pub fn get_geo_location(this: &ProjectApi) -> js_sys::Promise;
}
