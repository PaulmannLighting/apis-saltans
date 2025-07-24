use core::ops::RangeInclusive;

use le_stream::{FromLeStream, ToLeStream};

const INVALID_READING: i16 = -1; // 0xffff in unsigned representation as per 3.4.2.2.1.1.
const RANGE: RangeInclusive<i16> = -200..=200;

/// Represents a temperature in degrees Celsius, with handling for valid, out-of-range, and invalid readings.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Temperature {
    /// A valid temperature reading within the defined range in °C.
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

impl FromLeStream for Temperature {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        i16::from_le_stream(bytes).map(Self::from)
    }
}

impl ToLeStream for Temperature {
    type Iter = <i16 as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::Valid(temp) | Self::OutOfRange(temp) => temp.to_le_stream(),
            Self::InvalidReading => INVALID_READING.to_le_stream(),
        }
    }
}
