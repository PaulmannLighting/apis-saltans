use core::time::Duration;

use crate::constants::DECI_SECONDS_PER_MILLISECOND;

/// Trait to convert a value into deciseconds.
pub trait IntoDeciSeconds {
    /// Converts the value into deciseconds.
    fn into_deci_seconds(self) -> u128;
}

impl IntoDeciSeconds for Duration {
    fn into_deci_seconds(self) -> u128 {
        self.as_millis() / u128::from(DECI_SECONDS_PER_MILLISECOND)
    }
}
