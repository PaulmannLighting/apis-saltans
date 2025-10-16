use core::error::Error;
use core::fmt::Display;

/// An error which can occur when converting a [`Date`](super::Date) to a [`NaiveDate`](chrono::NaiveDate).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TryFromDateError {
    /// No year value is set.
    NoYear,
    /// No month value is set.
    NoMonth,
    /// No day of the month value is set.
    NoDayOfMonth,
    /// The year, month or day are out of range.
    InvalidDate {
        /// The year.
        year: u16,
        /// The month.
        month: u8,
        /// The day of the month.
        day_of_month: u8,
    },
}

impl Display for TryFromDateError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoYear => write!(f, "No year value is set"),
            Self::NoMonth => write!(f, "No month value is set"),
            Self::NoDayOfMonth => write!(f, "No day of month value is set"),
            Self::InvalidDate {
                year,
                month,
                day_of_month,
            } => write!(f, "Invalid date: {year}-{month}-{day_of_month}"),
        }
    }
}

impl Error for TryFromDateError {}
