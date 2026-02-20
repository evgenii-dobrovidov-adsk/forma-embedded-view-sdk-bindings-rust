#![doc = include_str!("../README.md")]

pub mod analysis;
pub mod area_metrics;
pub mod auth;
pub mod camera;
pub mod colorbar;
pub mod design_tool;
pub mod elements;
pub mod error;
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
pub mod types;

mod subscription;
pub use subscription::{Subscription, VoidSubscription};

pub use error::SdkError;
pub type Result<T> = std::result::Result<T, SdkError>;

use types::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

pub(crate) mod js {
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
        pub fn analysis(this: &EmbeddedViewSdk) -> crate::analysis::js::AnalysisApi;

        #[wasm_bindgen(method, getter)]
        pub fn extensions(this: &EmbeddedViewSdk) -> crate::extensions::js::ExtensionsApi;

        #[wasm_bindgen(method, getter)]
        pub fn elements(this: &EmbeddedViewSdk) -> crate::elements::js::ElementsApi;

        #[wasm_bindgen(method, getter)]
        pub fn generators(this: &EmbeddedViewSdk) -> crate::generators::js::GeneratorsApi;

        #[wasm_bindgen(method, getter, js_name = "geometry")]
        pub fn geometry_api(this: &EmbeddedViewSdk) -> crate::geometry::js::GeometryApi;

        #[wasm_bindgen(method, getter, js_name = "integrateElements")]
        pub fn integrate_elements(this: &EmbeddedViewSdk) -> crate::integrate::js::IntegrateApi;

        #[wasm_bindgen(method, getter)]
        pub fn library(this: &EmbeddedViewSdk) -> crate::library::js::LibraryApi;

        #[wasm_bindgen(method, getter)]
        pub fn project(this: &EmbeddedViewSdk) -> crate::project::js::ProjectApi;

        #[wasm_bindgen(method, getter)]
        pub fn proposal(this: &EmbeddedViewSdk) -> crate::proposal::js::ProposalApi;

        #[wasm_bindgen(method, getter)]
        pub fn camera(this: &EmbeddedViewSdk) -> crate::camera::js::CameraApi;

        #[wasm_bindgen(method, getter)]
        pub fn sun(this: &EmbeddedViewSdk) -> crate::sun::js::SunApi;

        #[wasm_bindgen(method, getter)]
        pub fn terrain(this: &EmbeddedViewSdk) -> crate::terrain::js::TerrainApi;

        #[wasm_bindgen(method, getter)]
        pub fn render(this: &EmbeddedViewSdk) -> crate::render::js::RenderApi;

        #[wasm_bindgen(method, getter)]
        pub fn selection(this: &EmbeddedViewSdk) -> crate::selection::js::SelectionApi;

        #[wasm_bindgen(method, getter, js_name = "areaMetrics")]
        pub fn area_metrics(this: &EmbeddedViewSdk) -> crate::area_metrics::js::AreaMetricsApi;

        #[wasm_bindgen(method, getter, js_name = "predictiveAnalysis")]
        pub fn predictive_analysis(
            this: &EmbeddedViewSdk,
        ) -> crate::predictive_analysis::js::PredictiveAnalysisApi;

        #[wasm_bindgen(method, getter, js_name = "designTool")]
        pub fn design_tool(this: &EmbeddedViewSdk) -> crate::design_tool::js::DesignToolApi;

        #[wasm_bindgen(method, getter)]
        pub fn auth(this: &EmbeddedViewSdk) -> crate::auth::js::AuthApi;

        #[wasm_bindgen(method, getter)]
        pub fn colorbar(this: &EmbeddedViewSdk) -> crate::colorbar::js::ColorbarApi;

        #[wasm_bindgen(method, getter, js_name = "geoData")]
        pub fn geo_data(this: &EmbeddedViewSdk) -> crate::geo_data::js::GeoDataApi;

        #[wasm_bindgen(method)]
        pub fn ping(this: &EmbeddedViewSdk) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getProjectId")]
        pub fn get_project_id(this: &EmbeddedViewSdk) -> String;

        #[wasm_bindgen(method, js_name = "getExtensionId")]
        pub fn get_extension_id(this: &EmbeddedViewSdk) -> String;

        #[wasm_bindgen(method, js_name = "getRegion")]
        pub fn get_region(this: &EmbeddedViewSdk) -> String;

        #[wasm_bindgen(method, js_name = "getPresentationUnitSystem")]
        pub fn get_presentation_unit_system(this: &EmbeddedViewSdk) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getCanEdit")]
        pub fn get_can_edit(this: &EmbeddedViewSdk) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getCanViewHub")]
        pub fn get_can_view_hub(this: &EmbeddedViewSdk) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getCanEditHub")]
        pub fn get_can_edit_hub(this: &EmbeddedViewSdk) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getEmbeddedViewId")]
        pub fn get_embedded_view_id(this: &EmbeddedViewSdk) -> String;

