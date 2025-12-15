use std::ops::Deref;

use le_stream::{FromLeStream, Prefixed, ToLeStream};

use crate::types::ChannelsField;

type ByteSizedVec<T> = heapless::Vec<T, { u8::MAX as usize }>;

/// Channel List structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct ChannelList {
    pages: Prefixed<u8, ByteSizedVec<ChannelsField>>,
}

impl ChannelList {
    /// Creates a new `ChannelList`.
    #[must_use]
    pub const fn new(pages: ByteSizedVec<ChannelsField>) -> Self {
        Self {
            pages: Prefixed::new(pages),
        }
    }

    /// Returns the pages.
    #[must_use]
    pub fn pages(&self) -> &[ChannelsField] {
        &self.pages
    }
}

impl Deref for ChannelList {
    type Target = [ChannelsField];

    fn deref(&self) -> &Self::Target {
        &self.pages
    }
}

impl From<ByteSizedVec<ChannelsField>> for ChannelList {
    fn from(value: ByteSizedVec<ChannelsField>) -> Self {
        Self::new(value)
    }
}
