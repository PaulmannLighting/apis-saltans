#[cfg(not(feature = "alloc"))]
use core::fmt::Write;
use core::fmt::{Display, Formatter};
use core::str::{FromStr, Utf8Error};

#[cfg(not(feature = "alloc"))]
use chrono::Datelike;
use chrono::NaiveDate;
use either::{Either, Left, Right};
use zb_core::TypeId;
use zb_core::types::{String, Type};

pub use self::parse_error::ParseError;
use crate::{Discrete, ReportingType};

mod parse_error;

const DATE_FORMAT: &str = "%Y%m%d";
const MAX_CUSTOM_SIZE: usize = 8;

/// A string type for the custom part of the date code, which can be up to `MAX_CUSTOM_SIZE` bytes long.
pub type CustomString = heapless::String<MAX_CUSTOM_SIZE>;

/// Zigbee Date Code attribute.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct DateCode {
    date: NaiveDate,
    custom: CustomString,
}

impl TypeId for DateCode {
    const ID: u8 = <String as TypeId>::ID;
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
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        #[cfg(feature = "alloc")]
        {
            write!(f, "{}{}", self.date.format(DATE_FORMAT), self.custom)
        }
        #[cfg(not(feature = "alloc"))]
        {
            write!(f, "{}{}", CompactDate(self.date), self.custom)
        }
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
        #[cfg(feature = "alloc")]
        date_code
            .date
            .format(DATE_FORMAT)
            .write_to(&mut buffer)
            .expect("Date fits into buffer.");
        #[cfg(not(feature = "alloc"))]
        write!(buffer, "{}", CompactDate(date_code.date)).expect("Date fits into buffer.");
        buffer
            .push_str(&date_code.custom)
            .expect("Custom string fits into buffer.");
        Self::from(buffer)
    }
}

impl From<DateCode> for Type {
    fn from(date_code: DateCode) -> Self {
        String::<16>::from(date_code).into()
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

impl TryFrom<Type> for DateCode {
    type Error = Type;

    fn try_from(typ: Type) -> Result<Self, Self::Error> {
        if let Type::String(value) = typ {
            value
                .try_as_str()
                .ok()
                .and_then(|value| value.parse().ok())
                .ok_or(Type::String(value))
        } else {
            Err(typ)
        }
    }
}

impl ReportingType for DateCode {
    type Configuration = Discrete<Self>;
}

#[cfg(not(feature = "alloc"))]
struct CompactDate(NaiveDate);
#[cfg(not(feature = "alloc"))]
impl Display for CompactDate {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        const UNSIGNED_YEARS: core::ops::Range<i32> = 0..10_000;
        let year = self.0.year();
        if UNSIGNED_YEARS.contains(&year) {
            write!(f, "{year:04}")?;
        } else {
            write!(f, "{year:+05}")?;
        }
        write!(f, "{:02}{:02}", self.0.month(), self.0.day())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn display_preserves_padded_and_signed_years() {
        for (year, expected) in [
            (0, "00000102"),
            (42, "00420102"),
            (-1, "-00010102"),
            (10_000, "+100000102"),
        ] {
            let code = DateCode::new(
                NaiveDate::from_ymd_opt(year, 1, 2).expect("valid date"),
                CustomString::new(),
            );
            assert_eq!(alloc::format!("{code}"), expected);
            assert_eq!(
                String::<16>::from(code).try_as_str().expect("ASCII"),
                expected
            );
        }
    }

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
