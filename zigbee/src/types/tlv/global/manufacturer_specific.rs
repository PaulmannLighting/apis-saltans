use std::iter::Chain;
use std::num::TryFromIntError;

use le_stream::{FromLeStream, FromLeStreamTagged, ToLeStream};

use crate::types::tlv::Tag;

/// Manufacturer Specific TLV global.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ManufacturerSpecific {
    manufacturer_id: u16,
    data: Vec<u8>,
}

impl ManufacturerSpecific {
    /// Create a new `ManufacturerSpecific`.
    ///
    /// # Errors
    ///
    /// If the length of `data` minus two cannot be represented as a `u8`, an error is returned.
    pub fn new(manufacturer_id: u16, data: Vec<u8>) -> Result<Self, Option<TryFromIntError>> {
        let Some(len) = 2usize
            .checked_add(data.len())
            .and_then(|len| len.checked_sub(1))
        else {
            return Err(None);
        };

        u8::try_from(len)
            .map(|_| Self {
                manufacturer_id,
                data,
            })
            .map_err(Some)
    }

    /// Get the manufacturer ID.
    #[must_use]
    pub const fn manufacturer_id(&self) -> u16 {
        self.manufacturer_id
    }

    /// Get the manufacturer specific data.
    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Tag for ManufacturerSpecific {
    const TAG: u8 = 64;

    fn size(&self) -> usize {
        2 + self.data.len()
    }
}

impl FromLeStreamTagged for ManufacturerSpecific {
    type Tag = u8;

    fn from_le_stream_tagged<T>(length: Self::Tag, mut bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        let Some(size) = usize::from(length).checked_add(1) else {
            return Err(length);
        };

        let Some(size) = size.checked_sub(2) else {
            return Err(length);
        };

        let Some(manufacturer_id) = u16::from_le_stream(&mut bytes) else {
            return Ok(None);
        };

        let mut data = Vec::with_capacity(size);

        for _ in 0..size {
            let Some(byte) = bytes.next() else {
                return Ok(None);
            };

            data.push(byte);
        }

        Ok(Some(Self {
            manufacturer_id,
            data,
        }))
    }
}

impl ToLeStream for ManufacturerSpecific {
    type Iter = Chain<
        Chain<Chain<<u8 as ToLeStream>::Iter, <u8 as ToLeStream>::Iter>, <u16 as ToLeStream>::Iter>,
        <Vec<u8> as ToLeStream>::Iter,
    >;

    fn to_le_stream(self) -> Self::Iter {
        Self::TAG
            .to_le_stream()
            .chain(self.serialized_size().to_le_stream())
            .chain(self.manufacturer_id.to_le_stream())
            .chain(self.data.to_le_stream())
    }
}
