use core::ops::Add;

use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta, TimeZone, Utc};
use le_stream::{FromLeStream, ToLeStream};

const BASE_DATE: NaiveDate = NaiveDate::from_ymd_opt(2000, 1, 1).expect("Default date is valid.");
const BASE_TIME: NaiveTime = NaiveTime::from_hms_opt(0, 0, 0).expect("Default time is valid.");
const BASE_NAIVE_DATETIME: NaiveDateTime = NaiveDateTime::new(BASE_DATE, BASE_TIME);
const BASE_DATETIME: DateTime<Utc> = DateTime::from_naive_utc_and_offset(BASE_NAIVE_DATETIME, Utc);
const NON_VALUE: u32 = 0xffff_ffff;

/// UTC time data type.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
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
    type Error = i64;

    /// Converts a [`TimeDelta`] to a `UtcTime`.
    ///
    /// This will discard any sub-second information.
    ///
    /// # Errors
    ///
    /// This will return an error if the number of seconds is out of range for a `u32`.
    fn try_from(value: TimeDelta) -> Result<Self, Self::Error> {
        let Ok(seconds) = u32::try_from(value.num_seconds()) else {
            return Err(value.num_seconds());
        };

        if seconds == NON_VALUE {
            return Err(seconds.into());
        }

        Ok(Self(seconds))
    }
}

impl<T> TryFrom<DateTime<T>> for UtcTime
where
    T: TimeZone,
{
    type Error = i64;

    /// Converts a [`DateTime`] to a `UtcTime`.
    ///
    /// This will discard any sub-second information.
    ///
    /// # Errors
    ///
    /// This will return an error if the number of seconds is out of range for a `u32`.
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
    fn datetime_from_utc_with_non_value() {
        let utc_time = UtcTime(NON_VALUE);
        let result = DateTime::<Utc>::try_from(utc_time);
        assert_eq!(result, Err(()));
    }

    #[test]
    fn try_from_utc_time() {
        let utc_time: UtcTime = BASE_DATETIME.try_into().unwrap();
        assert_eq!(utc_time, UtcTime(0));
    }

    #[test]
    fn try_from_utc_time_non_value() {
        let result = UtcTime::try_from(BASE_DATETIME.add(TimeDelta::seconds(NON_VALUE.into())));
        assert_eq!(result, Err(NON_VALUE.into()));
    }
}
