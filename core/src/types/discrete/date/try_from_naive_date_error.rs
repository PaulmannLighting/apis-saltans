use thiserror::Error;

/// An error that can occur when converting a [`NaiveDate`](chrono::NaiveDate) to a [`Date`](super::Date).
#[derive(Clone, Copy, Debug, Eq, Error, Hash, PartialEq)]
pub enum TryFromNaiveDateError {
    /// The year is out of range.
    ///
    /// A `Date` can only represent years in the range 1900-2154.
    #[error("Year {0} is out of range")]
    YearOverflow(i32),
    /// The year offset results in a non-value.
    ///
    /// This happens when the offset is 0xff (255), i.e. the year is 2155.
    #[error("Year offset results in non-value")]
    YearOffsetIsNonValue,
}
