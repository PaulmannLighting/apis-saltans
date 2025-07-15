use chrono::NaiveDate;

/// Zigbee Date Code string type, which is a fixed-size string of 16 bytes.
pub type DateCodeString = heapless::String<16>;

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

impl TryFrom<DateCodeString> for DateCode {
    type Error = chrono::ParseError;

    #[allow(clippy::unwrap_in_result)]
    fn try_from(string: DateCodeString) -> Result<Self, Self::Error> {
        let (date, remainder) = NaiveDate::parse_and_remainder(&string, "%Y%m%d")?;
        let mut custom = heapless::String::new();
        custom
            .push_str(remainder)
            .expect("Remainder should fit into custom string.");
        Ok(Self::new(date, custom))
    }
}

impl From<DateCode> for DateCodeString {
    fn from(date_code: DateCode) -> Self {
        let mut string = Self::new();
        string
            .push_str(&date_code.date.format("%Y%m%d").to_string())
            .expect("Date should fit into string.");
        string
            .push_str(date_code.custom())
            .expect("Custom part should fit into string.");
        string
    }
}
