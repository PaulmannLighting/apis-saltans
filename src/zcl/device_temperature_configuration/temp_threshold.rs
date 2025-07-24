use core::ops::RangeInclusive;

use le_stream::{FromLeStream, ToLeStream};

const DO_NOT_GENERATE: i16 = -32768; // 0x8000 as per 3.4.2.2.2.2.
const RANGE: RangeInclusive<i16> = -200..=200;

/// Represents a temperature threshold in degrees Celsius.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TempThreshold {
    /// A valid temperature threshold within the defined range in °C.
    Valid(i16),
    /// An out-of-range temperature threshold.
    OutOfRange(i16),
    /// A special value indicating that the temperature threshold should not generate an alarm.
    DoNotGenerate,
}

impl From<i16> for TempThreshold {
    fn from(value: i16) -> Self {
        if value == DO_NOT_GENERATE {
            Self::DoNotGenerate
        } else if RANGE.contains(&value) {
            Self::Valid(value)
        } else {
            Self::OutOfRange(value)
        }
    }
}

impl From<TempThreshold> for i16 {
    fn from(threshold: TempThreshold) -> Self {
        match threshold {
            TempThreshold::Valid(temp) | TempThreshold::OutOfRange(temp) => temp,
            TempThreshold::DoNotGenerate => DO_NOT_GENERATE,
        }
    }
}

impl FromLeStream for TempThreshold {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        i16::from_le_stream(bytes).map(Self::from)
    }
}

impl ToLeStream for TempThreshold {
    type Iter = <i16 as ToLeStream>::Iter;

    fn to_le_stream(self) -> Self::Iter {
        match self {
            Self::Valid(temp) | Self::OutOfRange(temp) => temp.to_le_stream(),
            Self::DoNotGenerate => DO_NOT_GENERATE.to_le_stream(),
        }
    }
}
