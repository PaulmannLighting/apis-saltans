use core::ops::RangeInclusive;

const RANGE: RangeInclusive<i16> = -200..=200;
const INVALID_READING: i16 = -1; // 0xffff in unsigned representation

/// Represents a temperature in degrees Celsius, with handling for valid, out-of-range, and invalid readings.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Temperature {
    /// A valid temperature reading within the defined range.
    Valid(i16),
    /// An out-of-range temperature reading.
    OutOfRange(i16),
    /// An invalid temperature reading.
    InvalidReading,
}

impl From<i16> for Temperature {
    fn from(value: i16) -> Self {
        if value == INVALID_READING {
            Self::InvalidReading
        } else if RANGE.contains(&value) {
            Self::Valid(value)
        } else {
            Self::OutOfRange(value)
        }
    }
}

impl From<Temperature> for i16 {
    fn from(temp: Temperature) -> Self {
        match temp {
            Temperature::OutOfRange(value) | Temperature::Valid(value) => value,
            Temperature::InvalidReading => INVALID_READING,
        }
    }
}
