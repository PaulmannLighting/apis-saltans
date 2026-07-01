use core::fmt;
use core::fmt::{Display, LowerHex, UpperHex};
use core::ops::RangeInclusive;

/// A Zigbee application endpoint ID.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
#[repr(transparent)]
pub struct Application(pub(super) u8);

impl Application {
    /// The minimum valid application endpoint ID.
    pub const MIN_ID: u8 = 1;

    /// The minimum valid application endpoint.
    pub const MIN: Self = Self(Self::MIN_ID);

    /// The maximum valid application endpoint ID.
    pub const MAX_ID: u8 = 240;

    /// The maximum valid application endpoint.
    pub const MAX: Self = Self(Self::MAX_ID);

    /// The valid application endpoint ID range.
    pub const RANGE: RangeInclusive<Self> = Self::MIN..=Self::MAX;

    /// Create a new `Application` endpoint ID if the given ID is valid.
    #[must_use]
    pub const fn new(id: u8) -> Option<Self> {
        if id >= Self::MIN_ID && id <= Self::MAX_ID {
            Some(Self(id))
        } else {
            None
        }
    }

    /// Create a new `Application` endpoint ID, clamping the given ID to the valid range.
    #[must_use]
    pub fn new_clamped(id: u8) -> Self {
        Self(id.clamp(Self::MIN_ID, Self::MAX_ID))
    }

    /// Create a new `Application` endpoint ID without checking validity.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the given ID is within the valid range (1..=240).
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_unchecked(id: u8) -> Self {
        Self(id)
    }
}

impl Default for Application {
    fn default() -> Self {
        Self::MIN
    }
}

impl Display for Application {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl UpperHex for Application {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        UpperHex::fmt(&self.0, f)
    }
}

impl LowerHex for Application {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        LowerHex::fmt(&self.0, f)
    }
}

impl From<Application> for u8 {
    fn from(endpoint: Application) -> Self {
        endpoint.0
    }
}

impl TryFrom<u8> for Application {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(value)
    }
}
