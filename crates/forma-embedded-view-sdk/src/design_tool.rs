use crate::subscription::VoidSubscription;
use crate::types::*;
use crate::Result;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

pub(crate) mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type DesignToolApi;

        #[wasm_bindgen(method, js_name = "getPoint")]
        pub fn get_point(this: &DesignToolApi) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getPolygon")]
        pub fn get_polygon(this: &DesignToolApi) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getExtrudedPolygon")]
        pub fn get_extruded_polygon(this: &DesignToolApi) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getLine")]
        pub fn get_line(this: &DesignToolApi) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "onEditStart")]
        pub fn on_edit_start(
            this: &DesignToolApi,
            callback: &Closure<dyn FnMut()>,
        ) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "onEditEnd")]
        pub fn on_edit_end(
            this: &DesignToolApi,
            callback: &Closure<dyn FnMut()>,
        ) -> ::js_sys::Promise;
    }
}

/// Enable design tools supplied by the host app (point, polygon, line picking).
pub struct DesignToolApi {
    inner: js::DesignToolApi,
}

impl DesignToolApi {
    pub(crate) fn from_raw(raw: js::DesignToolApi) -> Self {
        Self { inner: raw }
    }

    /// Activate tool for creating a point by clicking in the 3D scene.
    /// Returns `None` if the user cancelled.
    pub async fn get_point(&self) -> Result<Option<Vec3>> {
        let result = JsFuture::from(self.inner.get_point()).await?;
        if result.is_undefined() || result.is_null() {
            return Ok(None);
        }
        Ok(Some(serde_wasm_bindgen::from_value(result)?))
    }

    /// Activate tool for creating a polygon.
    /// Returns `None` if the user cancelled.
    pub async fn get_polygon(&self) -> Result<Option<Vec<Vec3>>> {
        let result = JsFuture::from(self.inner.get_polygon()).await?;
        if result.is_undefined() || result.is_null() {
            return Ok(None);
        }
        Ok(Some(serde_wasm_bindgen::from_value(result)?))
    }

    /// Activate tool for creating an extruded polygon.
    /// Returns `None` if the user cancelled.
    pub async fn get_extruded_polygon(&self) -> Result<Option<ExtrudedPolygon>> {
        let result = JsFuture::from(self.inner.get_extruded_polygon()).await?;
        if result.is_undefined() || result.is_null() {
            return Ok(None);
        }
        Ok(Some(serde_wasm_bindgen::from_value(result)?))
    }

    /// Activate tool for creating a line.
    /// Returns `None` if the user cancelled.
    pub async fn get_line(&self) -> Result<Option<Line>> {
        let result = JsFuture::from(self.inner.get_line()).await?;
        if result.is_undefined() || result.is_null() {
            return Ok(None);
        }
        Ok(Some(serde_wasm_bindgen::from_value(result)?))
    }

    /// Subscribe to the 'start' event for edits with the drawing tools.
    pub async fn on_edit_start(
        &self,
        mut callback: impl FnMut() + 'static,
    ) -> Result<VoidSubscription> {
        let closure = Closure::wrap(Box::new(move || {
            callback();
        }) as Box<dyn FnMut()>);
        let result = JsFuture::from(self.inner.on_edit_start(&closure)).await?;
        let unsubscribe_fn: ::js_sys::Function =
            ::js_sys::Reflect::get(&result, &"unsubscribe".into())?.into();
        Ok(VoidSubscription::new(closure, unsubscribe_fn))
    }

    /// Subscribe to the 'end' event for edits with the drawing tools.
    pub async fn on_edit_end(
        &self,
        mut callback: impl FnMut() + 'static,
    ) -> Result<VoidSubscription> {
        let closure = Closure::wrap(Box::new(move || {
            callback();
        }) as Box<dyn FnMut()>);
        let result = JsFuture::from(self.inner.on_edit_end(&closure)).await?;
        let unsubscribe_fn: ::js_sys::Function =
            ::js_sys::Reflect::get(&result, &"unsubscribe".into())?.into();
        Ok(VoidSubscription::new(closure, unsubscribe_fn))
    }
}
