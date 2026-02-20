use crate::subscription::Subscription;
use crate::types::*;
use crate::Result;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

pub(crate) mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type ProposalApi;

        #[wasm_bindgen(method, js_name = "getRootUrn")]
        pub fn get_root_urn(this: &ProposalApi) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getId")]
        pub fn get_id(this: &ProposalApi) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "addElement")]
        pub fn add_element(this: &ProposalApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "replaceElement")]
        pub fn replace_element(this: &ProposalApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "removeElement")]
        pub fn remove_element(this: &ProposalApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "replaceTerrain")]
        pub fn replace_terrain(this: &ProposalApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "updateElements")]
        pub fn update_elements(this: &ProposalApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn subscribe(
            this: &ProposalApi,
            callback: &Closure<dyn FnMut(JsValue)>,
            options: Option<&JsValue>,
        ) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "awaitProposalPersisted")]
        pub fn await_proposal_persisted(this: &ProposalApi) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "getAll")]
        pub fn get_all(this: &ProposalApi) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn get(this: &ProposalApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn create(this: &ProposalApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn update(this: &ProposalApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn delete(this: &ProposalApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn duplicate(this: &ProposalApi, request: &JsValue) -> ::js_sys::Promise;

        #[wasm_bindgen(method, js_name = "switch")]
        pub fn switch(this: &ProposalApi, request: &JsValue) -> ::js_sys::Promise;
    }
}

/// Access proposal metadata and modify elements in it.
pub struct ProposalApi {
    inner: js::ProposalApi,
}

impl ProposalApi {
    pub(crate) fn from_raw(raw: js::ProposalApi) -> Self {
        Self { inner: raw }
    }

    /// Fetch the top-level URN for the proposal.
    pub async fn get_root_urn(&self) -> Result<String> {
        let result = JsFuture::from(self.inner.get_root_urn()).await?;
        Ok(result.as_string().unwrap_or_default())
    }

    /// Fetch the unique identifier of the proposal.
    pub async fn get_id(&self) -> Result<String> {
        let result = JsFuture::from(self.inner.get_id()).await?;
        Ok(result.as_string().unwrap_or_default())
    }

    /// Add a new element to the proposal.
    pub async fn add_element(&self, request: &ProposalAddElementRequest) -> Result<PathResult> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.add_element(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Replace an element in the proposal.
    pub async fn replace_element(
        &self,
        request: &ProposalReplaceElementRequest,
    ) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.replace_element(&js_req)).await?;
        Ok(())
    }

    /// Remove an element from the proposal.
    pub async fn remove_element(&self, request: &ProposalRemoveElementRequest) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.remove_element(&js_req)).await?;
        Ok(())
    }

    /// Replace the existing terrain on the proposal.
    pub async fn replace_terrain(
        &self,
        request: &ProposalReplaceTerrainRequest,
    ) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.replace_terrain(&js_req)).await?;
        Ok(())
    }

    /// Execute a batch of element operations (add, replace, remove).
    pub async fn update_elements(
        &self,
        request: &ProposalUpdateElementsRequest,
    ) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.update_elements(&js_req)).await?;
        Ok(())
    }

    /// Subscribe to proposal changes.
    pub async fn subscribe(
        &self,
        mut callback: impl FnMut(ProposalChangeEvent) + 'static,
        options: Option<&ProposalSubscribeOptions>,
    ) -> Result<Subscription> {
        let closure = Closure::wrap(Box::new(move |val: JsValue| {
            if let Ok(event) = serde_wasm_bindgen::from_value(val) {
                callback(event);
            }
        }) as Box<dyn FnMut(JsValue)>);
        let js_opts = options
            .map(serde_wasm_bindgen::to_value)
            .transpose()?;
        let result =
            JsFuture::from(self.inner.subscribe(&closure, js_opts.as_ref())).await?;
        let unsubscribe_fn: ::js_sys::Function =
            ::js_sys::Reflect::get(&result, &"unsubscribe".into())?.into();
        Ok(Subscription::new(closure, unsubscribe_fn))
    }

    /// Wait until the currently loaded proposal is persisted.
    pub async fn await_proposal_persisted(&self) -> Result<()> {
        JsFuture::from(self.inner.await_proposal_persisted()).await?;
        Ok(())
    }

    /// Get all proposals for the current project.
    pub async fn get_all(&self) -> Result<serde_json::Value> {
        let result = JsFuture::from(self.inner.get_all()).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Get a proposal by ID and optional revision.
    pub async fn get(&self, request: &ProposalGetRequest) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.get(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Create a new proposal.
    pub async fn create(&self, request: &ProposalCreateRequest) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.create(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Update an existing proposal.
    pub async fn update(&self, request: &ProposalUpdateRequest) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.update(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Delete a proposal (soft delete).
    pub async fn delete(&self, request: &ProposalDeleteRequest) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.delete(&js_req)).await?;
        Ok(())
    }

    /// Duplicate a proposal.
    pub async fn duplicate(
        &self,
        request: &ProposalDuplicateRequest,
    ) -> Result<serde_json::Value> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        let result = JsFuture::from(self.inner.duplicate(&js_req)).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Switch to a different proposal.
    pub async fn switch(&self, request: &ProposalSwitchRequest) -> Result<()> {
        let js_req = serde_wasm_bindgen::to_value(request)?;
        JsFuture::from(self.inner.switch(&js_req)).await?;
        Ok(())
    }
}
