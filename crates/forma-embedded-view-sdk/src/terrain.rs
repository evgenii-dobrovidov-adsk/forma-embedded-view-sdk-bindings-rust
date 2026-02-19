use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Interact with the terrain in the 3D scene.
    pub type TerrainApi;

    #[wasm_bindgen(method, getter, js_name = "groundTexture")]
    pub fn ground_texture(this: &TerrainApi) -> GroundTextureApi;

    /// Fetch the bounding box for the terrain.
    #[wasm_bindgen(method, js_name = "getBbox")]
    pub fn get_bbox(this: &TerrainApi) -> js_sys::Promise;

    /// Get the elevation at a specific (x, y) point. Returns meters above sea level.
    #[wasm_bindgen(method, js_name = "getElevationAt")]
    pub fn get_elevation_at(this: &TerrainApi, request: &JsValue) -> js_sys::Promise;

    /// Retrieve all terrain pads defined in the current terrain.
    #[wasm_bindgen(method, js_name = "getPads")]
    pub fn get_pads(this: &TerrainApi) -> js_sys::Promise;

    /// Add new terrain pads to the existing pads.
    #[wasm_bindgen(method, js_name = "addPads")]
    pub fn add_pads(this: &TerrainApi, pads: &JsValue) -> js_sys::Promise;

    /// Replace all existing terrain pads with the provided array.
    #[wasm_bindgen(method, js_name = "applyPads")]
    pub fn apply_pads(this: &TerrainApi, pads: &JsValue) -> js_sys::Promise;
}

#[wasm_bindgen]
extern "C" {
    /// Manage ground textures applied to the terrain in the 3D scene.
    pub type GroundTextureApi;

    /// Add a ground texture to the terrain.
    #[wasm_bindgen(method)]
    pub fn add(this: &GroundTextureApi, request: &JsValue) -> js_sys::Promise;

    /// Update the texture data for an existing ground texture.
    #[wasm_bindgen(method, js_name = "updateTextureData")]
    pub fn update_texture_data(this: &GroundTextureApi, request: &JsValue) -> js_sys::Promise;

    /// Update the placement of an existing ground texture.
    #[wasm_bindgen(method, js_name = "updatePosition")]
    pub fn update_position(this: &GroundTextureApi, request: &JsValue) -> js_sys::Promise;

    /// Remove an existing ground texture.
    #[wasm_bindgen(method)]
    pub fn remove(this: &GroundTextureApi, request: &JsValue) -> js_sys::Promise;
}
