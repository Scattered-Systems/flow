/*
   Appellation: primitives <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::{constants::*, types::*};

mod constants {

}

mod types {
   use wasm_bindgen::prelude::JsError;


   pub type JsResult<T = ()> = Result<T, JsError>;
}

