use crate::types::*;
use crate::Result;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

fn mesh_request_to_js(request: &MeshRequest) -> std::result::Result<JsValue, JsValue> {
    let obj = ::js_sys::Object::new();
    ::js_sys::Reflect::set(&obj, &"id".into(), &request.id.clone().into())?;

    let geom = ::js_sys::Object::new();
    let position = ::js_sys::Float32Array::new_with_length(request.geometry_data.position.len() as u32);
    position.copy_from(&request.geometry_data.position);
    ::js_sys::Reflect::set(&geom, &"position".into(), &position)?;

    if let Some(ref color) = request.geometry_data.color {
        let color_arr = ::js_sys::Uint8Array::new_with_length(color.len() as u32);
        color_arr.copy_from(color);
        ::js_sys::Reflect::set(&geom, &"color".into(), &color_arr)?;
    }

    ::js_sys::Reflect::set(&obj, &"geometryData".into(), &geom)?;

    if let Some(ref transform) = request.transform {
        let t = serde_wasm_bindgen::to_value(transform)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        ::js_sys::Reflect::set(&obj, &"transform".into(), &t)?;
    }

    Ok(obj.into())
}

fn glb_request_to_js(request: &GlbRenderRequest) -> std::result::Result<JsValue, JsValue> {
    let obj = ::js_sys::Object::new();
    ::js_sys::Reflect::set(&obj, &"id".into(), &request.id.clone().into())?;

    let buf = ::js_sys::Uint8Array::new_with_length(request.glb.len() as u32);
    buf.copy_from(&request.glb);
    ::js_sys::Reflect::set(&obj, &"glb".into(), &buf.buffer())?;

    if let Some(ref transform) = request.transform {
        let t = serde_wasm_bindgen::to_value(transform)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
        ::js_sys::Reflect::set(&obj, &"transform".into(), &t)?;
    }

    Ok(obj.into())
}

pub(crate) mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type RenderApi;

        #[wasm_bindgen(method, getter)]
        pub fn glb(this: &RenderApi) -> RenderGlbApi;

        #[wasm_bindgen(method, getter)]
        pub fn geojson(this: &RenderApi) -> RenderGeojsonApi;

        #[wasm_bindgen(method, getter, js_name = "elementColors")]
        pub fn element_colors(this: &RenderApi) -> ElementColorApi;

        #[wasm_bindgen(method, js_name = "hideElement")]
        pub fn hide_element(this: &RenderApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "hideElementsBatch")]
        pub fn hide_elements_batch(this: &RenderApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "unhideElement")]
        pub fn unhide_element(this: &RenderApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "unhideElementsBatch")]
        pub fn unhide_elements_batch(this: &RenderApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "setElementsVisibility")]
        pub fn set_elements_visibility(this: &RenderApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "unhideAllElements")]
        pub fn unhide_all_elements(this: &RenderApi) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "addMesh")]
        pub fn add_mesh(this: &RenderApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "updateMesh")]
        pub fn update_mesh(this: &RenderApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn remove(this: &RenderApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn cleanup(this: &RenderApi) -> ::js_sys::Promise;
    }

    #[wasm_bindgen]
    extern "C" {
        pub type RenderGlbApi;

        #[wasm_bindgen(method)]
        pub fn add(this: &RenderGlbApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn update(this: &RenderGlbApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn remove(this: &RenderGlbApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn cleanup(this: &RenderGlbApi) -> ::js_sys::Promise;
    }

    #[wasm_bindgen]
    extern "C" {
        pub type RenderGeojsonApi;

        #[wasm_bindgen(method)]
        pub fn add(this: &RenderGeojsonApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn update(this: &RenderGeojsonApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn remove(this: &RenderGeojsonApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn cleanup(this: &RenderGeojsonApi) -> ::js_sys::Promise;
    }

    #[wasm_bindgen]
    extern "C" {
        pub type ElementColorApi;

        #[wasm_bindgen(method)]
        pub fn set(this: &ElementColorApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn clear(this: &ElementColorApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "clearAll")]
        pub fn clear_all(this: &ElementColorApi) -> ::js_sys::Promise;
    }
}

/// Render or color objects and elements in the 3D scene (non-persistent visual changes).
pub struct RenderApi {
    inner: js::RenderApi,
}

impl RenderApi {
    pub(crate) fn from_raw(raw: js::RenderApi) -> Self {
        Self { inner: raw }
    }

    /// Access the GLB render sub-API.
    pub fn glb(&self) -> RenderGlbApi {
        RenderGlbApi {
            inner: self.inner.glb(),
        }
    }

    /// Access the GeoJSON render sub-API.
    pub fn geojson(&self) -> RenderGeojsonApi {
        RenderGeojsonApi {
            inner: self.inner.geojson(),
        }
    }

    /// Access the element color sub-API.
    pub fn element_colors(&self) -> ElementColorApi {
        ElementColorApi {
            inner: self.inner.element_colors(),
        }
    }

    /// Hide an element from the scene.
    pub async fn hide_element(&self, request: &ElementVisibilityRequest) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.hide_element(&js_req)).await?;
        Ok(())
    }

    /// Hide a set of elements from the scene.
    pub async fn hide_elements_batch(
        &self,
        request: &ElementVisibilityBatchRequest,
    ) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.hide_elements_batch(&js_req)).await?;
        Ok(())
    }

    /// Unhide an element from the scene.
    pub async fn unhide_element(&self, request: &ElementVisibilityRequest) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.unhide_element(&js_req)).await?;
        Ok(())
    }

    /// Unhide a set of elements from the scene.
    pub async fn unhide_elements_batch(
        &self,
        request: &ElementVisibilityBatchRequest,
    ) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.unhide_elements_batch(&js_req)).await?;
        Ok(())
    }

    /// Set element visibility by path.
    pub async fn set_elements_visibility(
        &self,
        request: &SetElementsVisibilityRequest,
    ) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.set_elements_visibility(&js_req)).await?;
        Ok(())
    }

    /// Unhide all elements previously hidden by this API.
    pub async fn unhide_all_elements(&self) -> Result<()> {
        JsFuture::from(self.inner.unhide_all_elements()).await?;
        Ok(())
    }

    /// Add a mesh to the scene.
    pub async fn add_mesh(&self, request: &MeshRequest) -> Result<IdResult> {
        let js_req = mesh_request_to_js(request)?;
        let result = JsFuture::from(self.inner.add_mesh(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Upsert a mesh in the scene.
    pub async fn update_mesh(&self, request: &MeshRequest) -> Result<()> {
        let js_req = mesh_request_to_js(request)?;
        JsFuture::from(self.inner.update_mesh(&js_req)).await?;
        Ok(())
    }

    /// Remove a mesh from the scene.
    pub async fn remove(&self, request: &RemoveRequest) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.remove(&js_req)).await?;
        Ok(())
    }

    /// Remove all meshes added by this API from the scene.
    pub async fn cleanup(&self) -> Result<()> {
        JsFuture::from(self.inner.cleanup()).await?;
        Ok(())
    }
}

