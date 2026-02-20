use crate::subscription::Subscription;
use crate::Result;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

pub(crate) mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        pub type SelectionApi;

        #[wasm_bindgen(method, js_name = "getSelection")]
        pub fn get_selection(this: &SelectionApi) -> ::js_sys::Promise;

        #[wasm_bindgen(method)]
        pub fn subscribe(
            this: &SelectionApi,
            callback: &Closure<dyn FnMut(JsValue)>,
        ) -> ::js_sys::Promise;
    }
}

/// Interact with user's selection (shift-clicked elements in the scene).
pub struct SelectionApi {
    inner: js::SelectionApi,
}

impl SelectionApi {
    pub(crate) fn from_raw(raw: js::SelectionApi) -> Self {
        Self { inner: raw }
    }

    /// Get selected element paths.
    pub async fn get_selection(&self) -> Result<Vec<String>> {
        let result = JsFuture::from(self.inner.get_selection()).await?;
        Ok(serde_wasm_bindgen::from_value(result)?)
    }

    /// Subscribe to selection changes. Callback receives the list of selected paths.
    pub async fn subscribe(
        &self,
        mut callback: impl FnMut(Vec<String>) + 'static,
    ) -> Result<Subscription> {
        let closure = Closure::wrap(Box::new(move |val: JsValue| {
            if let Ok(paths) = serde_wasm_bindgen::from_value(val) {
                callback(paths);
            }
        }) as Box<dyn FnMut(JsValue)>);
        let result = JsFuture::from(self.inner.subscribe(&closure)).await?;
        let unsubscribe_fn: ::js_sys::Function =
            ::js_sys::Reflect::get(&result, &"unsubscribe".into())?.into();
        Ok(Subscription::new(closure, unsubscribe_fn))
    }
}
