mod read_attributes_status;

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::ops::Deref;

use le_stream::FromLeStream;
use zigbee::types::Type;

use self::read_attributes_status::ReadAttributesStatus;

/// Read Attributes Command.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Command {
    attribute_ids: Box<[u16]>,
}

impl Command {
    /// Creates a new `Read Attributes` command with the given attribute IDs.
    #[must_use]
    pub const fn new(attribute_ids: Box<[u16]>) -> Self {
        Self { attribute_ids }
    }

    /// Returns the attribute IDs of the command.
    pub fn attribute_ids(&self) -> &[u16] {
        &self.attribute_ids
    }
}

/// Read Attributes Response.
pub struct Response {
    attribute_values: BTreeMap<u16, Result<Type, u8>>,
}

impl Deref for Response {
    type Target = BTreeMap<u16, Result<Type, u8>>;

    fn deref(&self) -> &Self::Target {
        &self.attribute_values
    }
}

impl FromLeStream for Response {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        Vec::<ReadAttributesStatus>::from_le_stream(bytes).map(|items| Self {
            attribute_values: items
                .into_iter()
                .map(ReadAttributesStatus::into_parts)
                .collect(),
        })
    }
}
