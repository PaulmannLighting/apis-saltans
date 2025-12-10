use le_stream::FromLeStreamTagged;

/// Local TLV structure.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Local {
    tag: u8,
    data: Vec<u8>,
}

impl Local {
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
