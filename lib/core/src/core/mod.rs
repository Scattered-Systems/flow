/*
    Appellation: core <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{clients::*, crypto::*, primitives::*, states::*, utils::*};

mod clients;
mod crypto;
mod primitives;
mod states;

mod utils {
    use secp256k1::PublicKey;
    use web3::{
        signing::keccak256,
        types::{Address, U256},
    };

    /// Resolve a public key to the correct Ethereum Account [web3::types::Address]
    pub fn public_key_address(public_key: &PublicKey) -> Address {
        let public_key = public_key.serialize_uncompressed();
        debug_assert_eq!(public_key[0], 0x04);
        let hash = keccak256(&public_key[1..]);
        Address::from_slice(&hash[12..])
    }

    /// Save a serde enabled data-structure to a file
    pub fn save_to_file<'a, T: Clone + scsys::Deserialize<'a> + scsys::Serialize>(
        data: T,
        path: &str,
    ) -> scsys::BoxResult<T> {
        let file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(path)?;
        let buf_writer = std::io::BufWriter::new(file);
        serde_json::to_writer_pretty(buf_writer, &data)?;
        Ok(data.clone())
    }

    /// Convert wei to eth
    pub fn wei_to_eth(wei_val: U256) -> f64 {
        let res = wei_val.as_u128() as f64;
        res / 1_000_000_000_000_000_000.0
    }
}
