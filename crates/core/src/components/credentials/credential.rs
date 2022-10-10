/*
    Appellation: credentials <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use scsys::{BoxResult, Timestamp};

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Credential {
    pub account: String,
    pub created: Timestamp,
    pub data: Vec<String>,
}

impl Credential {
    pub fn new(account: String, data: Vec<String>) -> Self {
        Self {
            account,
            created: Timestamp::default(),
            data,
        }
    }

    pub fn save_to_file(&self, path: &str) -> BoxResult<Self> {
        crate::save_to_file(self.clone(), path)
    }
}

impl Default for Credential {
    fn default() -> Self {
        Self::new(String::new(), Vec::<String>::new())
    }
}

#[cfg(test)]
mod tests {
    use super::Credential;

    #[test]
    fn test_default_credential() {
        let actual = Credential::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
