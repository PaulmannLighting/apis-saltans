//! Reading Attributes Command and Response.

mod read_attributes_status;

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::iter::Empty;
use core::ops::Deref;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::Direction;
use zigbee::types::Type;

use self::read_attributes_status::ReadAttributesStatus;
use crate::Global;

/// Read Attributes Command.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
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
    #[must_use]
    pub fn attribute_ids(&self) -> &[u16] {
        &self.attribute_ids
    }
}

impl Global for Command {
    const ID: u8 = 0x00;
    const DIRECTION: Direction = Direction::ClientToServer;
}

/// Read Attributes Response.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Response {
    attribute_values: BTreeMap<u16, Result<Type, u8>>,
}

impl Global for Response {
    const ID: u8 = 0x01;
    const DIRECTION: Direction = Direction::ServerToClient;
}

impl Deref for Response {
    type Target = BTreeMap<u16, Result<Type, u8>>;

    fn deref(&self) -> &Self::Target {
        &self.attribute_values
    }
}

impl IntoIterator for Response {
    type Item = (u16, Result<Type, u8>);
    type IntoIter = alloc::collections::btree_map::IntoIter<u16, Result<Type, u8>>;

    fn into_iter(self) -> Self::IntoIter {
        self.attribute_values.into_iter()
    }
}

impl ToLeStream for Response {
    type Iter = Empty<u8>;

    fn to_le_stream(self) -> Self::Iter {
        todo!("Not implemented")
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
