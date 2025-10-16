use core::error::Error;
use core::fmt::Display;

/// An error that can occur when converting a [`NaiveDate`](chrono::NaiveDate) to a [`Date`](super::Date).
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TryFromNaiveDateError {
    YearOverflow(i32),
    YearOffsetIsNonValue,
}

impl Display for TryFromNaiveDateError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::YearOverflow(year) => write!(f, "Year {year} is out of range"),
            Self::YearOffsetIsNonValue => write!(f, "Year offset results in non-value"),
        }
    }
}

impl Error for TryFromNaiveDateError {}
