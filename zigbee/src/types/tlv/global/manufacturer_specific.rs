use le_stream::FromLeStream;

use crate::types::tlv::tlv::Tlv;

/// Manufacturer Specific TLV global.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream)]
pub struct ManufacturerSpecific {
    manufacturer_id: u16,
    data: Vec<u8>,
}

impl ManufacturerSpecific {
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

impl Tlv for ManufacturerSpecific {
    const TAG: u8 = 64;
}
