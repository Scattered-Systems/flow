/*
    Appellation: api <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use endpoints::*;

mod endpoints;

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ApplicationProgramInterface {
    pub server: crate::Server,
}
