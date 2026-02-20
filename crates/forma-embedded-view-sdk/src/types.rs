use serde::{Deserialize, Serialize};

pub type Vec3 = [f64; 3];

/// GeoJSON FeatureCollection. Uses `serde_json::Value` for maximum flexibility
/// with arbitrary GeoJSON properties and geometry types.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureCollection {
    pub r#type: String,
    pub features: Vec<Feature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub r#type: String,
    #[serde(default)]
    pub properties: serde_json::Value,
    pub geometry: serde_json::Value,
}

/// Licensing information governing use and transfer of data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Licensing {
    pub attributions: Vec<Attribution>,
    pub exportable: Option<bool>,
    #[serde(rename = "licenseUrl")]
    pub license_url: String,
    #[serde(rename = "providerDescriptionUrl")]
    pub provider_description_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribution {
    pub action: AttributionAction,
    pub content: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AttributionAction {
    Display,
    Transfer,
}

/// Library item status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LibraryStatus {
    Success,
    Failed,
    Pending,
}

/// Record of an item in the library service.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryItem {
    pub name: String,
    pub status: LibraryStatus,
    pub urn: Option<String>,
    #[serde(rename = "authContext")]
    pub auth_context: String,
    pub id: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: f64,
}

/// Data which defines the content of a library item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryItemData {
    pub name: String,
    pub status: LibraryStatus,
    pub urn: Option<String>,
}

/// Project metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Camera state returned by `camera.get_current()`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraState {
    pub position: Vec3,
    pub target: Vec3,
    #[serde(rename = "type")]
    pub camera_type: String,
}

/// Camera move request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraMoveRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Vec3>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<Vec3>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition: Option<bool>,
}

/// Camera capture request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraCaptureRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
}

/// Access token response from auth methods.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessTokenResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
}

/// Auth configuration for APS flows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "callbackUrl")]
    pub callback_url: String,
    pub scopes: Vec<String>,
}

/// Geometry data for mesh rendering (position + optional color/normals).
///
/// Note: these fields are converted to `Float32Array` / `Uint8Array` internally
/// when passed to the JS SDK, not via serde serialization.
#[derive(Debug, Clone)]
pub struct GeometryData {
    pub position: Vec<f32>,
    pub color: Option<Vec<u8>>,
}

/// Mesh rendering request (add or update).
///
/// Note: converted to JS manually (not via serde) to produce typed arrays
/// for `geometryData.position` / `geometryData.color`.
#[derive(Debug, Clone)]
pub struct MeshRequest {
    pub id: String,
    pub geometry_data: GeometryData,
    pub transform: Option<Transform>,
}

/// 4x4 transformation matrix stored column-major.
pub type Transform = [f64; 16];

/// Result of adding a mesh / GLB / GeoJSON with a generated id.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdResult {
    pub id: String,
}

/// Element path result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathResult {
    pub path: String,
}

/// Element URN result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrnResult {
    pub urn: String,
}

/// File upload result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileUploadResult {
    #[serde(rename = "fileId")]
    pub file_id: String,
    #[serde(rename = "blobId")]
    pub blob_id: String,
}

/// Sun date request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SunDateRequest {
    pub date: String,
}

/// Terrain bounding box.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainBbox {
    pub min: Vec3,
    pub max: Vec3,
}

/// Terrain elevation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElevationRequest {
    pub x: f64,
    pub y: f64,
}

/// Presentation unit system ("metric" or "imperial").
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum UnitSystem {
    Metric,
    Imperial,
}

/// Colorbar entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorbarEntry {
    pub color: String,
    pub label: String,
}

/// Colorbar add request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorbarAddRequest {
    pub entries: Vec<ColorbarEntry>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Extruded polygon returned by design tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtrudedPolygon {
    pub points: Vec<Vec3>,
    pub height: f64,
}

/// Line returned by design tool.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Line {
    pub points: Vec<Vec3>,
}

/// Embedded view state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmbeddedViewState {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Locale update event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocaleUpdate {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Floating panel options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FloatingPanelOptions {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Close embedded view options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloseEmbeddedViewOptions {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Create message port options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMessagePortOptions {
    #[serde(rename = "receiverId")]
    pub receiver_id: String,
}

