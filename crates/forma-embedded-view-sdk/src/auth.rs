use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Manage access tokens for APS (Autodesk Platform Services) auth flows.
    pub type AuthApi;

    /// Configure extension with client ID, callback URL, and scopes.
    #[wasm_bindgen(method)]
    pub fn configure(this: &AuthApi, input: &JsValue);

    /// Get the current access token if valid, refreshing if needed.
    /// Resolves to `{ accessToken: string } | undefined`.
    #[wasm_bindgen(method, js_name = "acquireTokenSilent")]
    pub fn acquire_token_silent(this: &AuthApi) -> js_sys::Promise;

    /// Acquire an access token via a popup authorization flow.
    /// Resolves to `{ accessToken: string }`.
    #[wasm_bindgen(method, js_name = "acquireTokenPopup")]
    pub fn acquire_token_popup(this: &AuthApi) -> js_sys::Promise;

    /// Acquire an access token by showing a login overlay, then popup.
    /// Resolves to `{ accessToken: string }`.
    #[wasm_bindgen(method, js_name = "acquireTokenOverlay")]
    pub fn acquire_token_overlay(this: &AuthApi) -> js_sys::Promise;

    /// Refresh the current token. Throws if no token is stored.
    /// Resolves to `{ accessToken: string }`.
    #[wasm_bindgen(method, js_name = "refreshCurrentToken")]
    pub fn refresh_current_token(this: &AuthApi) -> js_sys::Promise;
}
