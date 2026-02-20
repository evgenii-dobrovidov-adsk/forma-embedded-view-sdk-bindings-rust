use crate::types::*;
use crate::Result;
use wasm_bindgen_futures::JsFuture;

pub(crate) mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type PredictiveAnalysisApi;

        #[wasm_bindgen(method, js_name = "getWindParameters")]
        pub fn get_wind_parameters(this: &PredictiveAnalysisApi) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "predictWind")]
        pub fn predict_wind(
            this: &PredictiveAnalysisApi,
            request: &JsValue,
        ) -> ::js_sys::Promise;
    }
}

/// Interact with Forma's predictive models for rapid wind analysis.
pub struct PredictiveAnalysisApi {
    inner: js::PredictiveAnalysisApi,
}

impl PredictiveAnalysisApi {
    pub(crate) fn from_raw(raw: js::PredictiveAnalysisApi) -> Self {
        Self { inner: raw }
    }

    /// Get the wind parameters (wind rose + surface roughness) used by Forma.
    pub async fn get_wind_parameters(&self) -> Result<serde_json::Value> {
        let result = JsFuture::from(self.inner.get_wind_parameters()).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Predict wind conditions using Forma's rapid wind model.
    pub async fn predict_wind(
        &self,
        request: &PredictWindRequest,
    ) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.predict_wind(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }
}
