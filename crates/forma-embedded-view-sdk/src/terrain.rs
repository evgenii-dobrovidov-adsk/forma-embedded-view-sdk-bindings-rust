use crate::types::*;
use crate::Result;
use wasm_bindgen_futures::JsFuture;

pub(crate) mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type TerrainApi;

        #[wasm_bindgen(method, getter, js_name = "groundTexture")]
        pub fn ground_texture(this: &TerrainApi) -> GroundTextureApi;

        #[wasm_bindgen(method, js_name = "getBbox")]
        pub fn get_bbox(this: &TerrainApi) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getElevationAt")]
        pub fn get_elevation_at(this: &TerrainApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getPads")]
        pub fn get_pads(this: &TerrainApi) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "addPads")]
        pub fn add_pads(this: &TerrainApi, pads: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "applyPads")]
        pub fn apply_pads(this: &TerrainApi, pads: &JsValue) -> ::js_sys::Promise;
    }

    #[wasm_bindgen]
    extern "C" {
        pub type GroundTextureApi;

        #[wasm_bindgen(method)]
        pub fn add(this: &GroundTextureApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "updateTextureData")]
        pub fn update_texture_data(
            this: &GroundTextureApi,
            request: &JsValue,
        ) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "updatePosition")]
        pub fn update_position(this: &GroundTextureApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn remove(this: &GroundTextureApi, request: &JsValue) -> ::js_sys::Promise;
    }
}

/// Interact with the terrain in the 3D scene.
pub struct TerrainApi {
    inner: js::TerrainApi,
}

impl TerrainApi {
    pub(crate) fn from_raw(raw: js::TerrainApi) -> Self {
        Self { inner: raw }
    }

    /// Access the ground texture sub-API.
    pub fn ground_texture(&self) -> GroundTextureApi {
        GroundTextureApi {
            inner: self.inner.ground_texture(),
        }
    }

    /// Fetch the bounding box for the terrain.
    pub async fn get_bbox(&self) -> Result<TerrainBbox> {
        let result = JsFuture::from(self.inner.get_bbox()).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Get the elevation at a specific (x, y) point. Returns meters above sea level.
    pub async fn get_elevation_at(&self, request: &ElevationRequest) -> Result<f64> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.get_elevation_at(&js_req)).await?;
        Ok(result.as_f64().unwrap_or(0.0))
    }

    /// Retrieve all terrain pads defined in the current terrain.
    pub async fn get_pads(&self) -> Result<Vec<TerrainPad>> {
        let result = JsFuture::from(self.inner.get_pads()).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Add new terrain pads to the existing pads.
    pub async fn add_pads(&self, pads: &[TerrainPad]) -> Result<()> {
        let js_pads = serde_wasm_bindgen::to_value(pads)?;
        JsFuture::from(self.inner.add_pads(&js_pads)).await?;
        Ok(())
    }

    /// Replace all existing terrain pads with the provided array.
    pub async fn apply_pads(&self, pads: &[TerrainPad]) -> Result<()> {
        let js_pads = serde_wasm_bindgen::to_value(pads)?;
        JsFuture::from(self.inner.apply_pads(&js_pads)).await?;
        Ok(())
    }
}

/// Manage ground textures applied to the terrain in the 3D scene.
pub struct GroundTextureApi {
    inner: js::GroundTextureApi,
}

impl GroundTextureApi {
    /// Add a ground texture to the terrain.
    pub async fn add(&self, request: &GroundTextureAddRequest) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.add(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Update the texture data for an existing ground texture.
    pub async fn update_texture_data(
        &self,
        request: &GroundTextureUpdateDataRequest,
    ) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.update_texture_data(&js_req)).await?;
        Ok(())
    }

    /// Update the placement of an existing ground texture.
    pub async fn update_position(
        &self,
        request: &GroundTextureUpdatePositionRequest,
    ) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.update_position(&js_req)).await?;
        Ok(())
    }

    /// Remove an existing ground texture.
    pub async fn remove(&self, request: &GroundTextureRemoveRequest) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.remove(&js_req)).await?;
        Ok(())
    }
}
