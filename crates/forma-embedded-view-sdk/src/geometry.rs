use crate::types::*;
use crate::Result;
use wasm_bindgen_futures::JsFuture;

pub(crate) mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type GeometryApi;

        #[wasm_bindgen(method, js_name = "getPathsByCategory")]
        pub fn get_paths_by_category(this: &GeometryApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getPathsForVirtualElements")]
        pub fn get_paths_for_virtual_elements(
            this: &GeometryApi,
            request: Option<&JsValue>,
        ) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getFootprint")]
        pub fn get_footprint(this: &GeometryApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getTriangles")]
        pub fn get_triangles(this: &GeometryApi, request: Option<&JsValue>) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getPathsInsidePolygons")]
        pub fn get_paths_inside_polygons(
            this: &GeometryApi,
            request: &JsValue,
        ) -> ::js_sys::Promise;
    }
}

/// Read geometry data from Forma (paths by category, footprints, triangles).
pub struct GeometryApi {
    inner: js::GeometryApi,
}

impl GeometryApi {
    pub(crate) fn from_raw(raw: js::GeometryApi) -> Self {
        Self { inner: raw }
    }

    /// Fetch paths of all elements tagged with a specific category.
    pub async fn get_paths_by_category(
        &self,
        request: &GetPathsByCategoryRequest,
    ) -> Result<Vec<String>> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.get_paths_by_category(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Fetch paths of all elements with the 'virtual' property set to true.
    pub async fn get_paths_for_virtual_elements(&self) -> Result<Vec<String>> {
        let result =
            JsFuture::from(self.inner.get_paths_for_virtual_elements(None)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Fetch the footprint representation of an element.
    pub async fn get_footprint(
        &self,
        request: &GetFootprintRequest,
    ) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.get_footprint(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Fetch concatenated triangle mesh for an element and its children.
    /// Returns a flat Vec<f32> of vertex positions (x, y, z triplets).
    pub async fn get_triangles(
        &self,
        request: Option<&GetTrianglesRequest>,
    ) -> Result<Vec<f32>> {
        let js_req = request
            .map(serde_wasm_bindgen::to_value)
            .transpose()?;
        let result =
            JsFuture::from(self.inner.get_triangles(js_req.as_ref())).await?;
        let typed_array: ::js_sys::Float32Array = result.into();
        Ok(typed_array.to_vec())
    }

    /// Get all paths where geometry overlaps the provided polygons.
    pub async fn get_paths_inside_polygons(
        &self,
        request: &GetPathsInsidePolygonsRequest,
    ) -> Result<Vec<String>> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.get_paths_inside_polygons(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }
}
