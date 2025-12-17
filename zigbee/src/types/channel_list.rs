use std::fmt::Display;

use le_stream::{FromLeStream, Prefixed, ToLeStream};

use crate::types::ChannelsField;

/// Channel List structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct ChannelList {
    pages: Prefixed<u8, Box<[ChannelsField]>>,
}

impl ChannelList {
    /// Creates a new `ChannelList`.
    ///
    /// # Errors
    ///
    /// Returns the input `pages` if the length exceeds `u8::MAX`.
    pub fn new(pages: Box<[ChannelsField]>) -> Result<Self, Box<[ChannelsField]>> {
        pages.try_into().map(|pages| Self { pages })
    }

    /// Returns the pages.
    #[must_use]
    pub fn pages(&self) -> &[ChannelsField] {
        &self.pages
    }
}

impl Display for ChannelList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl TryFrom<Box<[ChannelsField]>> for ChannelList {
    type Error = Box<[ChannelsField]>;

    fn try_from(value: Box<[ChannelsField]>) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<Vec<ChannelsField>> for ChannelList {
    type Error = Box<[ChannelsField]>;

    fn try_from(value: Vec<ChannelsField>) -> Result<Self, Self::Error> {
        Self::try_from(value.into_boxed_slice())
    }
}
