use std::num::TryFromIntError;

use le_stream::FromLeStreamTagged;

/// Local TLV structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Local {
    tag: u8,
    data: Vec<u8>,
}

impl Local {
    /// Create a new `Local` TLV.
    ///
    /// # Errors
    ///
    /// If the length of `data` minus one cannot be represented as a `u8`, an error is returned.
    pub fn new(tag: u8, data: Vec<u8>) -> Result<Self, Option<TryFromIntError>> {
        let Some(len) = data.len().checked_sub(1) else {
            return Err(None);
        };

        u8::try_from(len).map(|_| Self { tag, data }).map_err(Some)
    }

    /// Get the tag.
    #[must_use]
    pub const fn tag(&self) -> u8 {
        self.tag
    }

    /// Get the data.
    #[must_use]
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl FromLeStreamTagged for Local {
    type Tag = u8;

    fn from_le_stream_tagged<T>(tag: Self::Tag, bytes: T) -> Result<Option<Self>, Self::Tag>
    where
        T: Iterator<Item = u8>,
    {
        Ok(Some(Self {
            tag,
            data: bytes.collect(),
        }))
    }
}
