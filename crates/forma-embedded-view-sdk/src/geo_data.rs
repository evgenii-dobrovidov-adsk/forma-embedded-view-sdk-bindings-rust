use crate::types::*;
use crate::Result;
use wasm_bindgen_futures::JsFuture;

pub(crate) mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type GeoDataApi;

        #[wasm_bindgen(method)]
        pub fn upload(this: &GeoDataApi, request: &JsValue) -> ::js_sys::Promise;
    }
}

/// Upload buildings, roads, and property boundaries as GeoJSON.
pub struct GeoDataApi {
    inner: js::GeoDataApi,
}

impl GeoDataApi {
    pub(crate) fn from_raw(raw: js::GeoDataApi) -> Self {
        Self { inner: raw }
    }

    /// Upload GeoJSON data to Forma and add it to the library.
    /// Supports 2.5D buildings, roads, and property boundaries.
    pub async fn upload(&self, request: &GeoDataUploadRequest) -> Result<LibraryItem> {
        let js_request = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.upload(&js_request)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }
}
