use super::date::Date;
use super::time_of_day::TimeOfDay;
use super::utc_time::UtcTime;

/// Time data type, which can represent a time of day, a date, or a UTC time.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Time {
    TimeOfDay(TimeOfDay),
    Date(Date),
    UtcTime(UtcTime),
}
