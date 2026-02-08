use serde::{de::DeserializeOwned, Serialize};
use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["windows", "__TAURI__", "core"], js_name = invoke)]
    async fn tauri_invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["windows", "__TAURI__", "core"], js_name = invoke)]
    async fn tauri_invoke_without_args(cmd: &str) -> JsValue;
}

pub async fn invoke<A, T>(cmd: &str, args: &A) -> Result<T, String>
where
    A: Serialize,
    T: DeserializeOwned,
{
    let js_args = serde_wasm_bindgen::to_value(args).map_err(|e| e.to_string())?;
    let res = tauri_invoke(cmd, js_args).await;
    serde_wasm_bindgen::from_value(res).map_err(|e| e.to_string())
}
