/*
   Appellation: primitives <module>
   Contrib: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
#[cfg(target_family = "wasm32-unknown-unknown")]
pub use self::wasm::*;
pub use self::{constants::*, statics::*, types::*};

mod constants {
    ///
    pub const DEFAULT_BUFFER_SIZE: usize = 1024;
    ///
    pub const DEFAULT_PORT: u16 = 8080;
    ///
    pub const LOCALHOST: [u8; 4] = [0, 0, 0, 0];
}

mod statics {}

mod types {
    /// [AsyncError] is a simple type alias for a [Box]ed [dyn std::error::Error] that is [Send] and [Sync].
    pub type AsyncError = Box<dyn std::error::Error + Send + Sync>;
    /// [AsyncResult] is a simple type alias for a [Result] with a [Box]ed [dyn std::error::Error] as its [Err] variant.
    pub type AsyncResult<T = ()> = Result<T, AsyncError>;
    ///
    pub type BoxError = Box<dyn std::error::Error>;
    /// [BoxResult] is a simple type alias for a [Result] with a [Box]ed [dyn std::error::Error] as its [Err] variant.
    pub type BoxResult<T = ()> = Result<T, BoxError>;
}

#[cfg(target_family = "wasm32-unknown-unknown")]
mod wasm {
    use wasm_bindgen::prelude::JsError;

    pub type JsResult<T = ()> = Result<T, JsError>;
}