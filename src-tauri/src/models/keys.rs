//! Key-related models.

use generic_array::{typenum::U32, GenericArray};

pub type RatchetKey = GenericArray<u8, U32>;

/// Keys used by Double Ratchet algorithm.
#[derive(Clone, Debug)]
pub struct Ratchet {
    /// Used to generate other keys.
    pub root_key: RatchetKey,
    /// Derive successive message keys.
    pub chain_key: RatchetKey,
    /// Unique key used to encrypt or decrypt a message.
    pub message_key: RatchetKey,
}

#[derive(Clone, Default)]
pub struct Keys {
    pub public_key: Option<x25519_dalek::PublicKey>,
    pub private_key: Option<x25519_dalek::StaticSecret>,
    /// Keys related to Double Ratchet algorithm.
    pub ratchet: Option<Ratchet>,
}
