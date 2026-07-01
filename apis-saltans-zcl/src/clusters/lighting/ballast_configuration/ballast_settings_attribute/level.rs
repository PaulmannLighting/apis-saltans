use core::num::NonZero;

/// Valid levels for the Ballast Settings Level attribute.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Level(NonZero<u8>);

impl Level {
    /// Minimum valid value for Level.
    pub const MIN: u8 = 0x01;

    /// Maximum valid value for Level.
    pub const MAX: u8 = 0xfe;
}

impl From<Level> for NonZero<u8> {
    fn from(value: Level) -> Self {
        value.0
    }
}

impl From<Level> for u8 {
    fn from(value: Level) -> Self {
        value.0.get()
    }
}

impl TryFrom<u8> for Level {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if (Self::MIN..=Self::MAX).contains(&value) {
            Ok(Self(
                #[expect(unsafe_code)]
                // SAFETY: value is guaranteed to be non-zero due to the range check above.
                unsafe {
                    NonZero::new_unchecked(value)
                },
            ))
        } else {
            Err(value)
        }
    }
}
