use std::iter::Chain;

use le_stream::{FromLeStream, ToLeStream};

use crate::types::tlv::Tag;

/// Device Capability Extension TLV
///
/// TODO: Make this bitflags, once values are known.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream)]
pub struct DeviceCapabilityExtension {
    bitmask: u16,
}

impl DeviceCapabilityExtension {
    /// Create a new `DeviceCapabilityExtension`.
    #[must_use]
    pub const fn new(bitmask: u16) -> Self {
        Self { bitmask }
    }
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

    fn size(&self) -> usize {
        2
    }
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

impl ToLeStream for DeviceCapabilityExtension {
    type Iter =
        Chain<Chain<<u8 as ToLeStream>::Iter, <u8 as ToLeStream>::Iter>, <u16 as ToLeStream>::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        Self::TAG
            .to_le_stream()
            .chain(self.serialized_size().to_le_stream())
            .chain(self.bitmask.to_le_stream())
    }
}
