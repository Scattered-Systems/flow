/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use libp2p::identity::{Keypair, PublicKey};
use libp2p::PeerId;

/// [FromPeerId] describes the conversion of a [PeerId] into some type
pub trait FromPeerId {
    fn from_pid(pid: PeerId) -> Self;
}

impl<T> FromPeerId for T
where
    T: From<PeerId>,
{
    fn from_pid(pid: PeerId) -> Self {
        Self::from(pid)
    }
}

/// [FromPublicKey] describes the conversion of a [PublicKey] into some type
pub trait FromPublicKey {
    fn from_pk(pk: PublicKey) -> Self;
}

impl<T> FromPublicKey for T
where
    T: From<PublicKey>,
{
    fn from_pk(pk: PublicKey) -> Self {
        Self::from(pk)
    }
}

/// [FromKeypair] describes the conversion of a [Keypair] into some type
pub trait FromKeypair {
    fn from_kp(kp: Keypair) -> Self;
}

impl<T> FromKeypair for T
where
    T: From<Keypair>,
{
    fn from_kp(kp: Keypair) -> Self {
        Self::from(kp)
    }
}
