/*
    Appellation: credentials <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

pub trait ICredential {
    fn authorize(&self, signature: String) -> bool
        where
            Self: Sized;
}

#[derive(Clone, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
pub enum CredentialState {
    Authenticated(Credential),
    Unauthenticated(Credential),
}

#[derive(Clone, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
pub struct Credential {
    pub account: String,
    pub created: i64,
    pub data: Vec<String>,
}

impl Credential {
    fn constructor(account: String, created: i64, data: Vec<String>) -> Self {
        Self {
            account,
            created,
            data,
        }
    }
    pub fn new(account: String, data: Vec<String>) -> Self {
        Self::constructor(account, chrono::Utc::now().timestamp(), data)
    }
}

impl Default for Credential {
    fn default() -> Self {
        Self::new(String::new(), Vec::<String>::new())
    }
}
