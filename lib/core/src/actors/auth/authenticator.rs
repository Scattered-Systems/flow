/*
    Appellation: authenticator <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
/// Implement a secure authenticator
#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Authenticator {
    pub authorizations: scsys::Dictionary,
    pub endpoint: String,
}

impl Authenticator {
    fn constructor(authorizations: scsys::Dictionary, endpoint: String) -> Self {
        Self {
            authorizations,
            endpoint,
        }
    }
    pub fn new(authorizations: scsys::Dictionary, endpoint: String) -> Self {
        Self::constructor(authorizations, endpoint)
    }
}

impl Default for Authenticator {
    fn default() -> Self {
        Self::new(scsys::Dictionary::new(), String::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_authenticator() {
        let actual = Authenticator::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
