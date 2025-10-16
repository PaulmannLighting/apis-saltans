use core::ops::Range;

use chrono::NaiveTime;
pub use error::Error;

mod error;

const VALID_HOURS: Range<u8> = 0..24;
const VALID_MINUTES: Range<u8> = 0..60;
const VALID_SECONDS: Range<u8> = 0..60;
const VALID_HUNDREDTHS: Range<u8> = 0..100;

/// Represents a time of day with hour, minute, second, and hundredths of a second.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimeOfDay {
    hour: u8,
    minute: u8,
    second: u8,
    hundredths: u8,
}

impl TimeOfDay {
    /// Create a new `TimeOfDay` instance.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if any of the provided values are out of range.
    pub fn try_new(hour: u8, minute: u8, second: u8, hundredths: u8) -> Result<Self, Error> {
        if !VALID_HOURS.contains(&hour) {
            return Err(Error::InvalidHour(hour));
        }

        if !VALID_MINUTES.contains(&minute) {
            return Err(Error::InvalidMinute(minute));
        }

        if !VALID_SECONDS.contains(&second) {
            return Err(Error::InvalidSecond(second));
        }

        if !VALID_HUNDREDTHS.contains(&hundredths) {
            return Err(Error::InvalidHundredths(hundredths));
        }

        // SAFETY: We just validated the inputs' constraints above.
        #[allow(unsafe_code)]
        Ok(unsafe { Self::new_unchecked(hour, minute, second, hundredths) })
    }

    /// Create a new `TimeOfDay` instance without checking the values.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the provided values are within the valid ranges:
    /// - Hour: 0 to 23
    /// - Minute: 0 to 59
    /// - Second: 0 to 59
    /// - Hundredths: 0 to 99
    #[allow(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_unchecked(hour: u8, minute: u8, second: u8, hundedths: u8) -> Self {
        Self {
            hour,
            minute,
            second,
            hundredths: hundedths,
        }
    }

    /// Get the hour of the day.
    #[must_use]
    pub const fn hour(self) -> u8 {
        self.hour
    }

    /// Get the minute of the hour.
    #[must_use]
    pub const fn minute(self) -> u8 {
        self.minute
    }

    /// Get the second of the minute.
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

impl TryFrom<TimeOfDay> for NaiveTime {
    type Error = ();

    fn try_from(value: TimeOfDay) -> Result<Self, Self::Error> {
        Self::from_hms_milli_opt(
            u32::from(value.hour),
            u32::from(value.minute),
            u32::from(value.second),
            u32::from(value.hundredths) * 10,
        )
        .ok_or(())
    }
}
