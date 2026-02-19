use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Manage items in the user's Library.
    pub type LibraryApi;

    /// Add data to Library as a new item.
    #[wasm_bindgen(method, js_name = "createItem")]
    pub fn create_item(this: &LibraryApi, request: &JsValue) -> js_sys::Promise;

    /// Update an existing library item.
    #[wasm_bindgen(method, js_name = "updateItem")]
    pub fn update_item(this: &LibraryApi, request: &JsValue) -> js_sys::Promise;

    /// Delete an existing library item.
    #[wasm_bindgen(method, js_name = "deleteItem")]
    pub fn delete_item(this: &LibraryApi, request: &JsValue) -> js_sys::Promise;
}
