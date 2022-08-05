/*
    Appellation: primitives <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use constants::*;
pub use types::*;

mod constants {
    ///
    pub const FLOW_MAINNET_PORT: u16 = 9090;
    ///
    pub const FLOW_SERVER_PORT: u16 = 8080;
    ///
    pub const FLOW_PROXY_PORT: u16 = 8080;

    pub struct Port(u16);

    #[derive(Clone, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
    pub struct Constants {
        pub name: String,
    }
}

mod types {
    use secp256k1::{PublicKey, SecretKey};
    pub use web3::types::Address as Web3Address;

    /// Type alias for a tuple ([secp256k1::SecretKey], [secp256k1::PublicKey])
    pub type SecpKeypair = (SecretKey, PublicKey);

    pub type Web3Http = web3::Web3<web3::transports::Http>;
    pub type Web3HttpResult = anyhow::Result<Web3Http>;
    pub type Web3WS = web3::Web3<web3::transports::WebSocket>;
    pub type Web3WSResult = anyhow::Result<Web3WS>;
}
