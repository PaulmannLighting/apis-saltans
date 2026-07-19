use thiserror::Error;

/// An error which can occur when converting a [`Date`](super::Date) to a [`NaiveDate`](chrono::NaiveDate).
#[derive(Clone, Copy, Debug, Eq, Error, Hash, PartialEq)]
pub enum TryIntoNaiveDateError {
    /// No year value is set.
    #[error("No year value is set")]
    NoYear,
    /// No month value is set.
    #[error("No month value is set")]
    NoMonth,
    /// No day of the month value is set.
    #[error("No day of month value is set")]
    NoDayOfMonth,
    /// The year, month or day are out of range.
    #[error("Invalid date: {year}-{month}-{day_of_month}")]
    InvalidDate {
        /// The year.
        year: u16,
        /// The month.
        month: u8,
        /// The day of the month.
        day_of_month: u8,
    },
}
