use core::num::NonZeroU32;

const MASK: u32 = 0xffff_ffff;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Uint32(NonZeroU32);

impl Uint32 {
    /// Create a new `Uint32`.
    #[must_use]
    pub const fn new(value: u32) -> Option<Self> {
        if let Some(inner) = NonZeroU32::new(value ^ MASK) {
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

impl From<Uint32> for u32 {
    fn from(value: Uint32) -> Self {
        value.get()
    }
}

impl TryFrom<u32> for Uint32 {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(value)
    }
}
