use core::num::NonZeroU16;

const MASK: u16 = 0xffff;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Uint16(NonZeroU16);

impl Uint16 {
    /// Create a new `Uint16`.
    #[must_use]
    pub const fn new(value: u16) -> Option<Self> {
        if let Some(inner) = NonZeroU16::new(value ^ MASK) {
            Some(Self(inner))
        } else {
            None
        }
    }

    /// Get the inner value.
    #[must_use]
    pub const fn get(self) -> u16 {
        self.0.get() ^ MASK
    }
}

impl From<Uint16> for u16 {
    fn from(value: Uint16) -> Self {
        value.get()
    }
}

impl TryFrom<u16> for Uint16 {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(value)
    }
}
