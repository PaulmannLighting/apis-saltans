use core::ops::RangeInclusive;

use le_stream::derive::{FromLeStream, ToLeStream};

const INVALID_READING: i16 = -1; // 0xffff in unsigned representation as per 3.4.2.2.1.1.
const RANGE: RangeInclusive<i16> = -200..=200;

/// Represents a temperature in degrees Celsius, with handling for valid, out-of-range, and invalid readings.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct Temperature(i16);

impl Temperature {
    /// Returns the temperature value in degrees Celsius.
    ///
    /// # Returns
    ///
    /// - `Ok(Some(value))` if the temperature is valid and within range.
    /// - `Ok(None)` if the temperature is invalid (i.e., `-1`).
    ///
    /// # Errors
    ///
    /// Returns `Err(value)` if the temperature is out of the valid range (-200 to 200 degrees Celsius).
    pub fn degrees_celsius(self) -> Result<Option<i16>, i16> {
        if self.0 == INVALID_READING {
            Ok(None)
        } else if RANGE.contains(&self.0) {
            Ok(Some(self.0))
        } else {
            Err(self.0)
        }
    }
}