        #[wasm_bindgen(method, js_name = "openFloatingPanel")]
        pub fn open_floating_panel(
            this: &EmbeddedViewSdk,
            options: &JsValue,
        ) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "closeEmbeddedView")]
        pub fn close_embedded_view(
            this: &EmbeddedViewSdk,
            options: &JsValue,
        ) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "onEmbeddedViewStateChange")]
        pub fn on_embedded_view_state_change(
            this: &EmbeddedViewSdk,
            handler: &Closure<dyn FnMut(JsValue)>,
        ) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "onLocaleUpdate")]
        pub fn on_locale_update(
            this: &EmbeddedViewSdk,
            handler: &Closure<dyn FnMut(JsValue)>,
        ) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "onEmbeddedViewClosing")]
        pub fn on_embedded_view_closing(
            this: &EmbeddedViewSdk,
            handler: &Closure<dyn FnMut(JsValue) -> ::js_sys::Promise>,
        ) -> crate::subscription::js::SubscriptionResult;

        #[wasm_bindgen(method, js_name = "createMessagePort")]
        pub fn create_message_port(
            this: &EmbeddedViewSdk,
            options: &JsValue,
        ) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "onMessagePort")]
        pub fn on_message_port(
            this: &EmbeddedViewSdk,
            handler: &Closure<dyn FnMut(JsValue)>,
        ) -> ::js_sys::Function;

        #[wasm_bindgen(static_method_of = EmbeddedViewSdk, js_class = "EmbeddedViewSdk", js_name = "getHostOrigin")]
        pub fn get_host_origin() -> String;
    }

    #[wasm_bindgen(module = "forma-embedded-view-sdk/auto")]
    extern "C" {
        #[wasm_bindgen(thread_local_v2, js_name = "Forma")]
        pub static FORMA: EmbeddedViewSdk;
    }
}

/// The main Forma Embedded View SDK entry point.
pub struct EmbeddedViewSdk {
    inner: js::EmbeddedViewSdk,
}

impl EmbeddedViewSdk {
    pub fn new(config: Option<&SdkConfig>) -> Result<Self> {
        let js_config = config
            .map(serde_wasm_bindgen::to_value)
            .transpose()?;
        Ok(Self {
            inner: js::EmbeddedViewSdk::new(js_config.as_ref()),
        })
    }

    pub(crate) fn from_raw(raw: js::EmbeddedViewSdk) -> Self {
        Self { inner: raw }
    }

    pub fn origin(&self) -> String {
        self.inner.origin()
    }

    pub fn get_project_id(&self) -> String {
        self.inner.get_project_id()
    }

    pub fn get_extension_id(&self) -> String {
        self.inner.get_extension_id()
    }

    pub fn get_region(&self) -> String {
        self.inner.get_region()
    }

    pub fn get_embedded_view_id(&self) -> String {
        self.inner.get_embedded_view_id()
    }

    pub fn get_host_origin() -> String {
        js::EmbeddedViewSdk::get_host_origin()
    }

    pub async fn ping(&self) -> Result<()> {
        JsFuture::from(self.inner.ping()).await?;
        Ok(())
    }

