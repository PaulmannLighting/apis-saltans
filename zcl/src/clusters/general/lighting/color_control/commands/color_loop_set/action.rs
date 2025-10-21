use num_traits::FromPrimitive;
pub use source::Source;

mod source;

/// Available color loop set actions.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Action {
    /// Deactivate the color loop.
    Deactivate,
    /// Activate the color loop.
    Activate(Source),
}

impl Action {
    /// Return the action as a `u8` value.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::Deactivate => 0x00,
            Self::Activate(source) => source as u8,
        }
    }
}

impl FromPrimitive for Action {
    fn from_i64(n: i64) -> Option<Self> {
        match u8::try_from(n).ok()? {
            0x00 => Some(Self::Deactivate),
            other => Some(Self::Activate(Source::from_u8(other)?)),
        }
    }

    fn from_u64(n: u64) -> Option<Self> {
        match u8::try_from(n).ok()? {
            0x00 => Some(Self::Deactivate),
            other => Some(Self::Activate(Source::from_u8(other)?)),
        }
    }
}
