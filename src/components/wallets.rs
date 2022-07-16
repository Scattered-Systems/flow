/*
    Appellation: wallets
    Context: 
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct WalletAccount {
    pub id: crate::Identity,
    pub key: String,
    pub label: String,
    pub timestamp: i64,
    pub data: Vec<String>,
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Wallet {
    pub id: crate::Identity,
    pub key: String,
    pub label: String,
    pub timestamp: i64,
    pub data: Vec<WalletAccount>,
}