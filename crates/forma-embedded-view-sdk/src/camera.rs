use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Interact with the camera in the 3D scene.
    pub type CameraApi;

    /// Move camera view to a new position.
    #[wasm_bindgen(method, js_name = "move")]
    pub fn move_(this: &CameraApi, request: &JsValue) -> js_sys::Promise;

    /// Toggle between perspective and orthographic camera.
    #[wasm_bindgen(method, js_name = "switchPerspective")]
    pub fn switch_perspective(this: &CameraApi) -> js_sys::Promise;

    /// Capture a screenshot of the current camera view as a canvas.
    /// Resolves to `HTMLCanvasElement`.
    #[wasm_bindgen(method)]
    pub fn capture(this: &CameraApi, request: &JsValue) -> js_sys::Promise;

    /// Fetch the current camera state.
    /// Resolves to `CameraState { position, target, type }`.
    #[wasm_bindgen(method, js_name = "getCurrent")]
    pub fn get_current(this: &CameraApi) -> js_sys::Promise;

    /// Subscribe to camera changes.
    /// Resolves to `{ unsubscribe: () => void }`.
    #[wasm_bindgen(method)]
    pub fn subscribe(
        this: &CameraApi,
        callback: &Closure<dyn FnMut(JsValue)>,
    ) -> js_sys::Promise;
}
