use core::fmt::Display;

/// Error while creating a `TimeOfDay` instance.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    /// The hour is invalid.
    InvalidHour(u8),
    /// The minute is invalid.
    InvalidMinute(u8),
    /// The second is invalid.
    InvalidSecond(u8),
    /// The hundredths of a second is invalid.
    InvalidHundredths(u8),
}

impl Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidHour(hour) => write!(f, "Invalid hour: {hour}"),
            Self::InvalidMinute(minute) => write!(f, "Invalid minute: {minute}"),
            Self::InvalidSecond(second) => write!(f, "Invalid second: {second}"),
            Self::InvalidHundredths(hundredths) => write!(f, "Invalid hundredths: {hundredths}"),
        }
    }
}

impl core::error::Error for Error {}
