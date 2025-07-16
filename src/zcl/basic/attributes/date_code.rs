use std::str::FromStr;

use chrono::NaiveDate;
use le_stream::{FromLeStream, ToLeStream};

const DATE_FORMAT: &str = "%Y%m%d";
const MAX_SIZE: usize = 16;
/// Zigbee Date Code string type, which is a fixed-size string of 16 bytes.
pub type DateCodeString = heapless::String<MAX_SIZE>;

/// Zigbee Date Code attribute.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DateCode {
    date: NaiveDate,
    custom: DateCodeString,
}

impl DateCode {
    /// Create a new `DateCode`.
    #[must_use]
    pub const fn new(date: NaiveDate, custom: DateCodeString) -> Self {
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

impl From<DateCode> for DateCodeString {
    fn from(date_code: DateCode) -> Self {
        let mut string = Self::new();
        string
            .push_str(&date_code.date.format(DATE_FORMAT).to_string())
            .expect("Date should fit into string.");
        string
            .push_str(date_code.custom())
            .expect("Custom part should fit into string.");
        string
    }
}

impl FromStr for DateCode {
    type Err = chrono::ParseError;

    #[allow(clippy::unwrap_in_result)]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (date, remainder) = NaiveDate::parse_and_remainder(s, DATE_FORMAT)?;
        let mut custom = heapless::String::new();
        custom
            .push_str(remainder)
            .expect("Remainder should fit into custom string.");
        Ok(Self::new(date, custom))
    }
}

impl TryFrom<DateCodeString> for DateCode {
    type Error = chrono::ParseError;

    fn try_from(string: DateCodeString) -> Result<Self, Self::Error> {
        string.parse()
    }
}

impl FromLeStream for DateCode {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        String::from_utf8_lossy(
            &bytes
                .take(MAX_SIZE)
                .collect::<heapless::Vec<u8, MAX_SIZE>>(),
        )
        .parse()
        .ok()
    }
}

impl ToLeStream for DateCode {
    type Iter = <heapless::Vec<u8, MAX_SIZE> as IntoIterator>::IntoIter;

    fn to_le_stream(self) -> Self::Iter {
        DateCodeString::from(self).into_bytes().into_iter()
    }
}
