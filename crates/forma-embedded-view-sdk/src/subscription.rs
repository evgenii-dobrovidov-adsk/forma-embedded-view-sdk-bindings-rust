use wasm_bindgen::prelude::*;

pub(crate) mod js {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_name = Object)]
        pub type SubscriptionResult;

        #[wasm_bindgen(method, getter)]
        pub fn unsubscribe(this: &SubscriptionResult) -> js_sys::Function;
    }
}

/// A handle to an active subscription. Unsubscribes automatically on drop.
pub struct Subscription {
    _closure: Closure<dyn FnMut(JsValue)>,
    unsubscribe_fn: js_sys::Function,
}

impl Subscription {
    pub(crate) fn new(closure: Closure<dyn FnMut(JsValue)>, unsubscribe_fn: js_sys::Function) -> Self {
        Self {
            _closure: closure,
            unsubscribe_fn,
        }
    }

    pub fn unsubscribe(&self) {
        let _ = self.unsubscribe_fn.call0(&JsValue::NULL);
    }
}

impl Drop for Subscription {
    fn drop(&mut self) {
        self.unsubscribe();
    }
}

/// A handle to an active subscription using a no-arg callback.
pub struct VoidSubscription {
    _closure: Closure<dyn FnMut()>,
    unsubscribe_fn: js_sys::Function,
}

impl VoidSubscription {
    pub(crate) fn new(closure: Closure<dyn FnMut()>, unsubscribe_fn: js_sys::Function) -> Self {
        Self {
            _closure: closure,
            unsubscribe_fn,
        }
    }

    pub fn unsubscribe(&self) {
        let _ = self.unsubscribe_fn.call0(&JsValue::NULL);
    }
}

impl Drop for VoidSubscription {
    fn drop(&mut self) {
        self.unsubscribe();
    }
}
