use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Create and upload elements to the integrate element system.
    pub type IntegrateApi;

    /// Create a hierarchy of elements (deprecated in favor of `create_element_v2`).
    #[wasm_bindgen(method, js_name = "createElementHierarchy")]
    pub fn create_element_hierarchy(this: &IntegrateApi, request: &JsValue) -> js_sys::Promise;

    /// Create a new element (v2). Resolves to `{ urn }`.
    #[wasm_bindgen(method, js_name = "createElementV2")]
    pub fn create_element_v2(this: &IntegrateApi, request: &JsValue) -> js_sys::Promise;

    /// Update an existing element. The update is merged onto the existing element.
    #[wasm_bindgen(method, js_name = "updateElementV2")]
    pub fn update_element_v2(this: &IntegrateApi, request: &JsValue) -> js_sys::Promise;

    /// Create and/or update multiple elements in a batch.
    #[wasm_bindgen(method, js_name = "batchIngestElementsV2")]
    pub fn batch_ingest_elements_v2(this: &IntegrateApi, request: &JsValue) -> js_sys::Promise;

    /// Upload a file to integrate file storage. Resolves to `{ fileId, blobId }`.
    #[wasm_bindgen(method, js_name = "uploadFile")]
    pub fn upload_file(this: &IntegrateApi, request: &JsValue) -> js_sys::Promise;

    /// Create a new URN for an element.
    #[wasm_bindgen(method, js_name = "createUrn")]
    pub fn create_urn(this: &IntegrateApi, authcontext: &str) -> String;
}
