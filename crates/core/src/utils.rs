/*
   Appellation: utils <module>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
#[cfg(target_family = "wasm32-unknown-unknown")]
pub use self::wasm::*;

/// [timestamp] is a simple wrapper for a [chrono::Utc] timestamp.
#[cfg(not(target_family = "wasm32-unknown-unknown"))]
pub fn timestamp() -> i64 {
    chrono::Utc::now().timestamp()
}

#[cfg(target_family = "wasm32-unknown-unknown")]
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
