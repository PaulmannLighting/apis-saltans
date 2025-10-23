use chrono::{NaiveTime, Timelike};
use log::debug;

pub use self::try_from_naive_time_error::TryFromNaiveTimeError;
pub use self::try_into_naive_time_error::TryIntoNaiveTimeError;

mod try_from_naive_time_error;
mod try_into_naive_time_error;

const NANOS_PER_HUNDREDTHS: u32 = 10_000_000;
const NON_VALUE: u8 = 0xff;

/// Represents a time of day with hour, minute, second, and hundredths of a second.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TimeOfDay {
    hour: u8,
    minute: u8,
    second: u8,
    hundredths: u8,
}

impl TimeOfDay {
    /// Get the hour of the day.
    #[must_use]
    pub const fn hour(self) -> Option<u8> {
        if self.hour == NON_VALUE {
            None
        } else {
            Some(self.hour)
        }
    }

    /// Get the minute of the hour.
    #[must_use]
    pub const fn minute(self) -> Option<u8> {
        if self.minute == NON_VALUE {
            None
        } else {
            Some(self.minute)
        }
    }

    /// Get the second of the minute.
    #[must_use]
    pub const fn second(self) -> Option<u8> {
        if self.second == NON_VALUE {
            None
        } else {
            Some(self.second)
        }
    }

    /// Get the hundredths of a second.
    #[must_use]
    pub const fn hundredths(self) -> Option<u8> {
        if self.hundredths == NON_VALUE {
            None
        } else {
            Some(self.hundredths)
        }
    }
}

impl TryFrom<NaiveTime> for TimeOfDay {
    type Error = TryFromNaiveTimeError;

    fn try_from(value: NaiveTime) -> Result<Self, Self::Error> {
        let Ok(hour) = value.hour().try_into() else {
            return Err(TryFromNaiveTimeError::InvalidHour(value.hour()));
        };

        let Ok(minute) = value.minute().try_into() else {
            return Err(TryFromNaiveTimeError::InvalidMinute(value.minute()));
        };

        let Ok(second) = value.second().try_into() else {
            return Err(TryFromNaiveTimeError::InvalidSecond(value.second()));
        };

        let hundredths = value.nanosecond() / NANOS_PER_HUNDREDTHS;

        let Ok(hundredths) = hundredths.try_into() else {
            return Err(TryFromNaiveTimeError::InvalidHundredths(
                value.nanosecond() / NANOS_PER_HUNDREDTHS,
            ));
        };

        let nanos = value.nanosecond() % NANOS_PER_HUNDREDTHS;

        if nanos != 0 {
            debug!("Warning: NaiveTime has non-zero nanoseconds part: {nanos}");
        }

        // NaiveTime guarantees valid ranges of all fields.
        Ok(Self {
            hour,
            minute,
            second,
            hundredths,
        })
    }
}

impl TryFrom<TimeOfDay> for NaiveTime {
    type Error = TryIntoNaiveTimeError;

    fn try_from(value: TimeOfDay) -> Result<Self, Self::Error> {
        let hour = value.hour().unwrap_or_default();
        let minute = value.minute().unwrap_or_default();
        let second = value.second().unwrap_or_default();
        let hundredths = value.hundredths().unwrap_or_default();
        Self::from_hms_milli_opt(
            u32::from(hour),
            u32::from(minute),
            u32::from(second),
            u32::from(hundredths) * 10,
        )
        .ok_or_else(|| TryIntoNaiveTimeError::new(hour, minute, second, hundredths))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_into_naive_time() {
        let time_of_day = TimeOfDay {
            hour: 12,
            minute: 34,
            second: 56,
            hundredths: 78,
        };
        let naive_time: NaiveTime = time_of_day.try_into().unwrap();
        assert_eq!(
            naive_time,
            NaiveTime::from_hms_milli_opt(12, 34, 56, 780).unwrap()
        );
    }

    #[test]
    fn from_naive_time() {
        let naive_time = NaiveTime::from_hms_milli_opt(12, 34, 56, 780).unwrap();
        let time_of_day: TimeOfDay = naive_time.try_into().unwrap();
        assert_eq!(time_of_day.hour(), Some(12));
        assert_eq!(time_of_day.minute(), Some(34));
        assert_eq!(time_of_day.second(), Some(56));
        assert_eq!(time_of_day.hundredths(), Some(78));
    }
}
