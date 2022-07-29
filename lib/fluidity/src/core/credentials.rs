/*
    Appellation: credentials <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum CredentialState {
    Authenticated(Credential),
    Unauthenticated(Credential),
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Credential {
    pub id: scsys::Id,
    pub access_type: Vec<String>,
    pub timestamp: i64,
    pub token: String,
}

impl Credential {
    fn constructor(
        id: scsys::Id,
        access_type: Vec<String>,
        timestamp: i64,
        token: String,
    ) -> Result<Self, scsys::BoxError> {
        Ok(Self {
            id,
            access_type,
            timestamp,
            token,
        })
    }
    pub fn new(access_type: Vec<String>, token: String) -> Self {
        match Self::constructor(
            scsys::Id::Obj(bson::oid::ObjectId::new()),
            access_type,
            scsys::BlockTz::now().timestamp(),
            token,
        ) {
            Ok(v) => v,
            Err(e) => panic!("Credential Error: {}", e),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct CredentialStore;
