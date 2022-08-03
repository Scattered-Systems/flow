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

    ///
    pub type SecpKeypair = (SecretKey, PublicKey);
}
