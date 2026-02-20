use crate::types::*;
use crate::Result;
use wasm_bindgen_futures::JsFuture;

pub(crate) mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type ColorbarApi;

        #[wasm_bindgen(method)]
        pub fn add(this: &ColorbarApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn remove(this: &ColorbarApi) -> ::js_sys::Promise;
    }
}

/// Add and remove a colorbar to the scene view for analysis heatmaps.
pub struct ColorbarApi {
    inner: js::ColorbarApi,
}

impl ColorbarApi {
    pub(crate) fn from_raw(raw: js::ColorbarApi) -> Self {
        Self { inner: raw }
    }

    /// Add a colorbar to the scene view. Only one colorbar at a time.
    pub async fn add(&self, request: &ColorbarAddRequest) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.add(&js_req)).await?;
        Ok(())
    }

    /// Remove the colorbar added by this embedded view.
    pub async fn remove(&self) -> Result<()> {
        JsFuture::from(self.inner.remove()).await?;
        Ok(())
    }
}
