use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Interact with Forma's predictive models for rapid wind analysis.
    pub type PredictiveAnalysisApi;

    /// Get the wind parameters (wind rose + surface roughness) used by Forma.
    #[wasm_bindgen(method, js_name = "getWindParameters")]
    pub fn get_wind_parameters(this: &PredictiveAnalysisApi) -> js_sys::Promise;

    /// Predict wind conditions using Forma's rapid wind model.
    /// Resolves to `PredictiveAnalysisGroundGrid`.
    #[wasm_bindgen(method, js_name = "predictWind")]
    pub fn predict_wind(this: &PredictiveAnalysisApi, request: &JsValue) -> js_sys::Promise;
}
