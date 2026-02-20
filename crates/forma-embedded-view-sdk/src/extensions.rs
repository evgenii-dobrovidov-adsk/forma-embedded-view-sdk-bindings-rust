use crate::types::*;
use crate::Result;
use wasm_bindgen_futures::JsFuture;

pub(crate) mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type ExtensionsApi;

        #[wasm_bindgen(method, getter)]
        pub fn storage(this: &ExtensionsApi) -> ExtensionsStorageApi;

        #[wasm_bindgen(method, js_name = "invokeEndpoint")]
        pub fn invoke_endpoint(this: &ExtensionsApi, request: &JsValue) -> ::js_sys::Promise;
    }

    #[wasm_bindgen]
    extern "C" {
        pub type ExtensionsStorageApi;

        #[wasm_bindgen(method, js_name = "setObject")]
        pub fn set_object(this: &ExtensionsStorageApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getTextObject")]
        pub fn get_text_object(this: &ExtensionsStorageApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getBinaryObject")]
        pub fn get_binary_object(
            this: &ExtensionsStorageApi,
            request: &JsValue,
        ) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "listObjects")]
        pub fn list_objects(
            this: &ExtensionsStorageApi,
            request: Option<&JsValue>,
        ) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "deleteObject")]
        pub fn delete_object(
            this: &ExtensionsStorageApi,
            request: &JsValue,
        ) -> ::js_sys::Promise;
    }
}

/// Access extension-specific services such as endpoints and cloud storage.
pub struct ExtensionsApi {
    inner: js::ExtensionsApi,
}

impl ExtensionsApi {
    pub(crate) fn from_raw(raw: js::ExtensionsApi) -> Self {
        Self { inner: raw }
    }

    /// Access the extension storage sub-API.
    pub fn storage(&self) -> ExtensionsStorageApi {
        ExtensionsStorageApi {
            inner: self.inner.storage(),
        }
    }

    /// Invoke an extension-specific endpoint.
    pub async fn invoke_endpoint(
        &self,
        request: &InvokeEndpointRequest,
    ) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.invoke_endpoint(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }
}

/// Extension storage backed by AWS S3 for saving data per authcontext.
pub struct ExtensionsStorageApi {
    inner: js::ExtensionsStorageApi,
}

impl ExtensionsStorageApi {
    /// Add or replace a storage object.
    pub async fn set_object(&self, request: &StorageSetObjectRequest) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.set_object(&js_req)).await?;
        Ok(())
    }

    /// Fetch string data for the specified key.
    pub async fn get_text_object(
        &self,
        request: &StorageGetTextObjectRequest,
    ) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.get_text_object(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Fetch binary data for the specified key.
    pub async fn get_binary_object(
        &self,
        request: &StorageGetBinaryObjectRequest,
    ) -> Result<Vec<u8>> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.get_binary_object(&js_req)).await?;
        let array = ::js_sys::Uint8Array::new(&result);
        Ok(array.to_vec())
    }

    /// List all storage objects for the extension.
    pub async fn list_objects(
        &self,
        request: Option<&StorageListObjectsRequest>,
    ) -> Result<serde_json::Value> {
        let js_req = request
            .map(serde_wasm_bindgen::to_value)
            .transpose()?;
        let result =
            JsFuture::from(self.inner.list_objects(js_req.as_ref())).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Delete object corresponding to the specified key.
    pub async fn delete_object(&self, request: &StorageDeleteObjectRequest) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.delete_object(&js_req)).await?;
        Ok(())
    }
}
