use crate::types::*;
use crate::Result;
use wasm_bindgen_futures::JsFuture;

pub(crate) mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type GeneratorsApi;

        #[wasm_bindgen(method)]
        pub fn put(this: &GeneratorsApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn list(this: &GeneratorsApi, request: Option<&JsValue>) -> ::js_sys::Promise;
    }
}

/// Manage generators registered in Forma.
pub struct GeneratorsApi {
    inner: js::GeneratorsApi,
}

impl GeneratorsApi {
    pub(crate) fn from_raw(raw: js::GeneratorsApi) -> Self {
        Self { inner: raw }
    }

    /// Create or replace a generator.
    pub async fn put(&self, request: &GeneratorPutRequest) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.put(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// List generators within the specified authcontext.
    pub async fn list(
        &self,
        request: Option<&GeneratorListRequest>,
    ) -> Result<serde_json::Value> {
        let js_req = request
            .map(serde_wasm_bindgen::to_value)
            .transpose()?;
        let result = JsFuture::from(self.inner.list(js_req.as_ref())).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }
}
