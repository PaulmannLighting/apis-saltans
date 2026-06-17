use core::fmt::{self, Display};

use le_stream::{FromLeStream, ToLeStream};

use crate::types::ChannelsField;

/// Channel List pages.
pub type Pages = heapless::Vec<ChannelsField, { u8::MAX as usize }, u8>;

/// Channel List structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct ChannelList {
    pages: heapless::Vec<ChannelsField, { u8::MAX as usize }, u8>,
}

impl ChannelList {
    /// Creates a new `ChannelList`.
    pub fn new(pages: Pages) -> Self {
        Self { pages }
    }

    /// Returns the pages.
    #[must_use]
    pub fn pages(&self) -> &[ChannelsField] {
        &self.pages
    }
}

impl Display for ChannelList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut channels = self.pages.iter();

        if let Some(channel) = channels.next() {
            write!(f, "{channel:#010X}")?;

            for channel in channels {
                write!(f, ", {channel:#010X}")?;
            }
        }

        write!(f, "]")
    }
}

impl From<Pages> for ChannelList {
    fn from(pages: Pages) -> Self {
        Self::new(pages)
    }
}
