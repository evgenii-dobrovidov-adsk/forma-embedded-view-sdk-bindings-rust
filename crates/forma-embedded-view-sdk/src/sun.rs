use crate::types::*;
use crate::Result;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

pub(crate) mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type SunApi;

        #[wasm_bindgen(method, js_name = "getDate")]
        pub fn get_date(this: &SunApi) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "setDate")]
        pub fn set_date(this: &SunApi, request: &JsValue) -> ::js_sys::Promise;
    }
}

/// Interact with the sun object in the 3D scene.
pub struct SunApi {
    inner: js::SunApi,
}

impl SunApi {
    pub(crate) fn from_raw(raw: js::SunApi) -> Self {
        Self { inner: raw }
    }

    /// Fetch the ISO date string corresponding to the current sun position.
    pub async fn get_date(&self) -> Result<String> {
        let result = JsFuture::from(self.inner.get_date()).await?;
        let date: &::js_sys::Date = wasm_bindgen::JsCast::unchecked_ref(&result);
        Ok(date.to_iso_string().into())
    }

    /// Set the position of the sun in the scene by date (ISO 8601 string).
    pub async fn set_date(&self, request: &SunDateRequest) -> Result<()> {
        let date = ::js_sys::Date::new(&JsValue::from_str(&request.date));
        let obj = ::js_sys::Object::new();
        ::js_sys::Reflect::set(&obj, &"date".into(), &date)?;
        JsFuture::from(self.inner.set_date(&obj)).await?;
        Ok(())
    }
}
