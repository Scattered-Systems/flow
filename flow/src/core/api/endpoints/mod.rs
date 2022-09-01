/*
   Appellation: endpoints
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::{crud::CrudRouter, ethereum::Web3Router, index::Homepage};

mod crud;
mod ethereum;
mod index;

pub trait PageRouter {
    fn path(&self) -> String;
    fn router(&self) -> axum::Router;
}
