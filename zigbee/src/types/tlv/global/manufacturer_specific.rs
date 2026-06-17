use core::iter::Chain;

use le_stream::{FromLeStream, FromLeStreamTagged, ToLeStream};

use crate::types::tlv::Tag;

/// Manufacturer specific data.
pub type Data = heapless::Vec<u8, { u8::MAX as usize }, u8>;

/// Manufacturer Specific TLV global.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct ManufacturerSpecific {
    manufacturer_id: u16,
    data: heapless::Vec<u8, { u8::MAX as usize }, u8>,
}

impl ManufacturerSpecific {
    /// Create a new `ManufacturerSpecific`.
    pub fn new(manufacturer_id: u16, data: Data) -> Option<Self> {
        if data.is_empty() {
            return None;
        };

        Some(Self {
            manufacturer_id,
            data,
        })
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

        let mut data = Data::new();

        for _ in 0..size {
            let Some(byte) = bytes.next() else {
                return Ok(None);
            };

            if data.push(byte).is_err() {
                return Ok(None);
            }
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
        <Data as IntoIterator>::IntoIter,
    >;

    fn to_le_stream(self) -> Self::Iter {
        Self::TAG
            .to_le_stream()
            .chain(self.serialized_size().to_le_stream())
            .chain(self.manufacturer_id.to_le_stream())
            .chain(self.data.into_iter())
    }
}
