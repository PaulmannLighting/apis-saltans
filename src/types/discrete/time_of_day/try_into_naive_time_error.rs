use core::error::Error;
use core::fmt::Display;

/// Error while creating a `TimeOfDay` instance.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub struct TryIntoNaiveTimeError {
    /// The hour.
    hour: u8,
    /// The minute.
    minute: u8,
    /// The second.
    second: u8,
    /// The hundredths of a second.
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

impl Display for TryIntoNaiveTimeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "Invalid time: {}:{}:{}.{}",
            self.hour, self.minute, self.second, self.hundredths
        )
    }
}

impl Error for TryIntoNaiveTimeError {}
