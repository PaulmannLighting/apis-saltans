use core::error::Error;
use core::fmt::Display;

/// Error while creating a `TimeOfDay` instance.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum TryFromNaiveTimeError {
    /// The hour is invalid.
    InvalidHour(u32),
    /// The minute is invalid.
    InvalidMinute(u32),
    /// The second is invalid.
    InvalidSecond(u32),
    /// The hundredths of a second is invalid.
    InvalidHundredths(u32),
}

impl Display for TryFromNaiveTimeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidHour(hour) => write!(f, "Invalid hour: {hour}"),
            Self::InvalidMinute(minute) => write!(f, "Invalid minute: {minute}"),
            Self::InvalidSecond(second) => write!(f, "Invalid second: {second}"),
            Self::InvalidHundredths(hundredths) => write!(f, "Invalid hundredths: {hundredths}"),
        }
    }
}

impl Error for TryFromNaiveTimeError {}
