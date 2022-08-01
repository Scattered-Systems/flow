/*
    Appellation: interface <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct AccessGrant {
    pub access: scsys::Dictionary,
    pub grant: Vec<String>,
    pub timestamp: i64,
}

impl AccessGrant {
    pub fn new(access: scsys::Dictionary, grant: Vec<String>, timestamp: i64) -> Self {
        Self {
            access,
            grant,
            timestamp,
        }
    }
}

impl Default for AccessGrant {
    fn default() -> Self {
        Self::new(
            scsys::Dictionary::new(),
            Vec::<String>::with_capacity(12),
            scsys::Temporal::now().timestamp(),
        )
    }
}

/// Describes the standard wallets interface for digital currencies
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Wallet {
    pub id: scsys::ObjectId,
    pub label: String,
}

impl Wallet {
    fn constructor(id: scsys::ObjectId, label: String) -> Self {
        Self { id, label }
    }
    pub fn new(label: String) -> Self {
        Self::constructor(scsys::ObjectId::new(), label)
    }
}

impl Default for Wallet {
    fn default() -> Self {
        Self::new(String::new())
    }
}
