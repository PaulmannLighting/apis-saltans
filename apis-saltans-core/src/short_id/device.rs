/// Allocated Zigbee NWK device short address.
///
/// Device short addresses exclude the coordinator address, the reserved range,
/// broadcast addresses, and the invalid-address sentinel.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Device(pub(crate) u16);

impl Device {
    pub(crate) const MIN_VALUE: u16 = 0x0001;
    pub(crate) const MAX_VALUE: u16 = 0xFFF7;

    /// Create a device short address from a raw 16-bit value.
    ///
    /// Returns [`None`] when `short_id` is outside the normal allocated-device
    /// address range.
    #[must_use]
    pub const fn new(short_id: u16) -> Option<Self> {
        if short_id >= Self::MIN_VALUE && short_id <= Self::MAX_VALUE {
            Some(Self(short_id))
        } else {
            None
        }
    }

    /// Return the raw 16-bit NWK short address value.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self.0
    }
}

impl From<Device> for u16 {
    fn from(device: Device) -> Self {
        device.0
    }
}

impl TryFrom<u16> for Device {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(value)
    }
}
