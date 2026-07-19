use thiserror::Error;

/// Error while creating a `TimeOfDay` instance.
#[derive(Clone, Copy, Debug, Eq, Error, Hash, PartialEq)]
#[error("Invalid time: {hour}:{minute}:{second}.{hundredths}")]
pub struct TryIntoNaiveTimeError {
    hour: u8,
    minute: u8,
    second: u8,
    hundredths: u8,
}

impl TryIntoNaiveTimeError {
    /// Create a new `IntoNaiveTimeError`.
    pub(crate) const fn new(hour: u8, minute: u8, second: u8, hundredths: u8) -> Self {
        Self {
            hour,
            minute,
            second,
            hundredths,
        }
    }

    /// Get the hour.
    #[must_use]
    pub const fn hour(self) -> u8 {
        self.hour
    }

    /// Get the minute.
    #[must_use]
    pub const fn minute(self) -> u8 {
        self.minute
    }

    /// Get the second.
    #[must_use]
    pub const fn second(self) -> u8 {
        self.second
    }

    /// Get the hundredths of a second.
    #[must_use]
    pub const fn hundredths(self) -> u8 {
        self.hundredths
    }
}
