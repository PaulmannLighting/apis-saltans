use chrono::{Datelike, NaiveDate, Weekday};
pub use try_from_date_error::TryFromDateError;
pub use try_from_naive_date_error::TryFromNaiveDateError;

mod try_from_date_error;
mod try_from_naive_date_error;

const YEAR_OFFSET: u16 = 1900;
const NON_VALUE: u8 = 0xff;

/// Represents a date with year, month, day of the month, and day of the week.
#[derive(Debug, Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Date {
    year: u8,
    month: u8,
    day_of_month: u8,
    day_of_week: u8,
}

impl Date {
    /// Return the year.
    #[must_use]
    pub fn year(self) -> Option<u16> {
        if self.year == NON_VALUE {
            None
        } else {
            u16::from(self.year).checked_add(YEAR_OFFSET)
        }
    }

    /// Return the month.
    #[must_use]
    pub const fn month(self) -> Option<u8> {
        if self.month == NON_VALUE {
            None
        } else {
            Some(self.month)
        }
    }

    /// Return the day of the month.
    #[must_use]
    pub const fn day_of_month(self) -> Option<u8> {
        if self.day_of_month == NON_VALUE {
            None
        } else {
            Some(self.day_of_month)
        }
    }

    /// Return the day of the week.
    #[must_use]
    pub const fn day_of_week(self) -> Option<u8> {
        if self.day_of_week == NON_VALUE {
            None
        } else {
            Some(self.day_of_week)
        }
    }

    /// Return the day of the week as a [`Weekday`].
    #[must_use]
    pub const fn weekday(self) -> Option<Weekday> {
        match self.day_of_week {
            1 => Some(Weekday::Mon),
            2 => Some(Weekday::Tue),
            3 => Some(Weekday::Wed),
            4 => Some(Weekday::Thu),
            5 => Some(Weekday::Fri),
            6 => Some(Weekday::Sat),
            7 => Some(Weekday::Sun),
            _ => None,
        }
    }

    /// Convert to an `Option<Date>`, returning `None` if all fields are non-values.
    #[must_use]
    pub const fn into_option(self) -> Option<Self> {
        if self.year == NON_VALUE
            && self.month == NON_VALUE
            && self.day_of_month == NON_VALUE
            && self.day_of_week == NON_VALUE
        {
            None
        } else {
            Some(self)
        }
    }
}

impl TryFrom<Date> for NaiveDate {
    type Error = TryFromDateError;

    fn try_from(value: Date) -> Result<Self, Self::Error> {
        let Some(year) = value.year() else {
            return Err(TryFromDateError::NoYear);
        };

        let Some(month) = value.month() else {
            return Err(TryFromDateError::NoMonth);
        };

        let Some(day_of_month) = value.day_of_month() else {
            return Err(TryFromDateError::NoDayOfMonth);
        };

        Self::from_ymd_opt(year.into(), month.into(), day_of_month.into()).ok_or(
            TryFromDateError::InvalidDate {
                year,
                month,
                day_of_month,
            },
        )
    }
}

impl TryFrom<NaiveDate> for Date {
    type Error = TryFromNaiveDateError;

