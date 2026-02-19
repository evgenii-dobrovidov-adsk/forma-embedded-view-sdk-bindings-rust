use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Enable design tools supplied by the host app (point, polygon, line picking).
    pub type DesignToolApi;

    /// Activate tool for creating a point by clicking in the 3D scene.
    /// Resolves to `Vec3 | undefined`.
    #[wasm_bindgen(method, js_name = "getPoint")]
    pub fn get_point(this: &DesignToolApi) -> js_sys::Promise;

    /// Activate tools for creating a polygon.
    /// Resolves to `Vec3[] | undefined`.
    #[wasm_bindgen(method, js_name = "getPolygon")]
    pub fn get_polygon(this: &DesignToolApi) -> js_sys::Promise;

    /// Activate tool for creating an extruded polygon.
    /// Resolves to `ExtrudedPolygon | undefined`.
    #[wasm_bindgen(method, js_name = "getExtrudedPolygon")]
    pub fn get_extruded_polygon(this: &DesignToolApi) -> js_sys::Promise;

    /// Activate tool for creating a line.
    /// Resolves to `Line | undefined`.
    #[wasm_bindgen(method, js_name = "getLine")]
    pub fn get_line(this: &DesignToolApi) -> js_sys::Promise;

    /// Subscribe to the 'start' event for edits with the drawing tools.
    #[wasm_bindgen(method, js_name = "onEditStart")]
    pub fn on_edit_start(
        this: &DesignToolApi,
        callback: &Closure<dyn FnMut()>,
    ) -> js_sys::Promise;

    /// Subscribe to the 'end' event for edits with the drawing tools.
    #[wasm_bindgen(method, js_name = "onEditEnd")]
    pub fn on_edit_end(this: &DesignToolApi, callback: &Closure<dyn FnMut()>) -> js_sys::Promise;
}
