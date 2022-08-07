/*
    Appellation: types <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use secp256k1::{PublicKey, SecretKey};

/// Type alias for a tuple ([secp256k1::SecretKey], [secp256k1::PublicKey])
pub type SecpKeypair = (SecretKey, PublicKey);
/// Type alias for a Web3 address [web3::Address]
pub type Web3Address = web3::types::Address;
/// Type alias for [web3::Web3] leveraging an HTTP transport [web3::transports::Http]
pub type Web3Http = web3::Web3<web3::transports::Http>;
/// Type alias for [anyhow::Result] of type [Web3Http]
pub type Web3HttpResult = anyhow::Result<Web3Http>;
/// Type alias for [web3::Web3] leveraging an WebSocket transport [web3::transports::WebSocket]
pub type Web3WS = web3::Web3<web3::transports::WebSocket>;
/// Type alias for [anyhow::Result] of type [Web3WS]
pub type Web3WSResult = anyhow::Result<Web3WS>;
