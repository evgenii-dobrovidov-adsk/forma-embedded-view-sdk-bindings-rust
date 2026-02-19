use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Interact with the sun object in the 3D scene.
    pub type SunApi;

    /// Fetch the `Date` corresponding to the current sun position.
    #[wasm_bindgen(method, js_name = "getDate")]
    pub fn get_date(this: &SunApi) -> js_sys::Promise;

    /// Set the position of the sun in the scene by date.
    #[wasm_bindgen(method, js_name = "setDate")]
    pub fn set_date(this: &SunApi, request: &JsValue) -> js_sys::Promise;
}
