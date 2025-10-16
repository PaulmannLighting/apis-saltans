use core::num::TryFromIntError;
use core::ops::Add;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, TimeZone, Utc};

// Base datetime is 2000-01-01T00:00:00
const BASE_DATETIME: DateTime<Utc> = DateTime::<Utc>::from_naive_utc_and_offset(
    NaiveDateTime::new(
        NaiveDate::from_ymd_opt(2000, 1, 1).expect("Default date is valid."),
        NaiveTime::from_hms_opt(0, 0, 0).expect("Default time is valid."),
    ),
    Utc,
);
const NON_VALUE: u32 = 0xffff_ffff;

/// UTC time data type.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UtcTime(u32);

impl From<UtcTime> for Option<u32> {
    fn from(value: UtcTime) -> Self {
        if value.0 == NON_VALUE {
            None
        } else {
            Some(value.0)
        }
    }
}

impl TryFrom<TimeDelta> for UtcTime {
    type Error = TryFromIntError;

    /// Converts a `TimeDelta` to a `UtcTime`.
    ///
    /// This will discard any sub-second information.
    fn try_from(value: TimeDelta) -> Result<Self, Self::Error> {
        value.num_seconds().try_into().map(Self)
    }
}

impl<T> TryFrom<DateTime<T>> for UtcTime
where
    T: TimeZone,
{
    type Error = TryFromIntError;

    /// Converts a `DateTime` to a `UtcTime`.
    ///
    /// This will discard any sub-second information.
    fn try_from(value: DateTime<T>) -> Result<Self, Self::Error> {
        Self::try_from(value.signed_duration_since(BASE_DATETIME))
    }
}

impl TryFrom<UtcTime> for TimeDelta {
    type Error = ();

    fn try_from(value: UtcTime) -> Result<Self, Self::Error> {
        Option::<u32>::from(value)
            .ok_or(())
            .map(|seconds| Self::seconds(seconds.into()))
    }
}

impl TryFrom<UtcTime> for DateTime<Utc> {
    type Error = ();

    fn try_from(value: UtcTime) -> Result<Self, Self::Error> {
        TimeDelta::try_from(value).map(|time_delta| BASE_DATETIME.add(time_delta))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn datetime_from_utc_no_offset() {
        let utc_time = UtcTime(0);
        let datetime: DateTime<Utc> = utc_time.try_into().unwrap();
        assert_eq!(datetime, BASE_DATETIME);
    }

    #[test]
    fn datetime_from_utc_with_offset() {
        let utc_time = UtcTime(1000);
        let datetime: DateTime<Utc> = utc_time.try_into().unwrap();
        assert_eq!(datetime, BASE_DATETIME.add(TimeDelta::seconds(1000)));
    }

    #[test]
    fn try_from_utc_time() {
        let utc_time: UtcTime = BASE_DATETIME.try_into().unwrap();
        assert_eq!(utc_time, UtcTime(0));
    }
}
