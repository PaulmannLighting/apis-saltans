use le_stream::FromLeStream;

use crate::types::tlv::Tag;

/// Device Capability Extension TLV
///
/// TODO: Make this bitflags, once values are known.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream)]
pub struct DeviceCapabilityExtension {
    bitmask: u16,
}

impl DeviceCapabilityExtension {
    /// Get the bitmask.
    #[must_use]
    pub const fn bitmask(self) -> u16 {
        self.bitmask
    }
}

impl Tag for DeviceCapabilityExtension {
    const TAG: u8 = 76;
}

impl From<DeviceCapabilityExtension> for u16 {
    fn from(value: DeviceCapabilityExtension) -> Self {
        value.bitmask
    }
}

impl From<u16> for DeviceCapabilityExtension {
    fn from(value: u16) -> Self {
        Self { bitmask: value }
    }
}
