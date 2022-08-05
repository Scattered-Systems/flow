/*
    Appellation: grant <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use rand::Rng;
use scsys::{Deserialize, Serialize};
use std::io::Read;

/// Define the default filepath for locating the BIP0039 english text file
pub const PATH_TO_BIP0039_DATA: &str = "../../.artifacts/data/BIP0039/english.txt";

#[derive(Clone, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub struct BIP0039 {
    pub data: Vec<String>,
}

impl BIP0039 {
    fn constructor(data: Vec<String>) -> Self {
        Self { data }
    }
    pub fn from_file(filepath: &str) -> Self {
        let mut file = match std::fs::File::open(std::path::Path::new(filepath)) {
            Ok(f) => f,
            Err(e) => panic!("File Error: {}", e),
        };
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents)
            .ok()
            .expect("failed to read!");
        let data: Vec<String> = file_contents
            .split("\n")
            .map(|s: &str| s.to_string())
            .collect();
        Self::constructor(data)
    }
}

impl Default for BIP0039 {
    fn default() -> Self {
        Self::from_file(PATH_TO_BIP0039_DATA)
    }
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct AccessGrant {
    pub access: scsys::Dictionary,
    pub grant: Vec<String>,
    pub timestamp: i64,
}

impl AccessGrant {
    pub fn generate_grant(size: usize) -> Vec<String> {
        generate_access_grant(size)
    }
    pub fn new(access: scsys::Dictionary, grant: Vec<String>, timestamp: i64) -> Self {
        Self {
            access,
            grant,
            timestamp,
        }
    }
}

impl Default for AccessGrant {
    fn default() -> Self {
        Self::new(
            scsys::Dictionary::new(),
            Self::generate_grant(12),
            scsys::Temporal::now().timestamp(),
        )
    }
}

/// Quickly create randomly generated access grants for users
pub fn generate_access_grant(size: usize) -> Vec<String> {
    let options = BIP0039::default().data;
    let mut cache = Vec::<String>::with_capacity(size);
    let mut rng = rand::thread_rng();
    for _ in 0..size {
        let random_index = rng.gen_range(0..options.clone().len());
        cache.push(options[random_index].clone())
    }
    cache
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bip0039() {
        let actual = BIP0039::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_access_grant() {
        let actual = AccessGrant::default();
        let expected = actual.clone();
        println!("{:#?}", actual.clone());
        assert_eq!("", "")
    }
}
