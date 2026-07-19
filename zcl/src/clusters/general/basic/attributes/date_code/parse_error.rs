use heapless::CapacityError;
use thiserror::Error;

/// Error when parsing a `DateCode`.
#[derive(Clone, Copy, Debug, Eq, Error, Hash, PartialEq)]
pub enum ParseError {
    /// The date within the string is invalid.
    #[error("{0}")]
    InvalidDate(
        #[from]
        #[source]
        chrono::ParseError,
    ),

    /// The custom part of the date code is too long.
    #[error("Custom part of date code is too long.")]
    CustomPartTooLong,
}

impl From<CapacityError> for ParseError {
    fn from(_: CapacityError) -> Self {
        Self::CustomPartTooLong
    }
}

#[cfg(test)]
mod tests {
    use core::error::Error as _;

    use chrono::NaiveDate;

    use super::ParseError;

    #[test]
    fn converted_chrono_error_is_retained_as_source() {
        let source =
            NaiveDate::parse_from_str("invalid", "%Y-%m-%d").expect_err("the input is not a date");
        let error = ParseError::from(source);

        assert!(error.source().is_some());
    }
}