    #[allow(clippy::unwrap_in_result)]
    fn try_from(value: NaiveDate) -> Result<Self, Self::Error> {
        let Some(year_offset) = value.year().checked_sub(i32::from(YEAR_OFFSET)) else {
            return Err(TryFromNaiveDateError::YearOverflow(value.year()));
        };

        let Ok(year) = year_offset.try_into() else {
            return Err(TryFromNaiveDateError::YearOverflow(year_offset));
        };

        if year == NON_VALUE {
            return Err(TryFromNaiveDateError::YearOffsetIsNonValue);
        }

        // NaiveDate guarantees valid month, day, and weekday values.
        // Furthermore, neither of those values can be 0xff, i.e. the non-value.
        let month = value
            .month()
            .try_into()
            .expect("NaiveDate guarantees a valid month.");
        let day_of_month = value
            .day()
            .try_into()
            .expect("NaiveDate guarantees a valid day.");
        let day_of_week = value
            .weekday()
            .number_from_monday()
            .try_into()
            .expect("NaiveDate guarantees a valid day of week.");
        Ok(Self {
            year,
            month,
            day_of_month,
            day_of_week,
        })
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    #[test]
    fn into_naive_date() {
        let date = Date {
            year: (2023 - YEAR_OFFSET).try_into().expect("Year is valid."),
            month: 3,
            day_of_month: 14,
            day_of_week: 2,
        };
        let naive_date: NaiveDate = date.try_into().unwrap();
        assert_eq!(naive_date, NaiveDate::from_ymd_opt(2023, 3, 14).unwrap());
    }

    #[test]
    fn into_naive_date_no_year() {
        let date = Date {
            year: 0xff,
            month: 3,
            day_of_month: 14,
            day_of_week: 2,
        };
        let result = NaiveDate::try_from(date);
        assert_eq!(result, Err(TryFromDateError::NoYear));
    }

    #[test]
    fn into_naive_date_no_month() {
        let date = Date {
            year: (2023 - YEAR_OFFSET).try_into().expect("Year is valid."),
            month: 0xff,
            day_of_month: 14,
            day_of_week: 2,
        };
        let result = NaiveDate::try_from(date);
        assert_eq!(result, Err(TryFromDateError::NoMonth));
    }

    #[test]
    fn into_naive_date_no_day_of_month() {
        let date = Date {
            year: (2023 - YEAR_OFFSET).try_into().expect("Year is valid."),
            month: 3,
            day_of_month: 0xff,
            day_of_week: 2,
        };
        let result = NaiveDate::try_from(date);
        assert_eq!(result, Err(TryFromDateError::NoDayOfMonth));
    }

    #[test]
    fn into_naive_date_invalid_month() {
        let date = Date {
            year: (2023 - YEAR_OFFSET).try_into().expect("Year is valid."),
            month: 13,
            day_of_month: 14,
            day_of_week: 2,
        };
        let result = NaiveDate::try_from(date);
        assert_eq!(
            result,
            Err(TryFromDateError::InvalidDate {
                year: 2023,
                month: 13,
                day_of_month: 14,
            })
        );
    }

    #[test]
    fn into_naive_date_invalid_day_of_month() {
        let date = Date {
            year: (2023 - YEAR_OFFSET).try_into().expect("Year is valid."),
            month: 3,
            day_of_month: 32,
            day_of_week: 2,
        };
        let result = NaiveDate::try_from(date);
        assert_eq!(
            result,
            Err(TryFromDateError::InvalidDate {
                year: 2023,
                month: 3,
                day_of_month: 32,
            })
        );
    }

    #[test]
    fn try_from_naive_date() {
        let naive_date = NaiveDate::from_ymd_opt(2023, 3, 14).unwrap();
        let date = Date::try_from(naive_date).unwrap();
        assert_eq!(date.year(), Some(2023));
        assert_eq!(date.month(), Some(3));
        assert_eq!(date.day_of_month(), Some(14));
        assert_eq!(date.day_of_week(), Some(2));
        assert_eq!(date.weekday(), Some(Weekday::Tue));
    }

    #[test]
    fn try_from_naive_date_year_overflow() {
        let naive_date = NaiveDate::from_ymd_opt(2156, 3, 14).unwrap();
        let result = Date::try_from(naive_date);
        assert_eq!(result, Err(TryFromNaiveDateError::YearOverflow(2156)));
    }

    #[test]
    fn try_from_naive_date_year_no_value() {
        let naive_date = NaiveDate::from_ymd_opt(2155, 3, 14).unwrap();
        let result = Date::try_from(naive_date);
        assert_eq!(result, Err(TryFromNaiveDateError::YearOffsetIsNonValue));
    }
}
