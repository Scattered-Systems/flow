/*
   Appellation: utils <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(any(feature = "wasm", all(target_family = "wasm", not(target_os = "wasi"))))]
pub use self::wasm::*;

/// [timestamp] is a simple wrapper for a [chrono::Utc] timestamp.
#[cfg(any(target_family = "unix", target_family = "windows"))]
pub fn timestamp() -> i64 {
    chrono::Utc::now().timestamp()
}

#[cfg(any(feature = "wasm", all(target_family = "wasm", not(target_os = "wasi"))))]
mod wasm {
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
}
