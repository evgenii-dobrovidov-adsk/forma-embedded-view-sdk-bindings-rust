use crate::subscription::Subscription;
use crate::types::*;
use crate::Result;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

pub(crate) mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type CameraApi;

        #[wasm_bindgen(method, js_name = "move")]
        pub fn move_(this: &CameraApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "switchPerspective")]
        pub fn switch_perspective(this: &CameraApi) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn capture(this: &CameraApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getCurrent")]
        pub fn get_current(this: &CameraApi) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn subscribe(
            this: &CameraApi,
            callback: &Closure<dyn FnMut(JsValue)>,
        ) -> ::js_sys::Promise;
    }
}

/// Interact with the camera in the 3D scene.
pub struct CameraApi {
    inner: js::CameraApi,
}

impl CameraApi {
    pub(crate) fn from_raw(raw: js::CameraApi) -> Self {
        Self { inner: raw }
    }

    /// Move camera view to a new position.
    pub async fn move_to(&self, request: &CameraMoveRequest) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.move_(&js_req)).await?;
        Ok(())
    }

    /// Toggle between perspective and orthographic camera.
    pub async fn switch_perspective(&self) -> Result<()> {
        JsFuture::from(self.inner.switch_perspective()).await?;
        Ok(())
    }

    /// Capture a screenshot of the current camera view.
    /// Returns a `CaptureResult` wrapping the canvas, with methods like `to_data_url()`.
    pub async fn capture(&self, request: &CameraCaptureRequest) -> Result<CaptureResult> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.capture(&js_req)).await?;
        Ok(CaptureResult { inner: result })
    }

    /// Fetch the current camera state.
    pub async fn get_current(&self) -> Result<CameraState> {
        let result = JsFuture::from(self.inner.get_current()).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Subscribe to camera changes.
    pub async fn subscribe(
        &self,
        mut callback: impl FnMut(CameraState) + 'static,
    ) -> Result<Subscription> {
        let closure = Closure::wrap(Box::new(move |val: JsValue| {
            if let Ok(state) = serde_wasm_bindgen::from_value(val) {
                callback(state);
            }
        }) as Box<dyn FnMut(JsValue)>);
        let result = JsFuture::from(self.inner.subscribe(&closure)).await?;
        let unsubscribe_fn: ::js_sys::Function =
            ::js_sys::Reflect::get(&result, &"unsubscribe".into())?.into();
        Ok(Subscription::new(closure, unsubscribe_fn))
    }
}
