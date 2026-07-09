/// Reserved Zigbee NWK short address.
///
/// These values are not valid allocated device addresses or broadcast
/// addresses, but are represented explicitly so raw short IDs can be classified
/// without losing information.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Reserved(pub(crate) u16);

impl Reserved {
    pub(crate) const MIN_VALUE: u16 = 0xFFF8;
    pub(crate) const MAX_VALUE: u16 = 0xFFFA;
}

impl Reserved {
    /// Return the raw 16-bit reserved short address value.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self.0
    }
}

impl From<Reserved> for u16 {
    fn from(reserved: Reserved) -> Self {
        reserved.0
    }
}
