use core::time::Duration;

use zigbee::constants::DECI_SECONDS_PER_MILLISECOND;

/// Trait to create a Duration from deciseconds.
pub trait FromDeciSeconds {
    /// Returns a Duration from deciseconds.
    fn from_deci_seconds(deci_seconds: u16) -> Self;
}

impl FromDeciSeconds for Duration {
    fn from_deci_seconds(deci_seconds: u16) -> Self {
        Self::from_millis(u64::from(deci_seconds) * DECI_SECONDS_PER_MILLISECOND)
    }
}
