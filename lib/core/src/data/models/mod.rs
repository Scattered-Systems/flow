/*
    Appellation: models <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{accounts::*, items::*, tokens::*};

mod accounts;
mod items;
mod tokens;

pub trait FlowModelSpec {
    fn create(&self) -> Result<Self, scsys::BoxError>
        where
            Self: Sized;
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ModelId {
    temporal: i64,
}
