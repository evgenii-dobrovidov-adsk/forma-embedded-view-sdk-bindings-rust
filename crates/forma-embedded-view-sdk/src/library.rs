use crate::types::*;
use crate::Result;
use wasm_bindgen_futures::JsFuture;

pub(crate) mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type LibraryApi;

        #[wasm_bindgen(method, js_name = "createItem")]
        pub fn create_item(this: &LibraryApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "updateItem")]
        pub fn update_item(this: &LibraryApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "deleteItem")]
        pub fn delete_item(this: &LibraryApi, request: &JsValue) -> ::js_sys::Promise;
    }
}

/// Manage items in the user's Library.
pub struct LibraryApi {
    inner: js::LibraryApi,
}

impl LibraryApi {
    pub(crate) fn from_raw(raw: js::LibraryApi) -> Self {
        Self { inner: raw }
    }

    /// Add data to Library as a new item.
    pub async fn create_item(&self, request: &LibraryCreateItemRequest) -> Result<LibraryItem> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.create_item(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Update an existing library item.
    pub async fn update_item(&self, request: &LibraryUpdateItemRequest) -> Result<LibraryItem> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.update_item(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Delete an existing library item.
    pub async fn delete_item(&self, request: &LibraryDeleteItemRequest) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.delete_item(&js_req)).await?;
        Ok(())
    }
}
