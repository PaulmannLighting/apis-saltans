use core::ops::RangeInclusive;

use le_stream::derive::{FromLeStream, ToLeStream};
use zigbee::types::Int16;

const RANGE: RangeInclusive<i16> = -200..=200;

/// Represents a temperature in degrees Celsius, with handling for valid, out-of-range, and invalid readings.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct Temperature(Int16);

impl Temperature {
    /// Creates a new `Temperature` instance.
    #[must_use]
    pub fn new(degrees_celsius: i16) -> Option<Self> {
        if RANGE.contains(&degrees_celsius) {
            Int16::try_from(degrees_celsius).ok().map(Self)
        } else {
            None
        }
    }

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
        Option::<i16>::from(self.0).map_or(Ok(None), |value| {
            if RANGE.contains(&value) {
                Ok(Some(value))
            } else {
                Err(value)
            }
        })
    }
}
