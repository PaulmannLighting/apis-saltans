use thiserror::Error;

/// Error while creating a `TimeOfDay` instance.
#[derive(Clone, Copy, Debug, Eq, Error, Hash, PartialEq)]
pub enum TryFromNaiveTimeError {
    /// The hour is invalid.
    #[error("Invalid hour: {0}")]
    InvalidHour(u32),
    /// The minute is invalid.
    #[error("Invalid minute: {0}")]
    InvalidMinute(u32),
    /// The second is invalid.
    #[error("Invalid second: {0}")]
    InvalidSecond(u32),
    /// The hundredths of a second is invalid.
    #[error("Invalid hundredths: {0}")]
    InvalidHundredths(u32),
}
