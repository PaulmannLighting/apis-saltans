use std::iter::Chain;
use std::num::TryFromIntError;

use le_stream::{FromLeStream, ToLeStream};

use crate::types::tlv::Tag;

/// Manufacturer Specific TLV global.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream)]
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
