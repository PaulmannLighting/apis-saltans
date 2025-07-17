use core::num::NonZeroU8;

const MASK: u8 = 0xff;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Uint8(NonZeroU8);

impl Uint8 {
    /// Create a new `Uint8`.
    #[must_use]
    pub const fn new(value: u8) -> Option<Self> {
        if let Some(inner) = NonZeroU8::new(value ^ MASK) {
            Some(Self(inner))
        } else {
            None
        }
    }

    /// Get the inner value.
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0.get() ^ MASK
    }
}

impl From<Uint8> for u8 {
    fn from(value: Uint8) -> Self {
        value.get()
    }
}

impl TryFrom<u8> for Uint8 {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(value)
    }
}
