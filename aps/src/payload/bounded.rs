use core::ops::Deref;

use heapless::Vec;
use le_stream::{FromLeStream, ToLeStream};

use crate::{CapacityError, CapacityKind, PAYLOAD_CAPACITY};

/// Owned APS bytes with fixed capacity and unprefixed wire encoding.
///
/// With `alloc`, the crate's `Payload` name instead denotes `bytes::Bytes`.
/// Cloning this bounded representation copies its bytes.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Payload(Vec<u8, PAYLOAD_CAPACITY>);

impl Payload {
    /// Create an empty payload.
    #[must_use]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    /// Collect unprefixed payload bytes without truncating on overflow.
    ///
    /// # Errors
    /// Returns a capacity error at the first byte beyond the configured maximum.
    pub fn try_from_iter<T>(bytes: T) -> Result<Self, CapacityError>
    where
        T: IntoIterator<Item = u8>,
    {
        let mut result = Self::new();
        for byte in bytes {
            result.0.push(byte).map_err(|_| CapacityError {
                kind: CapacityKind::Payload,
                required: PAYLOAD_CAPACITY + 1,
                capacity: PAYLOAD_CAPACITY,
            })?;
        }
        Ok(result)
    }
}

impl TryFrom<&[u8]> for Payload {
    type Error = CapacityError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        Vec::from_slice(bytes).map(Self).map_err(|_| CapacityError {
            kind: CapacityKind::Payload,
            required: bytes.len(),
            capacity: PAYLOAD_CAPACITY,
        })
    }
}

impl AsRef<[u8]> for Payload {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Deref for Payload {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl IntoIterator for Payload {
    type Item = u8;
    type IntoIter = <Vec<u8, PAYLOAD_CAPACITY> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl ToLeStream for Payload {
    type Iter = <Self as IntoIterator>::IntoIter;

    fn to_le_stream(self) -> Self::Iter {
        self.into_iter()
    }
}

impl FromLeStream for Payload {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        Self::try_from_iter(bytes).ok()
    }
}
