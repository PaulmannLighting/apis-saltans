use core::fmt::Display;
use core::str::{FromStr, Utf8Error};

use chrono::NaiveDate;
use either::{Either, Left, Right};
pub use parse_error::ParseError;
use zigbee::types::String;

mod parse_error;

const DATE_FORMAT: &str = "%Y%m%d";
const MAX_CUSTOM_SIZE: usize = 8;

/// A custom string type for the custom part of the date code, which can be up to `MAX_CUSTOM_SIZE` bytes long.
pub type CustomString = heapless::String<MAX_CUSTOM_SIZE>;

/// Zigbee Date Code attribute.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DateCode {
    date: NaiveDate,
    custom: CustomString,
}

impl DateCode {
    /// Create a new `DateCode`.
    #[must_use]
    pub const fn new(date: NaiveDate, custom: CustomString) -> Self {
        Self { date, custom }
    }

    /// Return the date.
    #[must_use]
    pub const fn date(&self) -> NaiveDate {
        self.date
    }

    /// Return the custom part of the date code.
    #[must_use]
    pub fn custom(&self) -> &str {
        &self.custom
    }
}

impl Display for DateCode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}{}", self.date.format(DATE_FORMAT), self.custom)
    }
}

impl From<DateCode> for String<16> {
    fn from(date_code: DateCode) -> Self {
        Self::from(&date_code)
    }
}

impl From<&DateCode> for String<16> {
    fn from(date_code: &DateCode) -> Self {
        let mut buffer = heapless::String::new();
        date_code
            .date
            .format(DATE_FORMAT)
            .write_to(&mut buffer)
            .expect("Date should be writable to buffer.");
        buffer
            .push_str(&date_code.custom)
            .expect("Custom string should fit into buffer.");
        Self::from(buffer)
    }
}

impl FromStr for DateCode {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (date, remainder) = NaiveDate::parse_and_remainder(s, DATE_FORMAT)?;
        let mut custom = CustomString::new();
        custom.push_str(remainder)?;
        Ok(Self::new(date, custom))
    }
}

impl<const CAPACITY: usize> TryFrom<String<CAPACITY>> for DateCode {
    type Error = Either<Utf8Error, ParseError>;

    fn try_from(value: String<CAPACITY>) -> Result<Self, Self::Error> {
        Self::from_str(value.try_as_str().map_err(Left)?).map_err(Right)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_with_custom() {
        let date_code = DateCode::from_str("20060814Custom").unwrap();
        assert_eq!(
            date_code.date(),
            NaiveDate::from_ymd_opt(2006, 8, 14).unwrap(),
        );
        assert_eq!(date_code.custom(), "Custom");
    }

    #[test]
    fn from_str_without_custom() {
        let date_code = DateCode::from_str("20060814").unwrap();
        assert_eq!(
            date_code.date(),
            NaiveDate::from_ymd_opt(2006, 8, 14).unwrap(),
        );
        assert_eq!(date_code.custom(), "");
    }

    #[test]
    fn to_string() {
        let date_code = DateCode::new(
            NaiveDate::from_ymd_opt(2006, 8, 14).unwrap(),
            "Custom".try_into().unwrap(),
        );
        assert_eq!(
            String::from(date_code).try_as_str().unwrap(),
            "20060814Custom"
        );
    }
}
