use std::fmt::{Debug, Display};
use serde::{Serialize, de::DeserializeOwned};
use serde_wasm_bindgen::{from_value, to_value};
use sycamore::prelude::console_log;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub fn log(msg: impl Display, line: u16, args: &impl Debug) {
    console_log!("{msg}: \nLine {}\n{:#?}",line.to_string(),args)
}

pub async fn call<T: DeserializeOwned>(
    cmd: &str,
    args: Option<impl Serialize>,
) -> Result<T, String> {
    match args {
        None => from_value::<T>(invoke_without_args(&cmd).await).map_err(|err| err.to_string()),
        Some(a) => from_value::<T>(invoke(&cmd, to_value(&a).unwrap()).await)
            .map_err(|err| err.to_string()),
    }
}
