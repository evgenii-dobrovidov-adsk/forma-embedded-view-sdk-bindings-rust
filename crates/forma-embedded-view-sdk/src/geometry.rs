use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Read geometry data from Forma (paths by category, footprints, triangles).
    pub type GeometryApi;

    /// Fetch paths of all elements tagged with a specific category.
    #[wasm_bindgen(method, js_name = "getPathsByCategory")]
    pub fn get_paths_by_category(this: &GeometryApi, request: &JsValue) -> js_sys::Promise;

    /// Fetch paths of all elements with the 'virtual' property set to true.
    #[wasm_bindgen(method, js_name = "getPathsForVirtualElements")]
    pub fn get_paths_for_virtual_elements(
        this: &GeometryApi,
        request: Option<&JsValue>,
    ) -> js_sys::Promise;

    /// Fetch the footprint representation of an element.
    #[wasm_bindgen(method, js_name = "getFootprint")]
    pub fn get_footprint(this: &GeometryApi, request: &JsValue) -> js_sys::Promise;

    /// Fetch concatenated triangle mesh for an element and its children.
    /// Resolves to `Float32Array`.
    #[wasm_bindgen(method, js_name = "getTriangles")]
    pub fn get_triangles(this: &GeometryApi, request: Option<&JsValue>) -> js_sys::Promise;

    /// Get all paths where geometry overlaps the provided polygons.
    #[wasm_bindgen(method, js_name = "getPathsInsidePolygons")]
    pub fn get_paths_inside_polygons(this: &GeometryApi, request: &JsValue) -> js_sys::Promise;
}
