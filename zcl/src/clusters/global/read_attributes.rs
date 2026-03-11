//! Reading Attributes Command and Response.

use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::collections::btree_map::IntoIter;
use core::iter::Empty;
use core::ops::Deref;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::Direction;
use zigbee::types::Type;

pub use self::read_attributes_status::ReadAttributesStatus;
use crate::command::Scoped;
use crate::{Customizable, Global, Scope};

mod read_attributes_status;

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

impl crate::Command for Command {
    const ID: u8 = 0x00;
    const DIRECTION: Direction = Direction::ClientToServer;
}

impl Scoped for Command {
    const SCOPE: Scope = Scope::Global;
}

impl Customizable for Command {}
impl Global for Command {}

/// Read Attributes Response.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Response {
    attribute_values: BTreeMap<u16, Result<Type, u8>>,
}

impl crate::Command for Response {
    const ID: u8 = 0x01;
    const DIRECTION: Direction = Direction::ServerToClient;
}

impl Scoped for Response {
    const SCOPE: Scope = Scope::Global;
}

impl Customizable for Response {}
impl Global for Response {}

impl Deref for Response {
    type Target = BTreeMap<u16, Result<Type, u8>>;

    fn deref(&self) -> &Self::Target {
        &self.attribute_values
    }
}

impl IntoIterator for Response {
    type Item = (u16, Result<Type, u8>);
    type IntoIter = IntoIter<u16, Result<Type, u8>>;

    fn into_iter(self) -> Self::IntoIter {
        self.attribute_values.into_iter()
    }
}

impl FromLeStream for Response {
    fn from_le_stream<T>(bytes: T) -> Option<Self>
    where
        T: Iterator<Item = u8>,
    {
        Box::<[ReadAttributesStatus]>::from_le_stream(bytes).map(|items| Self {
            attribute_values: items
                .into_iter()
                .map(ReadAttributesStatus::into_parts)
                .collect(),
        })
    }
}

impl ToLeStream for Response {
    type Iter = Empty<u8>;

    fn to_le_stream(self) -> Self::Iter {
        todo!("Not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::HeaderFactory;

    #[test]
    fn test_commutativeness() {
        let command = Command::new(Box::new([1, 2, 3]));
        let a = command.clone();
        let b = command;
        assert_eq!(
            a.with_manufacturer_code(Some(42))
                .for_cluster(123)
                .header(0x42),
            b.for_cluster(123)
                .with_manufacturer_code(Some(42))
                .header(0x42)
        );
    }

    #[test]
    fn test_header() {
        let header = Command::new(Box::new([1, 2, 3]))
            .for_cluster(123)
            .with_manufacturer_code(Some(42))
            .header(0x42);
        let control = header.control();
        assert_eq!(control.typ(), Ok(Scope::Global));
        assert_eq!(control.direction(), Direction::ClientToServer);
        assert!(!control.disable_default_response());
        assert_eq!(header.manufacturer_code(), Some(42));
        assert_eq!(header.seq(), 0x42);
        assert_eq!(header.command_id(), 0x00);
    }
}
