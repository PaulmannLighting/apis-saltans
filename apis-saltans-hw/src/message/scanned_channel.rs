/// A structure representing the result of a channel scan operation.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ScannedChannel {
    channel: u8,
    max_rssi_value: i8,
}

impl ScannedChannel {
    /// Create a new `ScannedChannel`.
    #[must_use]
    pub const fn new(channel: u8, max_rssi_value: i8) -> Self {
        Self {
            channel,
            max_rssi_value,
        }
    }

    /// Get the channel number.
    #[must_use]
    pub const fn channel(&self) -> u8 {
        self.channel
    }

    /// Get the maximum RSSI value observed on this channel.
    #[must_use]
    pub const fn max_rssi_value(&self) -> i8 {
        self.max_rssi_value
    }
}
