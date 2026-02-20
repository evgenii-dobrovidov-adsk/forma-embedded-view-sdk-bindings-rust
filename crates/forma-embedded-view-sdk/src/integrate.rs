use crate::types::*;
use crate::Result;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

pub(crate) mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type IntegrateApi;

        #[wasm_bindgen(method, js_name = "createElementHierarchy")]
        pub fn create_element_hierarchy(
            this: &IntegrateApi,
            request: &JsValue,
        ) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "createElementV2")]
        pub fn create_element_v2(this: &IntegrateApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "updateElementV2")]
        pub fn update_element_v2(this: &IntegrateApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "batchIngestElementsV2")]
        pub fn batch_ingest_elements_v2(
            this: &IntegrateApi,
            request: &JsValue,
        ) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "uploadFile")]
        pub fn upload_file(this: &IntegrateApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "createUrn")]
        pub fn create_urn(this: &IntegrateApi, authcontext: &str) -> String;
    }
}

/// Create and upload elements to the integrate element system.
pub struct IntegrateApi {
    inner: js::IntegrateApi,
}

impl IntegrateApi {
    pub(crate) fn from_raw(raw: js::IntegrateApi) -> Self {
        Self { inner: raw }
    }

    /// Create a hierarchy of elements (deprecated in favor of `create_element_v2`).
    pub async fn create_element_hierarchy(
        &self,
        request: &CreateElementHierarchyRequest,
    ) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.create_element_hierarchy(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Create a new element (v2).
    pub async fn create_element_v2(
        &self,
        request: &CreateElementV2Request,
    ) -> Result<UrnResult> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.create_element_v2(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Update an existing element. The update is merged onto the existing element.
    pub async fn update_element_v2(
        &self,
        request: &UpdateElementV2Request,
    ) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.update_element_v2(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Create and/or update multiple elements in a batch.
    pub async fn batch_ingest_elements_v2(
        &self,
        request: &BatchIngestElementsV2Request,
    ) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.batch_ingest_elements_v2(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Upload a file to integrate file storage.
    pub async fn upload_file(&self, request: &UploadFileRequest) -> Result<FileUploadResult> {
        let obj = ::js_sys::Object::new();
        let buf = ::js_sys::Uint8Array::new_with_length(request.data.len() as u32);
        buf.copy_from(&request.data);
        ::js_sys::Reflect::set(&obj, &JsValue::from_str("data"), &buf.buffer())?;
        ::js_sys::Reflect::set(&obj, &JsValue::from_str("name"), &JsValue::from_str(&request.name))?;
        let result = JsFuture::from(self.inner.upload_file(&obj)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Create a new URN for an element.
    pub fn create_urn(&self, authcontext: &str) -> String {
        self.inner.create_urn(authcontext)
    }
}
