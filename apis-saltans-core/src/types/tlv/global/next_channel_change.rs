use le_stream::{FromLeStream, ToLeStream};

use crate::types::ChannelsField;
use crate::types::tlv::Tag;

/// Next Channel Change TLV structure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct NextChannelChange {
    next_channel: ChannelsField,
}

impl NextChannelChange {
    /// Create a new `NextChannelChange`.
    #[must_use]
    pub const fn new(next_channel: ChannelsField) -> Self {
        Self { next_channel }
    }

    /// Get the next channel field.
    #[must_use]
    pub const fn next_channel(self) -> ChannelsField {
        self.next_channel
    }
}

impl Tag for NextChannelChange {
    const TAG: u8 = 68;
}

impl From<NextChannelChange> for ChannelsField {
    fn from(value: NextChannelChange) -> Self {
        value.next_channel
    }
}

impl From<ChannelsField> for NextChannelChange {
    fn from(next_channel: ChannelsField) -> Self {
        Self { next_channel }
    }
}
