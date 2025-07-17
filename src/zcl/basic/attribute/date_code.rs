use std::fmt::Display;
use std::str::FromStr;

use chrono::NaiveDate;
use le_stream::{FromLeStream, ToLeStream};
pub use parse_error::ParseError;

mod parse_error;

const DATE_FORMAT: &str = "%Y%m%d";
const DATE_SIZE: usize = 8;
const MAX_CUSTOM_SIZE: usize = 8;
const MAX_SIZE: usize = DATE_SIZE + MAX_CUSTOM_SIZE;

/// Zigbee Date Code string type, which is a fixed-size string of 16 bytes.
pub type DateCodeString = heapless::String<MAX_SIZE>;
/// A vector of bytes representing the date code, which can be up to `MAX_SIZE` bytes long.
pub type DateCodeBytes = heapless::Vec<u8, MAX_SIZE>;
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        DateCodeString::from(self).fmt(f)
    }
}

impl From<DateCode> for DateCodeString {
    fn from(date_code: DateCode) -> Self {
        Self::from(&date_code)
    }
}

impl From<&DateCode> for DateCodeString {
    fn from(date_code: &DateCode) -> Self {
        let mut string = Self::new();
        string
            .push_str(&date_code.date.format(DATE_FORMAT).to_string())
            .expect("Serialized date should fit into DateCodeString.");
        string
            .push_str(date_code.custom())
            .expect("Custom part should fit into DateCodeString.");
        string
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

impl FromLeStream for DateCode {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        String::from_utf8_lossy(&bytes.take(MAX_SIZE).collect::<DateCodeBytes>())
            .parse()
            .ok()
    }
}

impl ToLeStream for DateCode {
    type Iter = <DateCodeBytes as IntoIterator>::IntoIter;

    fn to_le_stream(self) -> Self::Iter {
        DateCodeString::from(self).into_bytes().into_iter()
    }
}