/// Render GLBs in the 3D scene.
pub struct RenderGlbApi {
    inner: js::RenderGlbApi,
}

impl RenderGlbApi {
    /// Add a GLB to the scene.
    pub async fn add(&self, request: &GlbRenderRequest) -> Result<IdResult> {
        let js_req = glb_request_to_js(request)?;
        let result = JsFuture::from(self.inner.add(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Upsert a GLB in the scene.
    pub async fn update(&self, request: &GlbRenderRequest) -> Result<()> {
        let js_req = glb_request_to_js(request)?;
        JsFuture::from(self.inner.update(&js_req)).await?;
        Ok(())
    }

    /// Remove a GLB from the scene.
    pub async fn remove(&self, request: &RemoveRequest) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.remove(&js_req)).await?;
        Ok(())
    }

    /// Remove all GLBs added by this API from the scene.
    pub async fn cleanup(&self) -> Result<()> {
        JsFuture::from(self.inner.cleanup()).await?;
        Ok(())
    }
}

/// Render GeoJSON in the 3D scene.
pub struct RenderGeojsonApi {
    inner: js::RenderGeojsonApi,
}

impl RenderGeojsonApi {
    /// Add GeoJSON to the scene.
    pub async fn add(&self, request: &GeoJsonRenderRequest) -> Result<IdResult> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.add(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Upsert GeoJSON in the scene.
    pub async fn update(&self, request: &GeoJsonRenderRequest) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.update(&js_req)).await?;
        Ok(())
    }

    /// Remove GeoJSON from the scene.
    pub async fn remove(&self, request: &RemoveRequest) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.remove(&js_req)).await?;
        Ok(())
    }

    /// Remove all GeoJSON added by this API from the scene.
    pub async fn cleanup(&self) -> Result<()> {
        JsFuture::from(self.inner.cleanup()).await?;
        Ok(())
    }
}

/// Set custom colors on elements in the scene.
pub struct ElementColorApi {
    inner: js::ElementColorApi,
}

impl ElementColorApi {
    /// Set color override on the specified elements.
    pub async fn set(&self, request: &ElementColorSetRequest) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.set(&js_req)).await?;
        Ok(())
    }

    /// Clear color override on the specified elements.
    pub async fn clear(&self, request: &ElementColorClearRequest) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.clear(&js_req)).await?;
        Ok(())
    }

    /// Clear all color overrides.
    pub async fn clear_all(&self) -> Result<()> {
        JsFuture::from(self.inner.clear_all()).await?;
        Ok(())
    }
}
