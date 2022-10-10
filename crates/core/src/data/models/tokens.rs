/*
    Appellation: tokens <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::utils::*;

pub enum Token {
    Token(String),
    OAuth2 {
        access_token: String,
        token_type: String,
        username: Option<String>,
    },
}

/// Defines the standard model for creating authentication tokens
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Tokens {
    pub access_type: Vec<String>,
    pub token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl Tokens {
    pub fn new(access_type: Vec<String>, token: String, username: Option<String>) -> Self {
        Self {
            access_type,
            token,
            username,
        }
    }
}

impl Default for Tokens {
    // Migrate the generative token to here
    fn default() -> Self {
        Self::new(Vec::<String>::new(), String::new(), None)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
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

#[derive(Clone, Copy, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
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
            scsys::Timestamp::now()
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
        let actual = Tokens::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
