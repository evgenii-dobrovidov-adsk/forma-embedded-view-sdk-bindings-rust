use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Access extension-specific services such as endpoints and cloud storage.
    pub type ExtensionsApi;

    #[wasm_bindgen(method, getter)]
    pub fn storage(this: &ExtensionsApi) -> ExtensionsStorageApi;

    /// Invoke an extension-specific endpoint.
    #[wasm_bindgen(method, js_name = "invokeEndpoint")]
    pub fn invoke_endpoint(this: &ExtensionsApi, request: &JsValue) -> js_sys::Promise;
}

#[wasm_bindgen]
extern "C" {
    /// Extension storage backed by AWS S3 for saving data per authcontext.
    pub type ExtensionsStorageApi;

    /// Add or replace a storage object.
    #[wasm_bindgen(method, js_name = "setObject")]
    pub fn set_object(this: &ExtensionsStorageApi, request: &JsValue) -> js_sys::Promise;

    /// Fetch string data for the specified key.
    #[wasm_bindgen(method, js_name = "getTextObject")]
    pub fn get_text_object(this: &ExtensionsStorageApi, request: &JsValue) -> js_sys::Promise;

    /// Fetch binary data for the specified key.
    #[wasm_bindgen(method, js_name = "getBinaryObject")]
    pub fn get_binary_object(this: &ExtensionsStorageApi, request: &JsValue) -> js_sys::Promise;

    /// List all storage objects for the extension.
    #[wasm_bindgen(method, js_name = "listObjects")]
    pub fn list_objects(this: &ExtensionsStorageApi, request: Option<&JsValue>) -> js_sys::Promise;

    /// Delete object corresponding to the specified key.
    #[wasm_bindgen(method, js_name = "deleteObject")]
    pub fn delete_object(this: &ExtensionsStorageApi, request: &JsValue) -> js_sys::Promise;
}
