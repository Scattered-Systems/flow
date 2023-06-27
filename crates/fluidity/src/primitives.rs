/*
   Appellation: primitives <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::{constants::*, types::*};
#[cfg(target_family = "wasm32-unknown-unknown")]
pub use self::wasm::*;

mod constants {

}

mod types {}

#[cfg(target_family = "wasm32-unknown-unknown")]
mod wasm {
   use wasm_bindgen::prelude::JsError;


   pub type JsResult<T = ()> = Result<T, JsError>;
}

