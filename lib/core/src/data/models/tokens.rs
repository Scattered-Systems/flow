/*
    Appellation: tokens <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use scsys::{Deserialize, Serialize};
pub use utils::*;

/// Defines the standard model for creating authentication tokens
#[derive(Clone, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub struct OAuthToken {
    pub access_type: Vec<String>,
    pub token: String,
}

impl OAuthToken {
    fn constructor(access_type: Vec<String>, token: String) -> Result<Self, scsys::BoxError> {
        Ok(Self { access_type, token })
    }
    pub fn new(access_type: Vec<String>, token: String) -> Self {
        match Self::constructor(access_type, token) {
            Ok(v) => v,
            Err(e) => panic!("Token Error: {}", e),
        }
    }
}

impl Default for OAuthToken {
    // Migrate the generative token to here
    fn default() -> Self {
        Self::new(Vec::<String>::new(), String::new())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Deserialize, Serialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}

impl Claims {
    fn constructor(sub: String, role: String, exp: usize) -> Self {
        Self { sub, role, exp }
    }
    pub fn new(sub: String, role: String, exp: usize) -> Self {
        Self::constructor(sub, role, exp)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Deserialize, Serialize)]
pub enum Role {
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
        match self {
            Role::User => write!(f, "User"),
            Role::Admin => write!(f, "Admin"),
        }
    }
}

impl Default for Claims {
    fn default() -> Self {
        Self::new(
            String::new(),
            String::new(),
            scsys::Utc::now()
                .checked_add_signed(chrono::Duration::seconds(60))
                .expect("Timestamp Error")
                .timestamp() as usize,
        )
    }
}

mod utils {
    use super::{Claims, Role};

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
        let actual = OAuthToken::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
