use core::iter::Chain;
use core::ops::Deref;

use le_stream::{FromLeStream, FromLeStreamTagged, ToLeStream};

use crate::types::tlv::{EncapsulatedGlobal, Local, Tag, Tlv, TlvVec};

/// Beacon Appendix Encapsulation TLV structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct BeaconAppendixEncapsulation {
    inner: TlvVec<Tlv<Local, EncapsulatedGlobal>>,
}

impl BeaconAppendixEncapsulation {
    /// Creates a new `BeaconAppendixEncapsulation`.
    pub fn new(inner: TlvVec<Tlv<Local, EncapsulatedGlobal>>) -> Option<Self> {
        if inner.is_empty() {
            return None;
        };

        Some(Self { inner })
    }
}

impl Tag for BeaconAppendixEncapsulation {
    const TAG: u8 = 73;

    fn size(&self) -> usize {
        self.inner.len()
    }
}

impl Deref for BeaconAppendixEncapsulation {
    type Target = TlvVec<Tlv<Local, EncapsulatedGlobal>>;

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

        let mut inner = TlvVec::new();

        for _ in 0..size {
            let Some(item) = Tlv::<Local, EncapsulatedGlobal>::from_le_stream(&mut bytes) else {
                return Ok(None);
            };

            if inner.push(item).is_err() {
                return Ok(None);
            }
        }

        Ok(Some(Self { inner }))
    }
}

impl ToLeStream for BeaconAppendixEncapsulation {
    type Iter = Chain<
        <u8 as ToLeStream>::Iter,
        <TlvVec<Tlv<Local, EncapsulatedGlobal>> as ToLeStream>::Iter,
    >;

    fn to_le_stream(self) -> Self::Iter {
        Self::TAG.to_le_stream().chain(self.inner.to_le_stream())
    }
}
