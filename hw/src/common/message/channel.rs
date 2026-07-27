use std::fmt::{Display, Formatter};

const MIN_CHANNEL: u8 = 11;
const MAX_CHANNEL: u8 = 26;

/// Zigbee channel on channel page zero.
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(try_from = "u8", into = "u8")
)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Channel(u8);

impl Channel {
    /// Lowest Zigbee channel on channel page zero.
    pub const MIN: Self = Self(MIN_CHANNEL);

    /// Highest Zigbee channel on channel page zero.
    pub const MAX: Self = Self(MAX_CHANNEL);

    /// Create a channel when the number is within the page-zero Zigbee range.
    #[must_use]
    pub const fn new(channel: u8) -> Option<Self> {
        if channel >= MIN_CHANNEL && channel <= MAX_CHANNEL {
            Some(Self(channel))
        } else {
            None
        }
    }

    /// Return the channel number.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        self.0
    }
}

impl Display for Channel {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl From<Channel> for u8 {
    fn from(channel: Channel) -> Self {
        channel.0
    }
}

impl TryFrom<u8> for Channel {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{Channel, MAX_CHANNEL, MIN_CHANNEL};

    const BELOW_MIN_CHANNEL: u8 = MIN_CHANNEL - 1;
    const ABOVE_MAX_CHANNEL: u8 = MAX_CHANNEL + 1;

    #[test]
    fn accepts_only_page_zero_zigbee_channels() {
        assert_eq!(Channel::new(MIN_CHANNEL), Some(Channel::MIN));
        assert_eq!(Channel::new(MAX_CHANNEL), Some(Channel::MAX));
        assert_eq!(Channel::new(BELOW_MIN_CHANNEL), None);
        assert_eq!(Channel::new(ABOVE_MAX_CHANNEL), None);
    }
}
