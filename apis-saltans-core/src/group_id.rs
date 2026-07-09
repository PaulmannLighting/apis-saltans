use core::num::NonZero;

/// Zigbee APS group identifier.
///
/// Valid group identifiers are non-zero values in the normal Zigbee group
/// address range. The high reserved range is intentionally rejected by
/// [`Self::new`].
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GroupId(NonZero<u16>);

impl GroupId {
    const MIN_VALUE: u16 = 0x0001;
    const MAX_VALUE: u16 = 0xFFF7;

    /// Create a group identifier from a raw 16-bit value.
    ///
    /// Returns [`None`] for `0x0000` and for values outside the valid Zigbee
    /// group identifier range.
    #[must_use]
    pub const fn new(id: u16) -> Option<Self> {
        if id >= Self::MIN_VALUE && id <= Self::MAX_VALUE {
            if let Some(id) = NonZero::new(id) {
                Some(Self(id))
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Return the raw 16-bit APS group identifier value.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        self.0.get()
    }
}

impl From<GroupId> for u16 {
    fn from(id: GroupId) -> Self {
        id.as_u16()
    }
}
