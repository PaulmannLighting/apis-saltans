use le_stream::FromLeStream;

use crate::types::tlv::Tag;

/// Symmetric Passphrase TLV structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream)]
pub struct SymmetricPassphrase {
    passphrase: [u8; 16],
}

impl SymmetricPassphrase {
    /// Get the symmetric passphrase.
    #[must_use]
    pub const fn passphrase(&self) -> &[u8; 16] {
        &self.passphrase
    }
}

impl Tag for SymmetricPassphrase {
    const TAG: u8 = 69;
}

impl From<SymmetricPassphrase> for [u8; 16] {
    fn from(value: SymmetricPassphrase) -> Self {
        value.passphrase
    }
}

impl From<[u8; 16]> for SymmetricPassphrase {
    fn from(passphrase: [u8; 16]) -> Self {
        Self { passphrase }
    }
}
