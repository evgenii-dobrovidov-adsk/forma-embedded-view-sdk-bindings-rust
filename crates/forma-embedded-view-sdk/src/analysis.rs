use crate::types::*;
use crate::Result;
use wasm_bindgen_futures::JsFuture;

pub(crate) mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type AnalysisApi;

        #[wasm_bindgen(method)]
        pub fn list(this: &AnalysisApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "triggerNoise")]
        pub fn trigger_noise(this: &AnalysisApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "triggerSun")]
        pub fn trigger_sun(this: &AnalysisApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getSunAnalysis")]
        pub fn get_sun_analysis(this: &AnalysisApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getNoiseAnalysis")]
        pub fn get_noise_analysis(this: &AnalysisApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getGroundGrid")]
        pub fn get_ground_grid(this: &AnalysisApi, request: &JsValue) -> ::js_sys::Promise;
    }
}

/// Interact with Forma's native analysis functionality (sun, noise, wind).
pub struct AnalysisApi {
    inner: js::AnalysisApi,
}

impl AnalysisApi {
    pub(crate) fn from_raw(raw: js::AnalysisApi) -> Self {
        Self { inner: raw }
    }

    /// Fetch analysis records connected to the currently open proposal.
    pub async fn list(&self, request: &AnalysisListRequest) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.list(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Trigger a noise analysis based on traffic data connected to roads and railways.
    pub async fn trigger_noise(&self, request: &TriggerNoiseRequest) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.trigger_noise(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Trigger a sun analysis for a specific day of the year.
    pub async fn trigger_sun(&self, request: &TriggerSunRequest) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.trigger_sun(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Fetch a specific sun analysis.
    pub async fn get_sun_analysis(
        &self,
        request: &GetAnalysisRequest,
    ) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.get_sun_analysis(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Fetch a specific noise analysis.
    pub async fn get_noise_analysis(
        &self,
        request: &GetAnalysisRequest,
    ) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.get_noise_analysis(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Fetch ground grid result for a sun or noise analysis.
    pub async fn get_ground_grid(
        &self,
        request: &GetGroundGridRequest,
    ) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.get_ground_grid(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }
}
