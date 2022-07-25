/*
    Appellation: interface <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

/// Defines the minimum framework necessary for the system to recognize a currency
pub trait Currency {
    fn name(&self) -> String
        where
            Self: Sized;
    fn symbol(&self) -> String
        where
            Self: Sized;
}

/// Describes the two versions of currencies
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CurrencyType<C = Box<dyn Currency>> {
    Fiat(C),
    Token(C),
}

/// Describes the standard wallet interface for digital currencies
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Wallet<Dt: Clone + std::fmt::Debug> {
    pub label: String,
    pub data: Vec<Dt>,
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        assert_eq!(f(4, 2), 6)
    }
}
