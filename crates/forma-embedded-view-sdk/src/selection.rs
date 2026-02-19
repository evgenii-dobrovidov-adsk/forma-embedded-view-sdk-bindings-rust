use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Interact with user's selection (shift-clicked elements in the scene).
    pub type SelectionApi;

    /// Get selected element paths. Resolves to `string[]`.
    #[wasm_bindgen(method, js_name = "getSelection")]
    pub fn get_selection(this: &SelectionApi) -> js_sys::Promise;

    /// Subscribe to selection changes.
    #[wasm_bindgen(method)]
    pub fn subscribe(
        this: &SelectionApi,
        callback: &Closure<dyn FnMut(JsValue)>,
    ) -> js_sys::Promise;
}
