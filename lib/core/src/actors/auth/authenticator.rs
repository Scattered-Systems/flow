/*
    Appellation: authenticator <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

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
}
