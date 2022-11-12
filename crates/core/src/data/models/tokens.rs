/*
    Appellation: tokens <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::utils::*;
use scsys::prelude::{chrono, Timestamp};
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Tokens {
    Token(String),
    OAuth2 {
        access_token: String,
        token_type: String,
        username: Option<String>,
    },
}

/// Defines the standard model for creating authentication tokens
#[derive(Clone, Debug, Default, Deserialize, Hash, PartialEq, Serialize)]
pub struct Token {
    pub access_type: Vec<String>,
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl Token {
    pub fn new(access_type: Vec<String>, token: String, username: Option<String>) -> Self {
        Self {
            access_type,
            token,
            username,
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

impl Claims {
    pub fn new(sub: String, role: String, exp: usize) -> Self {
        Self { sub, role, exp }
    }
}

#[derive(
    Clone, Debug, Default, Deserialize, EnumString, EnumVariantNames, Eq, Hash, PartialEq, Serialize,
)]
pub enum Role {
    #[default]
    User,
    Admin,
}

impl Role {
    pub fn from_str(role: &str) -> Role {
        match role {
            "Admin" => Role::Admin,
            _ => Role::User,
        }
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub(crate) mod utils {
    use super::{Claims, Role};
    use scsys::prelude::chrono;

    pub fn create_jwt(
        uid: &str,
        role: &Role,
        secret: String,
    ) -> jsonwebtoken::errors::Result<String> {
        let expiration = chrono::Utc::now()
            .checked_add_signed(chrono::Duration::seconds(60))
            .expect("DateTime Error")
            .timestamp() as usize;

        let claims = Claims {
            sub: uid.to_owned(),
            role: role.to_string(),
            exp: expiration as usize,
        };
        let header = jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512);
        jsonwebtoken::encode(
            &header,
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokens() {
        let actual = Token::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
