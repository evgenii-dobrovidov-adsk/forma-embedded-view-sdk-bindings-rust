use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Render or color objects and elements in the 3D scene (non-persistent visual changes).
    pub type RenderApi;

    #[wasm_bindgen(method, getter)]
    pub fn glb(this: &RenderApi) -> RenderGlbApi;

    #[wasm_bindgen(method, getter)]
    pub fn geojson(this: &RenderApi) -> RenderGeojsonApi;

    #[wasm_bindgen(method, getter, js_name = "elementColors")]
    pub fn element_colors(this: &RenderApi) -> ElementColorApi;

    /// Hide an element from the scene.
    #[wasm_bindgen(method, js_name = "hideElement")]
    pub fn hide_element(this: &RenderApi, request: &JsValue) -> js_sys::Promise;

    /// Hide a set of elements from the scene.
    #[wasm_bindgen(method, js_name = "hideElementsBatch")]
    pub fn hide_elements_batch(this: &RenderApi, request: &JsValue) -> js_sys::Promise;

    /// Unhide an element from the scene.
    #[wasm_bindgen(method, js_name = "unhideElement")]
    pub fn unhide_element(this: &RenderApi, request: &JsValue) -> js_sys::Promise;

    /// Unhide a set of elements from the scene.
    #[wasm_bindgen(method, js_name = "unhideElementsBatch")]
    pub fn unhide_elements_batch(this: &RenderApi, request: &JsValue) -> js_sys::Promise;

    /// Set element visibility by path.
    #[wasm_bindgen(method, js_name = "setElementsVisibility")]
    pub fn set_elements_visibility(this: &RenderApi, request: &JsValue) -> js_sys::Promise;

    /// Unhide all elements previously hidden by this API.
    #[wasm_bindgen(method, js_name = "unhideAllElements")]
    pub fn unhide_all_elements(this: &RenderApi) -> js_sys::Promise;

    /// Add a mesh to the scene. Resolves to `{ id }`.
    #[wasm_bindgen(method, js_name = "addMesh")]
    pub fn add_mesh(this: &RenderApi, request: &JsValue) -> js_sys::Promise;

    /// Upsert a mesh in the scene.
    #[wasm_bindgen(method, js_name = "updateMesh")]
    pub fn update_mesh(this: &RenderApi, request: &JsValue) -> js_sys::Promise;

    /// Remove a mesh from the scene.
    #[wasm_bindgen(method)]
    pub fn remove(this: &RenderApi, request: &JsValue) -> js_sys::Promise;

    /// Remove all meshes added by this API from the scene.
    #[wasm_bindgen(method)]
    pub fn cleanup(this: &RenderApi) -> js_sys::Promise;
}

#[wasm_bindgen]
extern "C" {
    /// Render GLBs in the 3D scene.
    pub type RenderGlbApi;

    /// Add a GLB to the scene. Resolves to `{ id }`.
    #[wasm_bindgen(method)]
    pub fn add(this: &RenderGlbApi, request: &JsValue) -> js_sys::Promise;

    /// Upsert a GLB in the scene.
    #[wasm_bindgen(method)]
    pub fn update(this: &RenderGlbApi, request: &JsValue) -> js_sys::Promise;

    /// Remove a GLB from the scene.
    #[wasm_bindgen(method)]
    pub fn remove(this: &RenderGlbApi, request: &JsValue) -> js_sys::Promise;

    /// Remove all GLBs added by this API from the scene.
    #[wasm_bindgen(method)]
    pub fn cleanup(this: &RenderGlbApi) -> js_sys::Promise;
}

#[wasm_bindgen]
extern "C" {
    /// Render GeoJSON in the 3D scene.
    pub type RenderGeojsonApi;

    /// Add GeoJSON to the scene. Resolves to `{ id }`.
    #[wasm_bindgen(method)]
    pub fn add(this: &RenderGeojsonApi, request: &JsValue) -> js_sys::Promise;

    /// Upsert GeoJSON in the scene.
    #[wasm_bindgen(method)]
    pub fn update(this: &RenderGeojsonApi, request: &JsValue) -> js_sys::Promise;

    /// Remove GeoJSON from the scene.
    #[wasm_bindgen(method)]
    pub fn remove(this: &RenderGeojsonApi, request: &JsValue) -> js_sys::Promise;

    /// Remove all GeoJSON added by this API from the scene.
    #[wasm_bindgen(method)]
    pub fn cleanup(this: &RenderGeojsonApi) -> js_sys::Promise;
}

#[wasm_bindgen]
extern "C" {
    /// Set custom colors on elements in the scene.
    pub type ElementColorApi;

    /// Set color override on the specified elements.
    #[wasm_bindgen(method)]
    pub fn set(this: &ElementColorApi, request: &JsValue) -> js_sys::Promise;

    /// Clear color override on the specified elements.
    #[wasm_bindgen(method)]
    pub fn clear(this: &ElementColorApi, request: &JsValue) -> js_sys::Promise;

    /// Clear all color overrides.
    #[wasm_bindgen(method, js_name = "clearAll")]
    pub fn clear_all(this: &ElementColorApi) -> js_sys::Promise;
}
