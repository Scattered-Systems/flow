/*
   Appellation: api <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::interface::Interface;

pub mod endpoints;
pub mod handlers;
mod interface;

/// Type alias for [axum::Json] with a default set equal to [serde_json::Value]
pub type AxumJson<T = serde_json::Value> = axum::Json<T>;
