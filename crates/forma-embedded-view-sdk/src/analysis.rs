use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Interact with Forma's native analysis functionality (sun, noise, wind).
    pub type AnalysisApi;

    /// Fetch analysis records connected to the currently open proposal.
    #[wasm_bindgen(method)]
    pub fn list(this: &AnalysisApi, request: &JsValue) -> js_sys::Promise;

    /// Trigger a noise analysis based on traffic data connected to roads and railways.
    #[wasm_bindgen(method, js_name = "triggerNoise")]
    pub fn trigger_noise(this: &AnalysisApi, request: &JsValue) -> js_sys::Promise;

    /// Trigger a sun analysis for a specific day of the year.
    #[wasm_bindgen(method, js_name = "triggerSun")]
    pub fn trigger_sun(this: &AnalysisApi, request: &JsValue) -> js_sys::Promise;

    /// Fetch a specific sun analysis.
    #[wasm_bindgen(method, js_name = "getSunAnalysis")]
    pub fn get_sun_analysis(this: &AnalysisApi, request: &JsValue) -> js_sys::Promise;

    /// Fetch a specific noise analysis.
    #[wasm_bindgen(method, js_name = "getNoiseAnalysis")]
    pub fn get_noise_analysis(this: &AnalysisApi, request: &JsValue) -> js_sys::Promise;

    /// Fetch ground grid result for a sun or noise analysis.
    #[wasm_bindgen(method, js_name = "getGroundGrid")]
    pub fn get_ground_grid(this: &AnalysisApi, request: &JsValue) -> js_sys::Promise;
}
