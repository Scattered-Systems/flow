/*
    Appellation: states <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
pub use crypto::*;
pub use state::*;

mod state;

pub trait Stateful<S> {
    fn state(&self) -> S
        where
            Self: Sized;
}

mod crypto {
    #[derive(Clone, Copy, Debug, Hash, PartialEq, scsys::Deserialize, scsys::Serialize)]
    pub enum CryptoStates {
        Insecure,
        Secure,
        Securing,
    }

    impl Default for CryptoStates {
        fn default() -> Self {
            Self::Secure
        }
    }
}
