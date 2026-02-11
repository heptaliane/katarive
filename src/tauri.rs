use serde::{de::DeserializeOwned, Serialize};
use shared::document::{Document, FetchDocumentArgs};
use shared::speaker::{SpeakerControlArgs, SpeakerStartArgs};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke, catch)]
    async fn tauri_invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke, catch)]
    async fn tauri_invoke_without_args(cmd: &str) -> Result<JsValue, JsValue>;
}

#[derive(Serialize)]
pub struct InvokeWrapper<A>
where
    A: Serialize,
{
    args: A,
}

pub async fn invoke<A, T>(cmd: &str, args: &A) -> Result<T, String>
where
    A: Serialize,
    T: DeserializeOwned,
{
    let js_args =
        serde_wasm_bindgen::to_value(&InvokeWrapper { args }).map_err(|e| e.to_string())?;
    let res = tauri_invoke(cmd, js_args)
        .await
        .map_err(|e| e.as_string().expect("Unsupported error type occured"))?;
    serde_wasm_bindgen::from_value(res).map_err(|e| e.to_string())
}

pub async fn fetch_document(url: String) -> Result<Document, String> {
    let args = FetchDocumentArgs { url };
    invoke::<FetchDocumentArgs, Document>("fetch_document", &args).await
}

pub async fn start_speech(name: String, options: HashMap<String, String>) -> Result<(), String> {
    let args = SpeakerStartArgs { name, options };
    invoke::<SpeakerStartArgs, ()>("start_speech", &args).await
}

pub async fn pause_speech(name: String) -> Result<(), String> {
    let args = SpeakerControlArgs { name };
    invoke::<SpeakerControlArgs, ()>("pause_speech", &args).await
}

pub async fn resume_speech(name: String) -> Result<(), String> {
    let args = SpeakerControlArgs { name };
    invoke::<SpeakerControlArgs, ()>("resume_speech", &args).await
}
