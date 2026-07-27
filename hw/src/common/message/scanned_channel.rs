use super::Channel;

/// A structure representing the result of a channel scan operation.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ScannedChannel {
    channel: Channel,
    max_rssi_dbm: i8,
}

impl ScannedChannel {
    /// Create a new `ScannedChannel`.
    #[must_use]
    pub const fn new(channel: Channel, max_rssi_dbm: i8) -> Self {
        Self {
            channel,
            max_rssi_dbm,
        }
    }

    /// Get the channel number.
    #[must_use]
    pub const fn channel(&self) -> Channel {
        self.channel
    }

    /// Return the maximum RSSI observed on this channel in dBm.
    #[must_use]
    pub const fn max_rssi_dbm(&self) -> i8 {
        self.max_rssi_dbm
    }
}