/// SDK configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdkConfig {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// GeoJSON render request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoJsonRenderRequest {
    pub id: String,
    pub data: FeatureCollection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<Transform>,
}

/// GLB render request.
///
/// Note: converted to JS manually (not via serde) to produce `ArrayBuffer`
/// for the `glb` field.
#[derive(Debug, Clone)]
pub struct GlbRenderRequest {
    pub id: String,
    pub glb: Vec<u8>,
    pub transform: Option<Transform>,
}

/// Remove request (by id).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveRequest {
    pub id: String,
}

/// Hide/unhide element request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementVisibilityRequest {
    pub path: String,
}

/// Batch element visibility request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementVisibilityBatchRequest {
    pub paths: Vec<String>,
}

/// Set elements visibility request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetElementsVisibilityRequest {
    pub paths: Vec<String>,
    pub visible: bool,
}

/// Element color set request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementColorSetRequest {
    pub paths: Vec<String>,
    pub color: String,
}

/// Element color clear request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementColorClearRequest {
    pub paths: Vec<String>,
}

/// Get paths by category request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPathsByCategoryRequest {
    pub category: String,
}

/// Get footprint request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFootprintRequest {
    pub path: String,
}

/// Get triangles request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTrianglesRequest {
    pub path: String,
}

/// Get paths inside polygons request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPathsInsidePolygonsRequest {
    pub polygons: Vec<Vec<Vec3>>,
}

/// Get element request (by URN).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetElementRequest {
    pub urn: String,
}

/// Get element by path request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetElementByPathRequest {
    pub path: String,
}

/// Get world transform request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWorldTransformRequest {
    pub path: String,
}

/// Edit element properties request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditPropertiesRequest {
    pub path: String,
    pub properties: serde_json::Value,
}

/// Volume mesh request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeMeshRequest {
    pub urn: String,
}

/// Footprint request (for representations).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepresentationFootprintRequest {
    pub urn: String,
}

/// Graph building request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphBuildingRequest {
    pub urn: String,
}

/// Blob get request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobGetRequest {
    pub id: String,
}

/// Invoke endpoint request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvokeEndpointRequest {
    #[serde(rename = "endpointId")]
    pub endpoint_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<serde_json::Value>,
}

/// Extension storage set object request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageSetObjectRequest {
    pub key: String,
    pub data: String,
}

/// Extension storage get text object request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageGetTextObjectRequest {
    pub key: String,
}

/// Extension storage get binary object request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageGetBinaryObjectRequest {
    pub key: String,
}

/// Extension storage list objects request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageListObjectsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
}

/// Extension storage delete object request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageDeleteObjectRequest {
    pub key: String,
}

/// Create floor stack request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFromFloorsRequest {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Proposal add element request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalAddElementRequest {
    pub urn: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Vec3>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform: Option<Transform>,
}

/// Proposal replace element request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalReplaceElementRequest {
    pub path: String,
    pub urn: String,
}

/// Proposal remove element request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalRemoveElementRequest {
    pub path: String,
}

/// Proposal replace terrain request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalReplaceTerrainRequest {
    pub urn: String,
}

/// Proposal update elements (batch) request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalUpdateElementsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<ProposalAddElementRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace: Option<Vec<ProposalReplaceElementRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<ProposalRemoveElementRequest>>,
}

/// Proposal subscribe options.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalSubscribeOptions {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Proposal get request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalGetRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}

/// Proposal create request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalCreateRequest {
    pub name: String,
}

/// Proposal update request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalUpdateRequest {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Proposal delete request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalDeleteRequest {
    pub id: String,
}

/// Proposal duplicate request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalDuplicateRequest {
    pub id: String,
}

/// Proposal switch request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalSwitchRequest {
    pub id: String,
}

/// Analysis list request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisListRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

/// Trigger noise analysis request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerNoiseRequest {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Trigger sun analysis request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TriggerSunRequest {
    pub date: String,
}

/// Get analysis request (by id).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAnalysisRequest {
    pub id: String,
}

/// Get ground grid request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGroundGridRequest {
    pub id: String,
}

/// Area metrics calculate request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AreaMetricsCalculateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paths: Option<Vec<String>>,
}

