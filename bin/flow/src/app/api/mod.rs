/*
    Appellation: api <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

///
pub trait APISpec {
    fn run(&self) -> Result<(), scsys::BoxError>;
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FlowAPI {
    pub address: std::net::SocketAddr,
}
