/*
    Appellation: id <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # Identity
//!
//! The `id` module provides a set of utilities for working with identity and identity-like types.
pub use self::identity::*;

pub(crate) mod identity;

pub mod atomic;

pub trait Identity {
    type Id;

    fn get_id(&self) -> Self::Id;
}

pub(crate) mod prelude {
    pub use super::atomic::AtomicId;
    pub use super::Identity;
}

#[cfg(test)]
mod tests {
    use super::atomic::AtomicId;

    #[test]
    fn test_atomic() {
        let id = AtomicId::new();
        assert_eq!(id.get(), 1);
        assert_eq!(*id.next(), 2);
        let id = AtomicId::new();
        assert_eq!(id.get(), 3);
    }
}
