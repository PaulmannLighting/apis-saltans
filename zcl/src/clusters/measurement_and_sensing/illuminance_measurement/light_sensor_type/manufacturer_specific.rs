use core::ops::RangeInclusive;

use le_stream::{FromLeStream, ToLeStream};

/// Manufacturer Specific Light Sensor Type.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, FromLeStream, ToLeStream)]
pub struct ManufacturerSpecific(u8);

impl ManufacturerSpecific {
    /// Valid range of manufacturer specific light sensor types.
    pub const VALID_RANGE: RangeInclusive<u8> = 0x40..=0xFE;
}

impl ManufacturerSpecific {
    /// Return the raw value of the manufacturer specific light sensor type.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self.0
    }
}

impl From<ManufacturerSpecific> for u8 {
    fn from(value: ManufacturerSpecific) -> Self {
        value.as_u8()
    }
}

impl TryFrom<u8> for ManufacturerSpecific {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if Self::VALID_RANGE.contains(&value) {
            Ok(Self(value))
        } else {
            Err(value)
        }
    }
}
