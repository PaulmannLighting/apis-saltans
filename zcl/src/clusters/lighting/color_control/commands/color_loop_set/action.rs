pub use self::source::Source;

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

impl From<Action> for u8 {
    fn from(action: Action) -> Self {
        action.as_u8()
    }
}

impl TryFrom<u8> for Action {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 => Ok(Self::Deactivate),
            other => Source::try_from(other)
                .map(Self::Activate)
                .map_err(|_| value),
        }
    }
}
