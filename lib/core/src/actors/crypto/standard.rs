/*
    Appellation: standard <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::CryptoStates;

#[derive(Clone, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
pub struct CryptoActor {
    pub state: CryptoStates,
}

impl CryptoActor {
    pub fn get_state(&self) -> CryptoStates {
        self.state.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let actual = CryptoActor {
            state: CryptoStates::default(),
        };
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
