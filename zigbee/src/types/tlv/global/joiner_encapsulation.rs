use std::iter::Chain;
use std::ops::{Deref, DerefMut};

use le_stream::{FromLeStream, FromLeStreamTagged, ToLeStream};

use crate::types::tlv::{EncapsulatedGlobal, Local, Tag, Tlv};

/// Joiner Encapsulation TLV structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct JoinerEncapsulation {
    inner: Vec<Tlv<Local, EncapsulatedGlobal>>,
}

impl JoinerEncapsulation {
    /// Creates a new `JoinerEncapsulation`.
    ///
    /// # Errors
    ///
    /// If the length of `inner` minus one cannot be represented as a `u8`, an error is returned.
    pub fn new(
        inner: Vec<Tlv<Local, EncapsulatedGlobal>>,
    ) -> Result<Self, Option<std::num::TryFromIntError>> {
        let Some(size) = inner.len().checked_sub(1) else {
            return Err(None);
        };

        u8::try_from(size).map(|_| Self { inner }).map_err(Some)
    }
}

impl Tag for JoinerEncapsulation {
    const TAG: u8 = 72;

    fn size(&self) -> usize {
        self.inner.len()
    }
}

impl Deref for JoinerEncapsulation {
    type Target = Vec<Tlv<Local, EncapsulatedGlobal>>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for JoinerEncapsulation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl FromLeStreamTagged for JoinerEncapsulation {
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

impl ToLeStream for JoinerEncapsulation {
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
