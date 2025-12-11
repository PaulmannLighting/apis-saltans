use std::iter::Chain;

use le_stream::{FromLeStream, FromLeStreamTagged, ToLeStream};

use crate::types::tlv::Tag;

/// Symmetric Passphrase TLV structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct SymmetricPassphrase {
    passphrase: [u8; 16],
}

impl SymmetricPassphrase {
    /// Create a new `SymmetricPassphrase`.
    #[must_use]
    pub const fn new(passphrase: [u8; 16]) -> Self {
        Self { passphrase }
    }

    /// Get the symmetric passphrase.
    #[must_use]
    pub const fn passphrase(&self) -> &[u8; 16] {
        &self.passphrase
    }
}

impl Tag for SymmetricPassphrase {
    const TAG: u8 = 69;

    fn size(&self) -> usize {
        16
    }
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

impl FromLeStreamTagged for SymmetricPassphrase {
    type Tag = u8;

    fn from_le_stream_tagged<T>(length: Self::Tag, bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        let Some(size) = usize::from(length).checked_add(1) else {
            return Err(length);
        };

        if size != 16 {
            return Err(length);
        }

        Ok(<[u8; 16]>::from_le_stream(bytes).map(Self::new))
    }
}

impl ToLeStream for SymmetricPassphrase {
    type Iter = Chain<
        Chain<<u8 as ToLeStream>::Iter, <u8 as ToLeStream>::Iter>,
        <[u8; 16] as ToLeStream>::Iter,
    >;

    fn to_le_stream(self) -> Self::Iter {
        Self::TAG
            .to_le_stream()
            .chain(self.serialized_size().to_le_stream())
            .chain(self.passphrase.to_le_stream())
    }
}