/// Predict wind request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictWindRequest {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Ground texture add request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundTextureAddRequest {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Ground texture update data request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundTextureUpdateDataRequest {
    pub id: String,
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Ground texture update position request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundTextureUpdatePositionRequest {
    pub id: String,
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Ground texture remove request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundTextureRemoveRequest {
    pub id: String,
}

/// Terrain pad.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerrainPad {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Generator put request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorPutRequest {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Generator list request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratorListRequest {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Library create item request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryCreateItemRequest {
    #[serde(flatten)]
    pub data: LibraryItemData,
}

/// Library update item request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryUpdateItemRequest {
    pub id: String,
    #[serde(flatten)]
    pub data: LibraryItemData,
}

/// Library delete item request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryDeleteItemRequest {
    pub id: String,
}

/// Integrate create element hierarchy request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateElementHierarchyRequest {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Integrate create element v2 request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateElementV2Request {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Integrate update element v2 request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateElementV2Request {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Integrate batch ingest elements v2 request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchIngestElementsV2Request {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Integrate upload file request.
///
/// Note: converted to JS manually (not via serde) to produce `ArrayBuffer`
/// for the `data` field.
#[derive(Debug, Clone)]
pub struct UploadFileRequest {
    pub data: Vec<u8>,
    pub name: String,
}

/// GeoData upload request types.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GeoDataType {
    Buildings,
    Roads,
    PropertyBoundaries,
}

/// Geo-location reference.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    #[serde(rename = "refPoint")]
    pub ref_point: [f64; 2],
    pub srid: u32,
}

/// GeoData upload request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoDataUploadRequest {
    pub data: FeatureCollection,
    #[serde(rename = "dataType")]
    pub data_type: GeoDataType,
    #[serde(rename = "geoLocation", skip_serializing_if = "Option::is_none")]
    pub geo_location: Option<GeoLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licensing: Option<Licensing>,
}

/// Proposal change event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProposalChangeEvent {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

/// Selection change event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionChangeEvent {
    pub paths: Vec<String>,
}

/// Opaque capture result from `camera.capture()`.
/// Wraps an `HTMLCanvasElement` internally and provides methods
/// to extract the image data without exposing DOM types.
pub struct CaptureResult {
    pub(crate) inner: wasm_bindgen::JsValue,
}

impl CaptureResult {
    /// Convert the captured canvas to a data URL (e.g. `data:image/png;base64,...`).
    pub fn to_data_url(&self) -> std::result::Result<String, String> {
        let canvas: &web_sys::HtmlCanvasElement = wasm_bindgen::JsCast::unchecked_ref(&self.inner);
        canvas.to_data_url().map_err(|e| format!("{e:?}"))
    }

    /// Convert the captured canvas to a data URL with a specific MIME type.
    pub fn to_data_url_with_type(&self, mime_type: &str) -> std::result::Result<String, String> {
        let canvas: &web_sys::HtmlCanvasElement = wasm_bindgen::JsCast::unchecked_ref(&self.inner);
        canvas
            .to_data_url_with_type(mime_type)
            .map_err(|e| format!("{e:?}"))
    }

    /// Get the width of the captured image in pixels.
    pub fn width(&self) -> u32 {
        let canvas: &web_sys::HtmlCanvasElement = wasm_bindgen::JsCast::unchecked_ref(&self.inner);
        canvas.width()
    }

    /// Get the height of the captured image in pixels.
    pub fn height(&self) -> u32 {
        let canvas: &web_sys::HtmlCanvasElement = wasm_bindgen::JsCast::unchecked_ref(&self.inner);
        canvas.height()
    }
}

impl std::fmt::Debug for CaptureResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CaptureResult")
            .field("width", &self.width())
            .field("height", &self.height())
            .finish()
    }
}

/// Opaque message port handle from `create_message_port()`.
/// Wraps a `MessagePort` internally.
pub struct MessagePortHandle {
    pub(crate) inner: wasm_bindgen::JsValue,
}

impl MessagePortHandle {
    /// Post a message through this port.
    pub fn post_message(&self, data: &serde_json::Value) -> crate::Result<()> {
        let js_val = serde_wasm_bindgen::to_value(data)?;
        let port: &web_sys::MessagePort = wasm_bindgen::JsCast::unchecked_ref(&self.inner);
        port.post_message(&js_val).map_err(crate::SdkError::from)
    }
}

impl std::fmt::Debug for MessagePortHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MessagePortHandle").finish()
    }
}
