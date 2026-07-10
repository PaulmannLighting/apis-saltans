use core::num::NonZero;

/// Zigbee APS group identifier.
///
/// Valid group identifiers are non-zero values in the normal Zigbee group
/// address range. The high reserved range is intentionally rejected by
/// [`Self::new`].
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(try_from = "u16", into = "u16")
)]
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

impl TryFrom<u16> for GroupId {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(value)
    }
}

impl_fmt_via_value!(GroupId, u16, |value| value.as_u16());
