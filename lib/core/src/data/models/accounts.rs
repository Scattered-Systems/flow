/*
    Appellation: accounts <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use scsys::{Deserialize, Serialize};

/// Defines a Web3 Account
#[derive(Clone, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub struct Web3Account {
    pub address: String,
    pub balance: usize,
    pub ensname: String,
}

impl Web3Account {
    fn constructor(address: String, balance: usize, ensname: String) -> Self {
        Self {
            address,
            balance,
            ensname,
        }
    }
    pub fn new(address: String, balance: usize, ensname: String) -> Self {
        Self::constructor(address, balance, ensname)
    }
    pub fn get_balance(&self) -> usize {
        todo!()
    }
}

impl Default for Web3Account {
    fn default() -> Self {
        Self::new(String::new(), 0, String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account() {
        let actual = Web3Account::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
