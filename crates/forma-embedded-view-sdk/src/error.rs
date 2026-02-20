use std::fmt;
use wasm_bindgen::JsValue;

#[derive(Debug)]
pub enum SdkError {
    Js(JsValue),
    Serialization(String),
}

impl fmt::Display for SdkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SdkError::Js(val) => write!(f, "JS error: {:?}", val),
            SdkError::Serialization(msg) => write!(f, "Serialization error: {msg}"),
        }
    }
}

impl std::error::Error for SdkError {}

impl From<JsValue> for SdkError {
    fn from(val: JsValue) -> Self {
        SdkError::Js(val)
    }
}

impl From<serde_wasm_bindgen::Error> for SdkError {
    fn from(err: serde_wasm_bindgen::Error) -> Self {
        SdkError::Serialization(format!("{err}"))
    }
}
