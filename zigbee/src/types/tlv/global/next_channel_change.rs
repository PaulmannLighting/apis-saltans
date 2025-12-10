use le_stream::{FromLeStream, ToLeStream};

use crate::types::ChannelsField;
use crate::types::tlv::Tag;

/// Next Channel Change TLV structure.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct NextChannelChange {
    next_channel: ChannelsField,
}

impl NextChannelChange {
    /// Get the next channel field.
    #[must_use]
    pub const fn next_channel(self) -> ChannelsField {
        self.next_channel
    }
}

impl Tag for NextChannelChange {
    const TAG: u8 = 68;
}
