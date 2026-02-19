#![doc = include_str!("../README.md")]

pub mod analysis;
pub mod area_metrics;
pub mod auth;
pub mod camera;
pub mod colorbar;
pub mod design_tool;
pub mod elements;
pub mod extensions;
pub mod generators;
pub mod geo_data;
pub mod geometry;
pub mod integrate;
pub mod library;
pub mod predictive_analysis;
pub mod project;
pub mod proposal;
pub mod render;
pub mod selection;
pub mod sun;
pub mod terrain;

mod subscription;
pub use subscription::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "forma-embedded-view-sdk")]
extern "C" {
    #[wasm_bindgen(js_name = EmbeddedViewSdk)]
    pub type EmbeddedViewSdk;

    #[wasm_bindgen(constructor, js_class = "EmbeddedViewSdk")]
    pub fn new(config: Option<&JsValue>) -> EmbeddedViewSdk;

    #[wasm_bindgen(method, getter)]
    pub fn origin(this: &EmbeddedViewSdk) -> String;

    #[wasm_bindgen(method, getter)]
    pub fn analysis(this: &EmbeddedViewSdk) -> analysis::AnalysisApi;

    #[wasm_bindgen(method, getter)]
    pub fn extensions(this: &EmbeddedViewSdk) -> extensions::ExtensionsApi;

    #[wasm_bindgen(method, getter)]
    pub fn elements(this: &EmbeddedViewSdk) -> elements::ElementsApi;

    #[wasm_bindgen(method, getter)]
    pub fn generators(this: &EmbeddedViewSdk) -> generators::GeneratorsApi;

    #[wasm_bindgen(method, getter, js_name = "geometry")]
    pub fn geometry_api(this: &EmbeddedViewSdk) -> geometry::GeometryApi;

    #[wasm_bindgen(method, getter, js_name = "integrateElements")]
    pub fn integrate_elements(this: &EmbeddedViewSdk) -> integrate::IntegrateApi;

    #[wasm_bindgen(method, getter)]
    pub fn library(this: &EmbeddedViewSdk) -> library::LibraryApi;

    #[wasm_bindgen(method, getter)]
    pub fn project(this: &EmbeddedViewSdk) -> project::ProjectApi;

    #[wasm_bindgen(method, getter)]
    pub fn proposal(this: &EmbeddedViewSdk) -> proposal::ProposalApi;

    #[wasm_bindgen(method, getter)]
    pub fn camera(this: &EmbeddedViewSdk) -> camera::CameraApi;

    #[wasm_bindgen(method, getter)]
    pub fn sun(this: &EmbeddedViewSdk) -> sun::SunApi;

    #[wasm_bindgen(method, getter)]
    pub fn terrain(this: &EmbeddedViewSdk) -> terrain::TerrainApi;

    #[wasm_bindgen(method, getter)]
    pub fn render(this: &EmbeddedViewSdk) -> render::RenderApi;

    #[wasm_bindgen(method, getter)]
    pub fn selection(this: &EmbeddedViewSdk) -> selection::SelectionApi;

    #[wasm_bindgen(method, getter, js_name = "areaMetrics")]
    pub fn area_metrics(this: &EmbeddedViewSdk) -> area_metrics::AreaMetricsApi;

    #[wasm_bindgen(method, getter, js_name = "predictiveAnalysis")]
    pub fn predictive_analysis(
        this: &EmbeddedViewSdk,
    ) -> predictive_analysis::PredictiveAnalysisApi;

    #[wasm_bindgen(method, getter, js_name = "designTool")]
    pub fn design_tool(this: &EmbeddedViewSdk) -> design_tool::DesignToolApi;

    #[wasm_bindgen(method, getter)]
    pub fn auth(this: &EmbeddedViewSdk) -> auth::AuthApi;

    #[wasm_bindgen(method, getter)]
    pub fn colorbar(this: &EmbeddedViewSdk) -> colorbar::ColorbarApi;

    #[wasm_bindgen(method, getter, js_name = "geoData")]
    pub fn geo_data(this: &EmbeddedViewSdk) -> geo_data::GeoDataApi;

    // --- Methods ---

    #[wasm_bindgen(method)]
    pub fn ping(this: &EmbeddedViewSdk) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "getProjectId")]
    pub fn get_project_id(this: &EmbeddedViewSdk) -> String;

    #[wasm_bindgen(method, js_name = "getExtensionId")]
    pub fn get_extension_id(this: &EmbeddedViewSdk) -> String;

    #[wasm_bindgen(method, js_name = "getRegion")]
    pub fn get_region(this: &EmbeddedViewSdk) -> String;

    #[wasm_bindgen(method, js_name = "getPresentationUnitSystem")]
    pub fn get_presentation_unit_system(this: &EmbeddedViewSdk) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "getCanEdit")]
    pub fn get_can_edit(this: &EmbeddedViewSdk) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "getCanViewHub")]
    pub fn get_can_view_hub(this: &EmbeddedViewSdk) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "getCanEditHub")]
    pub fn get_can_edit_hub(this: &EmbeddedViewSdk) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "getEmbeddedViewId")]
    pub fn get_embedded_view_id(this: &EmbeddedViewSdk) -> String;

    #[wasm_bindgen(method, js_name = "openFloatingPanel")]
    pub fn open_floating_panel(this: &EmbeddedViewSdk, options: &JsValue) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "closeEmbeddedView")]
    pub fn close_embedded_view(this: &EmbeddedViewSdk, options: &JsValue) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "onEmbeddedViewStateChange")]
    pub fn on_embedded_view_state_change(
        this: &EmbeddedViewSdk,
        handler: &Closure<dyn FnMut(JsValue)>,
    ) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "onLocaleUpdate")]
    pub fn on_locale_update(
        this: &EmbeddedViewSdk,
        handler: &Closure<dyn FnMut(JsValue)>,
    ) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "onEmbeddedViewClosing")]
    pub fn on_embedded_view_closing(
        this: &EmbeddedViewSdk,
        handler: &Closure<dyn FnMut(JsValue) -> js_sys::Promise>,
    ) -> SubscriptionResult;

    #[wasm_bindgen(method, js_name = "createMessagePort")]
    pub fn create_message_port(this: &EmbeddedViewSdk, options: &JsValue) -> js_sys::Promise;

    #[wasm_bindgen(method, js_name = "onMessagePort")]
    pub fn on_message_port(
        this: &EmbeddedViewSdk,
        handler: &Closure<dyn FnMut(JsValue)>,
    ) -> js_sys::Function;

    #[wasm_bindgen(static_method_of = EmbeddedViewSdk, js_class = "EmbeddedViewSdk", js_name = "getHostOrigin")]
    pub fn get_host_origin() -> String;
}

#[wasm_bindgen(module = "forma-embedded-view-sdk/auto")]
extern "C" {
    #[wasm_bindgen(thread_local_v2, js_name = "Forma")]
    pub static FORMA: EmbeddedViewSdk;
}
