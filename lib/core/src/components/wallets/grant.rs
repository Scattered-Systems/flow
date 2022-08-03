/*
    Appellation: grant <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
/// Encapsulates a group of possible access grants
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum Grant {
    Alt([String; 16]),
    Std([String; 12]),
}

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
