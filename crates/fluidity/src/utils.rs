/*
   Appellation: utils <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn timestamp() -> i64 {
    chrono::Utc::now().timestamp()
}
