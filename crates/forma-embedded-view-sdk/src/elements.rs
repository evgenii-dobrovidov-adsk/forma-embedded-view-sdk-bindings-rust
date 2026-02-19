use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Read elements and create buildings in Forma's element systems.
    pub type ElementsApi;

    #[wasm_bindgen(method, getter, js_name = "floorStack")]
    pub fn floor_stack(this: &ElementsApi) -> FloorStackApi;

    #[wasm_bindgen(method, getter)]
    pub fn representations(this: &ElementsApi) -> RepresentationsApi;

    #[wasm_bindgen(method, getter)]
    pub fn blobs(this: &ElementsApi) -> BlobsApi;

    /// Get an element by URN. Resolves to `{ element, elements }`.
    #[wasm_bindgen(method)]
    pub fn get(this: &ElementsApi, request: &JsValue) -> js_sys::Promise;

    /// Get an element hierarchy located at a path relative to the root.
    #[wasm_bindgen(method, js_name = "getByPath")]
    pub fn get_by_path(this: &ElementsApi, request: &JsValue) -> js_sys::Promise;

    /// Get the world transform of an element relative to the root element.
    #[wasm_bindgen(method, js_name = "getWorldTransform")]
    pub fn get_world_transform(this: &ElementsApi, request: &JsValue) -> js_sys::Promise;

    /// Add, edit, and remove custom properties on elements via JSON Merge Patch.
    #[wasm_bindgen(method, js_name = "editProperties")]
    pub fn edit_properties(this: &ElementsApi, request: &JsValue) -> js_sys::Promise;
}

#[wasm_bindgen]
extern "C" {
    /// Create Floor Stack buildings.
    pub type FloorStackApi;

    /// Create a 2.5D building from a stack of floors.
    #[wasm_bindgen(method, js_name = "createFromFloors")]
    pub fn create_from_floors(this: &FloorStackApi, request: &JsValue) -> js_sys::Promise;

    /// Create multiple 2.5D buildings from stacks of floors.
    #[wasm_bindgen(method, js_name = "createFromFloorsBatch")]
    pub fn create_from_floors_batch(this: &FloorStackApi, request: &JsValue) -> js_sys::Promise;
}

#[wasm_bindgen]
extern "C" {
    /// Access derived representations of elements (volume mesh, footprint, etc.).
    pub type RepresentationsApi;

    /// Get the volume mesh for an element (GLB data).
    #[wasm_bindgen(method, js_name = "volumeMesh")]
    pub fn volume_mesh(this: &RepresentationsApi, request: &JsValue) -> js_sys::Promise;

    /// Get the footprint of an element (GeoJSON FeatureCollection).
    #[wasm_bindgen(method)]
    pub fn footprint(this: &RepresentationsApi, request: &JsValue) -> js_sys::Promise;

    /// Get the gross floor area polygons for an element.
    #[wasm_bindgen(method, js_name = "grossFloorAreaPolygons")]
    pub fn gross_floor_area_polygons(
        this: &RepresentationsApi,
        request: &JsValue,
    ) -> js_sys::Promise;

    /// Get the graph building representation for an element.
    #[wasm_bindgen(method, js_name = "graphBuilding")]
    pub fn graph_building(this: &RepresentationsApi, request: &JsValue) -> js_sys::Promise;
}

#[wasm_bindgen]
extern "C" {
    /// Read blobs related to an element.
    pub type BlobsApi;

    /// Retrieve a blob by its ID. Resolves to `{ data: ArrayBuffer }`.
    #[wasm_bindgen(method)]
    pub fn get(this: &BlobsApi, request: &JsValue) -> js_sys::Promise;
}
