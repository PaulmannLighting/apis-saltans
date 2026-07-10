use le_stream::{FromLeStream, ToLeStream};

use crate::types::tlv::Tag;

/// Manufacturer specific data.
pub type Data = heapless::Vec<u8, { u8::MAX as usize }, u8>;

/// Manufacturer Specific TLV global.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct ManufacturerSpecific {
    manufacturer_id: u16,
    data: heapless::Vec<u8, { u8::MAX as usize }, u8>,
}

impl ManufacturerSpecific {
    /// Create a new `ManufacturerSpecific`.
    #[must_use]
    pub fn new(manufacturer_id: u16, data: Data) -> Option<Self> {
        if data.is_empty() {
            return None;
        }

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
}
