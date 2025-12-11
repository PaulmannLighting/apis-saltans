use std::iter::Chain;
use std::num::TryFromIntError;
use std::ops::Deref;

use le_stream::{FromLeStream, FromLeStreamTagged, ToLeStream};

use crate::types::tlv::{EncapsulatedGlobal, Local, Tag, Tlv};

/// Beacon Appendix Encapsulation TLV structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct BeaconAppendixEncapsulation {
    inner: Vec<Tlv<Local, EncapsulatedGlobal>>,
}

impl BeaconAppendixEncapsulation {
    /// Creates a new `BeaconAppendixEncapsulation`.
    ///
    /// # Errors
    ///
    /// If the length of `inner` minus one cannot be represented as a `u8`, an error is returned.
    pub fn new(
        inner: Vec<Tlv<Local, EncapsulatedGlobal>>,
    ) -> Result<Self, Option<TryFromIntError>> {
        let Some(size) = inner.len().checked_sub(1) else {
            return Err(None);
        };

        u8::try_from(size).map(|_| Self { inner }).map_err(Some)
    }
}

impl Tag for BeaconAppendixEncapsulation {
    const TAG: u8 = 73;

    fn size(&self) -> usize {
        self.inner.len()
    }
}

impl Deref for BeaconAppendixEncapsulation {
    type Target = Vec<Tlv<Local, EncapsulatedGlobal>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl FromLeStreamTagged for BeaconAppendixEncapsulation {
    type Tag = u8;

    fn from_le_stream_tagged<T>(length: Self::Tag, mut bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        let Some(size) = usize::from(length).checked_add(1) else {
            return Err(length);
        };

        let mut inner = Vec::with_capacity(size);

        for _ in 0..size {
            let Some(item) = Tlv::<Local, EncapsulatedGlobal>::from_le_stream(&mut bytes) else {
                return Ok(None);
            };
            inner.push(item);
        }

        Ok(Some(Self { inner }))
    }
}

impl ToLeStream for BeaconAppendixEncapsulation {
    type Iter = Chain<
        Chain<<u8 as ToLeStream>::Iter, <u8 as ToLeStream>::Iter>,
        <Vec<Tlv<Local, EncapsulatedGlobal>> as ToLeStream>::Iter,
    >;

    fn to_le_stream(self) -> Self::Iter {
        Self::TAG
            .to_le_stream()
            .chain(self.serialized_size().to_le_stream())
            .chain(self.inner.to_le_stream())
    }
}