    pub async fn get_presentation_unit_system(&self) -> Result<UnitSystem> {
        let result = JsFuture::from(self.inner.get_presentation_unit_system()).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    pub async fn get_can_edit(&self) -> Result<bool> {
        let result = JsFuture::from(self.inner.get_can_edit()).await?;
        Ok(result.as_bool().unwrap_or(false))
    }

    pub async fn get_can_view_hub(&self) -> Result<bool> {
        let result = JsFuture::from(self.inner.get_can_view_hub()).await?;
        Ok(result.as_bool().unwrap_or(false))
    }

    pub async fn get_can_edit_hub(&self) -> Result<bool> {
        let result = JsFuture::from(self.inner.get_can_edit_hub()).await?;
        Ok(result.as_bool().unwrap_or(false))
    }

    pub async fn open_floating_panel(&self, options: &FloatingPanelOptions) -> Result<()> {
        let js_options = serde_wasm_bindgen::to_value(options)?;
        JsFuture::from(self.inner.open_floating_panel(&js_options)).await?;
        Ok(())
    }

    pub async fn close_embedded_view(&self, options: &CloseEmbeddedViewOptions) -> Result<()> {
        let js_options = serde_wasm_bindgen::to_value(options)?;
        JsFuture::from(self.inner.close_embedded_view(&js_options)).await?;
        Ok(())
    }

    pub async fn on_embedded_view_state_change(
        &self,
        mut callback: impl FnMut(EmbeddedViewState) + 'static,
    ) -> Result<Subscription> {
        let closure = Closure::wrap(Box::new(move |val: JsValue| {
            if let Ok(state) = serde_wasm_bindgen::from_value(val) {
                callback(state);
            }
        }) as Box<dyn FnMut(JsValue)>);
        let result = JsFuture::from(self.inner.on_embedded_view_state_change(&closure)).await?;
        let unsubscribe_fn: ::js_sys::Function = ::js_sys::Reflect::get(&result, &"unsubscribe".into())?.into();
        Ok(Subscription::new(closure, unsubscribe_fn))
    }

    pub async fn on_locale_update(
        &self,
        mut callback: impl FnMut(LocaleUpdate) + 'static,
    ) -> Result<Subscription> {
        let closure = Closure::wrap(Box::new(move |val: JsValue| {
            if let Ok(update) = serde_wasm_bindgen::from_value(val) {
                callback(update);
            }
        }) as Box<dyn FnMut(JsValue)>);
        let result = JsFuture::from(self.inner.on_locale_update(&closure)).await?;
        let unsubscribe_fn: ::js_sys::Function = ::js_sys::Reflect::get(&result, &"unsubscribe".into())?.into();
        Ok(Subscription::new(closure, unsubscribe_fn))
    }

    pub async fn create_message_port(
        &self,
        options: &CreateMessagePortOptions,
    ) -> Result<MessagePortHandle> {
        let js_options = serde_wasm_bindgen::to_value(options)?;
        let result = JsFuture::from(self.inner.create_message_port(&js_options)).await?;
        Ok(MessagePortHandle { inner: result })
    }

    pub fn on_message_port(
        &self,
        mut callback: impl FnMut(serde_json::Value) + 'static,
    ) -> Subscription {
        let closure = Closure::wrap(Box::new(move |val: JsValue| {
            if let Ok(v) = serde_wasm_bindgen::from_value(val) {
                callback(v);
            }
        }) as Box<dyn FnMut(JsValue)>);
        let unsubscribe_fn = self.inner.on_message_port(&closure);
        Subscription::new(closure, unsubscribe_fn)
    }

    // ---- Sub-API accessors ----

    pub fn analysis(&self) -> analysis::AnalysisApi {
        analysis::AnalysisApi::from_raw(self.inner.analysis())
    }

    pub fn area_metrics(&self) -> area_metrics::AreaMetricsApi {
        area_metrics::AreaMetricsApi::from_raw(self.inner.area_metrics())
    }

    pub fn auth(&self) -> auth::AuthApi {
        auth::AuthApi::from_raw(self.inner.auth())
    }

    pub fn camera(&self) -> camera::CameraApi {
        camera::CameraApi::from_raw(self.inner.camera())
    }

    pub fn colorbar(&self) -> colorbar::ColorbarApi {
        colorbar::ColorbarApi::from_raw(self.inner.colorbar())
    }

    pub fn design_tool(&self) -> design_tool::DesignToolApi {
        design_tool::DesignToolApi::from_raw(self.inner.design_tool())
    }

    pub fn elements(&self) -> elements::ElementsApi {
        elements::ElementsApi::from_raw(self.inner.elements())
    }

    pub fn extensions(&self) -> extensions::ExtensionsApi {
        extensions::ExtensionsApi::from_raw(self.inner.extensions())
    }

    pub fn generators(&self) -> generators::GeneratorsApi {
        generators::GeneratorsApi::from_raw(self.inner.generators())
    }

    pub fn geometry(&self) -> geometry::GeometryApi {
        geometry::GeometryApi::from_raw(self.inner.geometry_api())
    }

    pub fn integrate_elements(&self) -> integrate::IntegrateApi {
        integrate::IntegrateApi::from_raw(self.inner.integrate_elements())
    }

    pub fn library(&self) -> library::LibraryApi {
        library::LibraryApi::from_raw(self.inner.library())
    }

    pub fn project(&self) -> project::ProjectApi {
        project::ProjectApi::from_raw(self.inner.project())
    }

    pub fn proposal(&self) -> proposal::ProposalApi {
        proposal::ProposalApi::from_raw(self.inner.proposal())
    }

    pub fn render(&self) -> render::RenderApi {
        render::RenderApi::from_raw(self.inner.render())
    }

    pub fn selection(&self) -> selection::SelectionApi {
        selection::SelectionApi::from_raw(self.inner.selection())
    }

    pub fn sun(&self) -> sun::SunApi {
        sun::SunApi::from_raw(self.inner.sun())
    }

    pub fn terrain(&self) -> terrain::TerrainApi {
        terrain::TerrainApi::from_raw(self.inner.terrain())
    }

    pub fn geo_data(&self) -> geo_data::GeoDataApi {
        geo_data::GeoDataApi::from_raw(self.inner.geo_data())
    }

    pub fn predictive_analysis(&self) -> predictive_analysis::PredictiveAnalysisApi {
        predictive_analysis::PredictiveAnalysisApi::from_raw(self.inner.predictive_analysis())
    }
}

/// Pre-configured singleton SDK instance (from `forma-embedded-view-sdk/auto`).
pub fn forma() -> EmbeddedViewSdk {
    js::FORMA.with(|f| {
        let js_val: &JsValue = f.as_ref();
        let cloned: js::EmbeddedViewSdk = js_val.clone().unchecked_into();
        EmbeddedViewSdk::from_raw(cloned)
    })
}
