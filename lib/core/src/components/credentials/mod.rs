/*
    Appellation: credentials <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use self::{credential::*, utils::*};

mod credential;

/// Implements the credential specification for the systems to properly authenticate actors
pub trait CredentialSpec: Clone + PartialEq + std::fmt::Debug {
    fn get(&self) -> scsys::BoxResult<Self>
        where
            Self: Sized,
    {
        Ok(self.clone())
    }
    fn authorize(&self, signature: String) -> bool
        where
            Self: Sized;
}

#[derive(Clone, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
pub enum CredentialState<T = Credential> {
    Authenticated(T),
    Unauthenticated(T),
}

mod utils {}
