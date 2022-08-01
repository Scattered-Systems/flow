/*
    Appellation: asset <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
/// Implement a simple ERC20 compatible asset specification
pub trait ERC20Spec {
    fn decimals(&self) -> usize;
    fn name(&self) -> String;
    fn supply(&self) -> usize;
    fn symbol(&self) -> String;
}
