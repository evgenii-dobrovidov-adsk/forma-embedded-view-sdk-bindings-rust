use crate::types::*;
use crate::Result;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

pub(crate) mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type ElementsApi;

        #[wasm_bindgen(method, getter, js_name = "floorStack")]
        pub fn floor_stack(this: &ElementsApi) -> FloorStackApi;

        #[wasm_bindgen(method, getter)]
        pub fn representations(this: &ElementsApi) -> RepresentationsApi;

        #[wasm_bindgen(method, getter)]
        pub fn blobs(this: &ElementsApi) -> BlobsApi;

        #[wasm_bindgen(method)]
        pub fn get(this: &ElementsApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getByPath")]
        pub fn get_by_path(this: &ElementsApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getWorldTransform")]
        pub fn get_world_transform(this: &ElementsApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "editProperties")]
        pub fn edit_properties(this: &ElementsApi, request: &JsValue) -> ::js_sys::Promise;
    }

    #[wasm_bindgen]
    extern "C" {
        pub type FloorStackApi;

        #[wasm_bindgen(method, js_name = "createFromFloors")]
        pub fn create_from_floors(this: &FloorStackApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "createFromFloorsBatch")]
        pub fn create_from_floors_batch(
            this: &FloorStackApi,
            request: &JsValue,
        ) -> ::js_sys::Promise;
    }

    #[wasm_bindgen]
    extern "C" {
        pub type RepresentationsApi;

        #[wasm_bindgen(method, js_name = "volumeMesh")]
        pub fn volume_mesh(this: &RepresentationsApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn footprint(this: &RepresentationsApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "grossFloorAreaPolygons")]
        pub fn gross_floor_area_polygons(
            this: &RepresentationsApi,
            request: &JsValue,
        ) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "graphBuilding")]
        pub fn graph_building(this: &RepresentationsApi, request: &JsValue) -> ::js_sys::Promise;
    }

    #[wasm_bindgen]
    extern "C" {
        pub type BlobsApi;

        #[wasm_bindgen(method)]
        pub fn get(this: &BlobsApi, request: &JsValue) -> ::js_sys::Promise;
    }
}

/// Read elements and create buildings in Forma's element systems.
pub struct ElementsApi {
    inner: js::ElementsApi,
}

impl ElementsApi {
    pub(crate) fn from_raw(raw: js::ElementsApi) -> Self {
        Self { inner: raw }
    }

    /// Access the floor stack sub-API.
    pub fn floor_stack(&self) -> FloorStackApi {
        FloorStackApi {
            inner: self.inner.floor_stack(),
        }
    }

    /// Access the representations sub-API.
    pub fn representations(&self) -> RepresentationsApi {
        RepresentationsApi {
            inner: self.inner.representations(),
        }
    }

    /// Access the blobs sub-API.
    pub fn blobs(&self) -> BlobsApi {
        BlobsApi {
            inner: self.inner.blobs(),
        }
    }

    /// Get an element by URN.
    pub async fn get(&self, request: &GetElementRequest) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.get(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Get an element hierarchy located at a path relative to the root.
    pub async fn get_by_path(&self, request: &GetElementByPathRequest) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.get_by_path(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Get the world transform of an element relative to the root element.
    pub async fn get_world_transform(
        &self,
        request: &GetWorldTransformRequest,
    ) -> Result<Transform> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.get_world_transform(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Add, edit, and remove custom properties on elements via JSON Merge Patch.
    pub async fn edit_properties(&self, request: &EditPropertiesRequest) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.edit_properties(&js_req)).await?;
        Ok(())
    }
}

/// Create Floor Stack buildings.
pub struct FloorStackApi {
    inner: js::FloorStackApi,
}

impl FloorStackApi {
    /// Create a 2.5D building from a stack of floors.
    pub async fn create_from_floors(
        &self,
        request: &CreateFromFloorsRequest,
    ) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.create_from_floors(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Create multiple 2.5D buildings from stacks of floors.
    pub async fn create_from_floors_batch(
        &self,
        request: &CreateFromFloorsRequest,
    ) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.create_from_floors_batch(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }
}

/// Access derived representations of elements (volume mesh, footprint, etc.).
pub struct RepresentationsApi {
    inner: js::RepresentationsApi,
}

impl RepresentationsApi {
    /// Get the volume mesh for an element (GLB binary data).
    pub async fn volume_mesh(&self, request: &VolumeMeshRequest) -> Result<Vec<u8>> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.volume_mesh(&js_req)).await?;
        let array = ::js_sys::Uint8Array::new(&result);
        Ok(array.to_vec())
    }

    /// Get the footprint of an element (GeoJSON FeatureCollection).
    pub async fn footprint(
        &self,
        request: &RepresentationFootprintRequest,
    ) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.footprint(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Get the gross floor area polygons for an element.
    pub async fn gross_floor_area_polygons(
        &self,
        request: &RepresentationFootprintRequest,
    ) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.gross_floor_area_polygons(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Get the graph building representation for an element.
    pub async fn graph_building(
        &self,
        request: &GraphBuildingRequest,
    ) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.graph_building(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }
}

/// Read blobs related to an element.
pub struct BlobsApi {
    inner: js::BlobsApi,
}

impl BlobsApi {
    /// Retrieve a blob by its ID. Returns the raw binary data.
    pub async fn get(&self, request: &BlobGetRequest) -> Result<Vec<u8>> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.get(&js_req)).await?;
        let data = ::js_sys::Reflect::get(&result, &JsValue::from_str("data"))?;
        let array = ::js_sys::Uint8Array::new(&data);
        Ok(array.to_vec())
    }
}
