/*
    Appellation: signals <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub mod power;
pub mod proceed;

pub(crate) mod prelude {
    pub use super::power::Power;
    pub use super::proceed::Proceed;
}

#[cfg(test)]
mod tests {}
