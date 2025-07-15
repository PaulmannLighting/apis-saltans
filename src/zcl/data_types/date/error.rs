use std::fmt::Display;

/// Error while creating a `TimeOfDay` instance.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Error {
    /// The year is invalid.
    InvalidYear(u16),
    /// The month is invalid.
    InvalidMonth(u8),
    /// The day of the month is invalid.
    InvalidDayOfMonth(u8),
    /// The day of the week is invalid.
    InvalidDayOfWeek(u8),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidYear(year) => write!(f, "Invalid year: {year}"),
            Self::InvalidMonth(month) => write!(f, "Invalid month: {month}"),
            Self::InvalidDayOfMonth(day_of_month) => {
                write!(f, "Invalid day of month: {day_of_month}")
            }
            Self::InvalidDayOfWeek(day_of_week) => write!(f, "Invalid day of week: {day_of_week}"),
        }
    }
}

impl std::error::Error for Error {}
