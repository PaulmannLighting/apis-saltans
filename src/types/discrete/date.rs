use core::ops::RangeInclusive;

pub use error::Error;

const VALID_MONTHS: RangeInclusive<u8> = 1..=12;
const VALID_DAYS_OF_MONTH: RangeInclusive<u8> = 1..=31;
const VALID_DAYS_OF_WEEK: RangeInclusive<u8> = 1..=7;
const YEAR_OFFSET: u16 = 1900;

mod error;

/// Represents a date with year, month, day of the month, and day of the week.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Date {
    year: u8,
    month: u8,
    day_of_month: u8,
    day_of_week: u8,
}

impl Date {
    /// Create a new `Date` instance.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the provided values are out of range.
    pub fn try_new(year: u16, month: u8, day_of_month: u8, day_of_week: u8) -> Result<Self, Error> {
        let Some(year) = year.checked_sub(YEAR_OFFSET) else {
            return Err(Error::InvalidYear(year));
        };

        let Ok(year) = u8::try_from(year) else {
            return Err(Error::InvalidYear(year));
        };

        if !VALID_MONTHS.contains(&month) {
            return Err(Error::InvalidMonth(month));
        }

        if !VALID_DAYS_OF_MONTH.contains(&day_of_month) {
            return Err(Error::InvalidDayOfMonth(day_of_month));
        }

        if !VALID_DAYS_OF_WEEK.contains(&day_of_week) {
            return Err(Error::InvalidDayOfWeek(day_of_week));
        }

        Ok(Self {
            year,
            month,
            day_of_month,
            day_of_week,
        })
    }

    /// Create a new `Date` instance without checking the values.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the values are within the valid ranges:
    #[allow(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_unchecked(
        year: u8,
        month: u8,
        day_of_month: u8,
        day_of_week: u8,
    ) -> Self {
        Self {
            year,
            month,
            day_of_month,
            day_of_week,
        }
    }

    /// Return the year.
    #[must_use]
    pub fn year(self) -> u16 {
        u16::from(self.year) + YEAR_OFFSET
    }

    /// Return the month.
    #[must_use]
    pub const fn month(self) -> u8 {
        self.month
    }

    /// Return the day of the month.
    #[must_use]
    pub const fn day_of_month(self) -> u8 {
        self.day_of_month
    }

    /// Return the day of the week.
    #[must_use]
    pub const fn day_of_week(self) -> u8 {
        self.day_of_week
    }
}
