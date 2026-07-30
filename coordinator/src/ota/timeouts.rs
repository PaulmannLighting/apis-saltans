use std::time::Duration;

const DEFAULT_DISCOVERY_TIMEOUT: Duration = Duration::from_mins(15);
const DEFAULT_BLOCK_INACTIVITY_TIMEOUT: Duration = Duration::from_mins(15);
const DEFAULT_TOTAL_TRANSFER_TIMEOUT: Duration = Duration::from_hours(24);

/// Deadlines governing one coordinator-managed OTA update offer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UpdateTimeouts {
    discovery: Duration,
    block_inactivity: Duration,
    total_transfer: Duration,
}

impl UpdateTimeouts {
    /// Create OTA update deadlines.
    ///
    /// `discovery` limits how long the client may take to accept the offer. Once a compatible
    /// query or valid block request arrives, `block_inactivity` limits the delay between transfer
    /// requests. `total_transfer` bounds the complete offer regardless of activity.
    #[must_use]
    pub const fn new(
        discovery: Duration,
        block_inactivity: Duration,
        total_transfer: Duration,
    ) -> Self {
        Self {
            discovery,
            block_inactivity,
            total_transfer,
        }
    }

    /// Return the maximum time allowed for the client to accept the image offer.
    #[must_use]
    pub const fn discovery(self) -> Duration {
        self.discovery
    }

    /// Return the maximum inactivity between accepted transfer requests.
    #[must_use]
    pub const fn block_inactivity(self) -> Duration {
        self.block_inactivity
    }

    /// Return the maximum duration of the complete OTA offer.
    #[must_use]
    pub const fn total_transfer(self) -> Duration {
        self.total_transfer
    }
}

impl Default for UpdateTimeouts {
    fn default() -> Self {
        Self::new(
            DEFAULT_DISCOVERY_TIMEOUT,
            DEFAULT_BLOCK_INACTIVITY_TIMEOUT,
            DEFAULT_TOTAL_TRANSFER_TIMEOUT,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DEFAULT_BLOCK_INACTIVITY_TIMEOUT, DEFAULT_DISCOVERY_TIMEOUT,
        DEFAULT_TOTAL_TRANSFER_TIMEOUT, UpdateTimeouts,
    };

    #[test]
    fn default_uses_bounded_deadlines() {
        let timeouts = UpdateTimeouts::default();

        assert_eq!(timeouts.discovery(), DEFAULT_DISCOVERY_TIMEOUT);
        assert_eq!(
            timeouts.block_inactivity(),
            DEFAULT_BLOCK_INACTIVITY_TIMEOUT
        );
        assert_eq!(timeouts.total_transfer(), DEFAULT_TOTAL_TRANSFER_TIMEOUT);
    }
}
