use alloc::string::ToString;
use core::fmt::Display;
use core::str::FromStr;

use chrono::NaiveDate;
pub use parse_error::ParseError;

use crate::types::String;

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

impl From<DateCode> for String {
    fn from(date_code: DateCode) -> Self {
        Self::from(&date_code)
    }
}

impl From<&DateCode> for String {
    fn from(date_code: &DateCode) -> Self {
        date_code
            .to_string()
            .try_into()
            .expect("Date code should fit into a String.")
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
