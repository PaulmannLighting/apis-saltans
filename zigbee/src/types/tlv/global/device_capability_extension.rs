use std::iter::Chain;

use le_stream::{FromLeStream, FromLeStreamTagged, ToLeStream};

use crate::types::tlv::Tag;

/// Device Capability Extension TLV
///
/// TODO: Make this bitflags, once values are known.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
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

impl FromLeStreamTagged for DeviceCapabilityExtension {
    type Tag = u8;

    fn from_le_stream_tagged<T>(length: Self::Tag, mut bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        let Some(size) = usize::from(length).checked_add(1) else {
            return Err(length);
        };

        if size != 2 {
            return Err(length);
        }

        Ok(u16::from_le_stream(&mut bytes).map(Self::new))
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
