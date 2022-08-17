/*
    Appellation: utils <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use rand::Rng;
use secp256k1::PublicKey;
use std::io::Read;
use web3::{
    signing::keccak256,
    types::{Address, U256},
};

/// From the given path, collect the file lines into a [Vec<String>]
pub fn extract_file_from_path(path: &str) -> Vec<String> {
    let mut file = match std::fs::File::open(std::path::Path::new(path.clone())) {
        Ok(f) => f,
        Err(e) => panic!("File Error: {}", e),
    };
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("File Error");
    buffer.split("\n").map(|s: &str| s.to_string()).collect()
}

/// Create a random set of elements from a source via index
pub fn generate_collection_from_reference(reference: Vec<String>, size: usize) -> Vec<String> {
        let mut cache = Vec::<String>::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            let random_index = rng.gen_range(0..reference.clone().len());
            cache.push(reference[random_index].clone())
        }
        cache
}

/// Quickly generate a random, secure string
pub fn generate_random_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

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

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        let actual = f(4, 4);
        let expected: usize = 8;
        assert_eq!(actual, expected)
    }
}
