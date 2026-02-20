use crate::types::*;
use crate::Result;
use wasm_bindgen_futures::JsFuture;

pub(crate) mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type AreaMetricsApi;

        #[wasm_bindgen(method)]
        pub fn calculate(this: &AreaMetricsApi, request: &JsValue) -> ::js_sys::Promise;
    }
}

/// Calculate area metrics / key figures for elements.
pub struct AreaMetricsApi {
    inner: js::AreaMetricsApi,
}

impl AreaMetricsApi {
    pub(crate) fn from_raw(raw: js::AreaMetricsApi) -> Self {
        Self { inner: raw }
    }

    /// Calculate area metrics for the given paths. If no paths are given,
    /// metrics are calculated for all elements.
    pub async fn calculate(
        &self,
        request: &AreaMetricsCalculateRequest,
    ) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.calculate(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }
}
