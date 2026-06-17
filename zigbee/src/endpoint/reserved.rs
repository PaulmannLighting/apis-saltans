use core::fmt;
use core::fmt::{Display, LowerHex, UpperHex};

/// A Zigbee reserved endpoint ID.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
#[repr(transparent)]
pub struct Reserved(pub(crate) u8);

impl Reserved {
    /// The minimum valid reserved endpoint ID.
    pub const MIN: u8 = 241;

    /// The maximum valid reserved endpoint ID.
    pub const MAX: u8 = 254;

    /// Create a new `Reserved` endpoint ID if the given ID is valid.
    #[must_use]
    pub const fn new(id: u8) -> Option<Self> {
        if id >= Self::MIN && id <= Self::MAX {
            Some(Self(id))
        } else {
            None
        }
    }

    /// Create a new `Reserved` endpoint ID, clamping the given ID to the valid range.
    #[must_use]
    pub fn new_clamped(id: u8) -> Self {
        Self(id.clamp(Self::MIN, Self::MAX))
    }

    /// Create a new `Reserved` endpoint ID without checking validity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the given ID is within the valid range (241..=254).
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_unchecked(id: u8) -> Self {
        Self(id)
    }
}

impl Display for Reserved {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl LowerHex for Reserved {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        LowerHex::fmt(&self.0, f)
    }
}

impl UpperHex for Reserved {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        UpperHex::fmt(&self.0, f)
    }
}

impl From<Reserved> for u8 {
    fn from(endpoint: Reserved) -> Self {
        endpoint.0
    }
}

impl TryFrom<u8> for Reserved {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(value)
    }
}
