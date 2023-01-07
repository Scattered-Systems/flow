/*
   Appellation: utils <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use crate::JsResult;
use gloo::net::http::{Request, Response};
use wasm_bindgen::prelude::*;

pub async fn fetch(req: Request) -> JsResult<Response> {
    let res = req.send().await?;
    Ok(res)
}

#[wasm_bindgen]
pub fn timestamp() -> i64 {
    chrono::Utc::now().timestamp()
}
