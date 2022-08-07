/*
    Appellation: grant <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::PATH_TO_BIP0039_DATA;

use rand::Rng;
use std::io::Read;

/// Implement the BIP0039 standard, defaulting searching for the file at the project root
#[derive(Clone, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
pub struct BIP0039 {
    pub data: Vec<String>,
}

impl BIP0039 {
    fn constructor(data: Vec<String>) -> Self {
        Self { data }
    }
    pub fn from_file(path: &str) -> Self {
        Self::constructor(extract_file_from_path(path))
    }
}

impl Default for BIP0039 {
    fn default() -> Self {
        Self::constructor(extract_file_from_path(PATH_TO_BIP0039_DATA))
    }
}

#[derive(Clone, Debug, PartialEq, scsys::Deserialize, scsys::Serialize)]
pub struct AccessGrant {
    pub grant: String,
    pub timestamp: scsys::Timestamp,
}

impl AccessGrant {
    fn constructor(grant: String, timestamp: scsys::Timestamp) -> Self {
        Self { grant, timestamp }
    }
    pub fn generator(size: usize) -> String {
        generate_access_grant(size).join(" ")
    }
    pub fn new(grant: String) -> Self {
        Self::constructor(grant, scsys::Timestamp::new())
    }
}

impl Default for AccessGrant {
    fn default() -> Self {
        Self::new(Self::generator(12))
    }
}

/// Quickly create randomly generated access grants for users
fn generate_access_grant(size: usize) -> Vec<String> {
    let source: BIP0039 = BIP0039::default();
    let options = source.data;
    let mut cache = Vec::<String>::with_capacity(size);
    let mut rng = rand::thread_rng();
    for _ in 0..size {
        let random_index = rng.gen_range(0..options.clone().len());
        cache.push(options[random_index].clone())
    }
    cache
}

fn extract_file_from_path(path: &str) -> Vec<String> {
    let mut file = match std::fs::File::open(std::path::Path::new(path.clone())) {
        Ok(f) => f,
        Err(e) => panic!("File Error: {}", e),
    };
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).expect("File Error");
    buffer.split("\n").map(|s: &str| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bip0039() {
        assert_eq!(
            BIP0039::default().data,
            extract_file_from_path(PATH_TO_BIP0039_DATA)
        )
    }

    #[test]
    fn test_access_grant() {
        let actual = AccessGrant::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
