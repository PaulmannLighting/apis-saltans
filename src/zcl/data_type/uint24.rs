use std::num::NonZeroU32;

const MASK: u32 = 0x00ff_ffff;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Uint24(NonZeroU32);

impl Uint24 {
    /// Create a new `Uint24`.
    #[must_use]
    pub const fn new(value: u32) -> Option<Self> {
        let normalized = value & MASK;

        if normalized != value {
            return None;
        }

        if let Some(inner) = NonZeroU32::new(normalized ^ MASK) {
            Some(Self(inner))
        } else {
            None
        }
    }

    /// Get the inner value.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0.get() ^ MASK
    }
}

impl From<Uint24> for u32 {
    fn from(value: Uint24) -> Self {
        value.get()
    }
}

impl TryFrom<u32> for Uint24 {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(value)
    }
}
