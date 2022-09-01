/*
    Appellation: tokens <module>
    Contributors: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct TokenModel {
    pub access_token: String,
    pub token_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl TokenModel {
    pub fn new(access_token: String, token_type: String, username: Option<String>) -> Self {
        Self {
            access_token,
            token_type,
            username,
        }
    }
}

impl Default for TokenModel {
    fn default() -> Self {
        Self::new(String::new(), String::new(), Some(String::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::TokenModel;

    #[test]
    fn test_token_model() {
        let actual = TokenModel::default();
        let expected: TokenModel = actual.clone();
        assert_eq!(actual, expected)
    }
}
