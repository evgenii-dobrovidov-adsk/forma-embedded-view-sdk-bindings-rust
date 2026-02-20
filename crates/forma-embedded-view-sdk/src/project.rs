use crate::types::*;
use crate::Result;
use wasm_bindgen_futures::JsFuture;

pub(crate) mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type ProjectApi;

        #[wasm_bindgen(method)]
        pub fn get(this: &ProjectApi) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getGeoLocation")]
        pub fn get_geo_location(this: &ProjectApi) -> ::js_sys::Promise;
    }
}

/// Access project-level metadata.
pub struct ProjectApi {
    inner: js::ProjectApi,
}

impl ProjectApi {
    pub(crate) fn from_raw(raw: js::ProjectApi) -> Self {
        Self { inner: raw }
    }

    /// Fetch all project metadata.
    pub async fn get(&self) -> Result<Project> {
        let result = JsFuture::from(self.inner.get()).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Fetch project location as `[latitude, longitude]`, or `None` if not set.
    pub async fn get_geo_location(&self) -> Result<Option<[f64; 2]>> {
        let result = JsFuture::from(self.inner.get_geo_location()).await?;
        if result.is_undefined() || result.is_null() {
            return Ok(None);
        }
        Ok(Some(serde_wasm_bindgen::from_value(result)?))
    }
}
