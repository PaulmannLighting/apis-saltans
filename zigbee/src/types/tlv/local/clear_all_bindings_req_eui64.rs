use std::iter::Chain;
use std::num::TryFromIntError;
use std::ops::Deref;

use le_stream::{FromLeStream, FromLeStreamTagged, ToLeStream};
use macaddr::MacAddr8;

use crate::types::tlv::Tag;

/// Clear All Bindings Request EUI64 List.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ClearAllBindingsReqEui64 {
    eui64s: Vec<MacAddr8>,
}

impl ClearAllBindingsReqEui64 {
    /// Creates a new `ClearAllBindingsReqEui64`.
    ///
    /// # Errors
    ///
    /// Returns an error if the length of `eui64s` cannot be represented as `u8`.
    pub fn new(eui64s: Vec<MacAddr8>) -> Result<Self, TryFromIntError> {
        u8::try_from(eui64s.len()).map(|_| Self { eui64s })
    }

    /// Returns a reference to the EUI64 list.
    #[must_use]
    pub fn eui64s(&self) -> &[MacAddr8] {
        &self.eui64s
    }
}

impl Tag for ClearAllBindingsReqEui64 {
    const TAG: u8 = 0x00;

    fn size(&self) -> usize {
        self.eui64s.len()
    }
}

impl Deref for ClearAllBindingsReqEui64 {
    type Target = [MacAddr8];

    fn deref(&self) -> &Self::Target {
        &self.eui64s
    }
}

impl FromLeStreamTagged for ClearAllBindingsReqEui64 {
    type Tag = u8;

    fn from_le_stream_tagged<T>(length: Self::Tag, mut bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        let Some(size) = usize::from(length).checked_add(1) else {
            return Err(length);
        };

        let mut eui64s = Vec::with_capacity(size);

        for _ in 0..size {
            let Some(eui64) = MacAddr8::from_le_stream(&mut bytes) else {
                return Ok(None);
            };
            eui64s.push(eui64);
        }

        Ok(Some(Self { eui64s }))
    }
}

impl ToLeStream for ClearAllBindingsReqEui64 {
    type Iter = Chain<
        Chain<<u8 as ToLeStream>::Iter, <u8 as ToLeStream>::Iter>,
        <Vec<MacAddr8> as ToLeStream>::Iter,
    >;

    fn to_le_stream(self) -> Self::Iter {
        Self::TAG
            .to_le_stream()
            .chain(self.serialized_size().to_le_stream())
            .chain(self.eui64s.to_le_stream())
    }
}
