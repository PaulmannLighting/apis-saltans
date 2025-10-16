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

/// UTC time data type.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct UtcTime(u32);

impl UtcTime {
    /// Creates a new `UtcTime` instance from a [`TimeDelta`].
    ///
    /// # Returns
    ///
    /// A tuple of the `UtcTime` instance and the remaining nanoseconds.
    ///
    /// # Errors
    ///
    /// Returns an [`TryFromIntError`] if the seconds part of the `TimeDelta` cannot be converted to `u32`.
    pub fn try_from_time_delta(time_delta: TimeDelta) -> Result<(Self, i32), TryFromIntError> {
        time_delta
            .num_seconds()
            .try_into()
            .map(|seconds| (Self(seconds), time_delta.subsec_nanos()))
    }

    /// Creates a new `UtcTime` instance from a [`DateTime`].
    ///
    /// # Returns
    ///
    /// A tuple of the `UtcTime` instance and the remaining nanoseconds.
    ///
    /// # Errors
    ///
    /// Returns an [`TryFromIntError`] if the seconds part of the `TimeDelta` cannot be converted to `u32`.
    pub fn try_from_date_time<T>(date_time: DateTime<T>) -> Result<(Self, i32), TryFromIntError>
    where
        T: TimeZone,
    {
        Self::try_from_time_delta(date_time.signed_duration_since(BASE_DATETIME))
    }
}

impl From<UtcTime> for TimeDelta {
    fn from(value: UtcTime) -> Self {
        Self::seconds(i64::from(value.0))
    }
}

impl From<UtcTime> for DateTime<Utc> {
    fn from(value: UtcTime) -> Self {
        BASE_DATETIME.add(TimeDelta::from(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_offset() {
        let utc_time = UtcTime(0);
        let datetime: DateTime<Utc> = utc_time.into();
        assert_eq!(datetime, BASE_DATETIME);
    }

    #[test]
    fn some_offset() {
        let utc_time = UtcTime(1000);
        let datetime: DateTime<Utc> = utc_time.into();
        assert_eq!(datetime, BASE_DATETIME.add(TimeDelta::seconds(1000)));
    }
}
