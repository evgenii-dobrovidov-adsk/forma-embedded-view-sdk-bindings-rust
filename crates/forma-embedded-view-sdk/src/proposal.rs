use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Access proposal metadata and modify elements in it.
    pub type ProposalApi;

    /// Fetch the top-level URN for the proposal.
    #[wasm_bindgen(method, js_name = "getRootUrn")]
    pub fn get_root_urn(this: &ProposalApi) -> js_sys::Promise;

    /// Fetch the unique identifier of the proposal.
    #[wasm_bindgen(method, js_name = "getId")]
    pub fn get_id(this: &ProposalApi) -> js_sys::Promise;

    /// Add a new element to the proposal. Resolves to `{ path }`.
    #[wasm_bindgen(method, js_name = "addElement")]
    pub fn add_element(this: &ProposalApi, request: &JsValue) -> js_sys::Promise;

    /// Replace an element in the proposal.
    #[wasm_bindgen(method, js_name = "replaceElement")]
    pub fn replace_element(this: &ProposalApi, request: &JsValue) -> js_sys::Promise;

    /// Remove an element from the proposal.
    #[wasm_bindgen(method, js_name = "removeElement")]
    pub fn remove_element(this: &ProposalApi, request: &JsValue) -> js_sys::Promise;

    /// Replace the existing terrain on the proposal.
    #[wasm_bindgen(method, js_name = "replaceTerrain")]
    pub fn replace_terrain(this: &ProposalApi, request: &JsValue) -> js_sys::Promise;

    /// Execute a batch of element operations (add, replace, remove).
    #[wasm_bindgen(method, js_name = "updateElements")]
    pub fn update_elements(this: &ProposalApi, request: &JsValue) -> js_sys::Promise;

    /// Subscribe to proposal changes.
    #[wasm_bindgen(method)]
    pub fn subscribe(
        this: &ProposalApi,
        callback: &Closure<dyn FnMut(JsValue)>,
        options: Option<&JsValue>,
    ) -> js_sys::Promise;

    /// Wait until the currently loaded proposal is persisted.
    #[wasm_bindgen(method, js_name = "awaitProposalPersisted")]
    pub fn await_proposal_persisted(this: &ProposalApi) -> js_sys::Promise;

    /// Get all proposals for the current project.
    #[wasm_bindgen(method, js_name = "getAll")]
    pub fn get_all(this: &ProposalApi) -> js_sys::Promise;

    /// Get a proposal by ID and optional revision.
    #[wasm_bindgen(method)]
    pub fn get(this: &ProposalApi, request: &JsValue) -> js_sys::Promise;

    /// Create a new proposal.
    #[wasm_bindgen(method)]
    pub fn create(this: &ProposalApi, request: &JsValue) -> js_sys::Promise;

    /// Update an existing proposal.
    #[wasm_bindgen(method)]
    pub fn update(this: &ProposalApi, request: &JsValue) -> js_sys::Promise;

    /// Delete a proposal (soft delete).
    #[wasm_bindgen(method)]
    pub fn delete(this: &ProposalApi, request: &JsValue) -> js_sys::Promise;

    /// Duplicate a proposal.
    #[wasm_bindgen(method)]
    pub fn duplicate(this: &ProposalApi, request: &JsValue) -> js_sys::Promise;

    /// Switch to a different proposal.
    #[wasm_bindgen(method, js_name = "switch")]
    pub fn switch(this: &ProposalApi, request: &JsValue) -> js_sys::Promise;
}
