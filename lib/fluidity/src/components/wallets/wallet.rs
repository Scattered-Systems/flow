/*
    Appellation: interface <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/



/// Describes the standard wallets interface for digital currencies
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Wallet<Dt: Clone + std::fmt::Debug> {
    pub label: String,
    pub data: Vec<Dt>,
}