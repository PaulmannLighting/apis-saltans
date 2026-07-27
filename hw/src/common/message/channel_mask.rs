use std::iter::FromIterator;

use super::Channel;

const SINGLE_CHANNEL_BIT: u32 = 1;
const VALID_CHANNEL_MASK: u32 = 0x07FF_F800;

/// Bit mask selecting channels on Zigbee channel page zero.
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ChannelMask(u32);

impl ChannelMask {
    /// Mask selecting every Zigbee channel on channel page zero.
    pub const ALL: Self = Self(VALID_CHANNEL_MASK);

    /// Create a channel mask if it contains no unsupported channel bits.
    #[must_use]
    pub const fn new(mask: u32) -> Option<Self> {
        if mask & !VALID_CHANNEL_MASK == 0 {
            Some(Self(mask))
        } else {
            None
        }
    }

    /// Create an empty channel mask.
    #[must_use]
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Return the raw page-zero channel bits.
    #[must_use]
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Return whether the mask contains a channel.
    #[must_use]
    pub const fn contains(self, channel: Channel) -> bool {
        self.0 & (SINGLE_CHANNEL_BIT << channel.as_u8()) != 0
    }
}

impl FromIterator<Channel> for ChannelMask {
    fn from_iter<T>(channels: T) -> Self
    where
        T: IntoIterator<Item = Channel>,
    {
        let mut mask = 0;

        for channel in channels {
            mask |= SINGLE_CHANNEL_BIT << channel.as_u8();
        }

        Self(mask)
    }
}

impl From<ChannelMask> for u32 {
    fn from(mask: ChannelMask) -> Self {
        mask.0
    }
}

impl TryFrom<u32> for ChannelMask {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{Channel, ChannelMask, SINGLE_CHANNEL_BIT};

    const INVALID_CHANNEL_BIT: u32 = SINGLE_CHANNEL_BIT;

    #[test]
    fn rejects_bits_outside_page_zero_channels() {
        assert_eq!(ChannelMask::new(INVALID_CHANNEL_BIT), None);
        assert_eq!(
            ChannelMask::new(ChannelMask::ALL.bits()),
            Some(ChannelMask::ALL)
        );
    }

    #[test]
    fn collects_channels_into_a_mask() {
        let mask: ChannelMask = [Channel::MIN, Channel::MAX].into_iter().collect();

        assert!(mask.contains(Channel::MIN));
        assert!(mask.contains(Channel::MAX));
    }
}
