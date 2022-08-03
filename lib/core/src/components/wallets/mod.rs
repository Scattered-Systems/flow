/*
    Appellation: wallets <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use grant::*;
pub use wallet::*;

mod grant;
mod wallet;

/// Outlines the minimum requirements for creating ERC20 tokens
pub trait ERC20Spec: Clone + std::fmt::Debug {
    fn decimals(&self) -> usize;
    fn name(&self) -> String;
    fn symbol(&self) -> String;
    fn supply(&self) -> usize;
}
