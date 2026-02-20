use crate::types::*;
use crate::Result;
use wasm_bindgen_futures::JsFuture;

pub(crate) mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type AuthApi;

        #[wasm_bindgen(method)]
        pub fn configure(this: &AuthApi, input: &JsValue);

        #[wasm_bindgen(method, js_name = "acquireTokenSilent")]
        pub fn acquire_token_silent(this: &AuthApi) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "acquireTokenPopup")]
        pub fn acquire_token_popup(this: &AuthApi) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "acquireTokenOverlay")]
        pub fn acquire_token_overlay(this: &AuthApi) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "refreshCurrentToken")]
        pub fn refresh_current_token(this: &AuthApi) -> ::js_sys::Promise;
    }
}

/// Manage access tokens for APS (Autodesk Platform Services) auth flows.
pub struct AuthApi {
    inner: js::AuthApi,
}

impl AuthApi {
    pub(crate) fn from_raw(raw: js::AuthApi) -> Self {
        Self { inner: raw }
    }

    /// Configure extension with client ID, callback URL, and scopes.
    pub fn configure(&self, config: &AuthConfig) -> Result<()> {
        let js_config = serde_wasm_bindgen::to_value(config)?;
        self.inner.configure(&js_config);
        Ok(())
    }

    /// Get the current access token if valid, refreshing if needed.
    /// Returns `None` if no token is available.
    pub async fn acquire_token_silent(&self) -> Result<Option<AccessTokenResponse>> {
        let result = JsFuture::from(self.inner.acquire_token_silent()).await?;
        if result.is_undefined() || result.is_null() {
            return Ok(None);
        }
        Ok(Some(serde_wasm_bindgen::from_value(result)?))
    }

    /// Acquire an access token via a popup authorization flow.
    pub async fn acquire_token_popup(&self) -> Result<AccessTokenResponse> {
        let result = JsFuture::from(self.inner.acquire_token_popup()).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Acquire an access token by showing a login overlay, then popup.
    pub async fn acquire_token_overlay(&self) -> Result<AccessTokenResponse> {
        let result = JsFuture::from(self.inner.acquire_token_overlay()).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Refresh the current token. Returns an error if no token is stored.
    pub async fn refresh_current_token(&self) -> Result<AccessTokenResponse> {
        let result = JsFuture::from(self.inner.refresh_current_token()).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }
}
