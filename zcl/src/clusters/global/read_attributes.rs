//! Reading Attributes Command and Response.

use core::iter::Empty;
use core::ops::Deref;
use std::boxed::Box;
use std::collections::BTreeMap;
use std::collections::btree_map::IntoIter;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::types::Type;
use zigbee::{Direction, ExpectResponse};

pub use self::read_attributes_status::ReadAttributesStatus;
use crate::command::Scoped;
use crate::{ParseAttributeError, ReadableAttribute, Scope, global};

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

impl ExpectResponse<crate::Cluster> for Command {
    type Response = Response;
}

impl From<Command> for crate::Cluster {
    fn from(command: Command) -> Self {
        Self::Global(command.into())
    }
}

/// Read Attributes Response.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Response {
    attribute_values: BTreeMap<u16, Type>,
}

impl Response {
    /// Creates a new instance of [`Response`].
    #[must_use]
    pub const fn new(attribute_values: BTreeMap<u16, Type>) -> Self {
        Self { attribute_values }
    }

    /// Returns an iterator over the parsed attribute values in the response.
    pub fn parse<T>(self) -> impl Iterator<Item = Result<T::Attribute, ParseAttributeError<T>>>
    where
        T: ReadableAttribute,
    {
        self.attribute_values.into_iter().map(|(id, typ)| {
            T::try_from(id)
                .map_err(ParseAttributeError::InvalidId)
                .and_then(|id| T::Attribute::try_from((id, typ)).map_err(Into::into))
        })
    }
}

impl crate::Command for Response {
    const ID: u8 = 0x01;
    const DIRECTION: Direction = Direction::ServerToClient;
}

impl Scoped for Response {
    const SCOPE: Scope = Scope::Global;
}

impl From<Response> for crate::Cluster {
    fn from(command: Response) -> Self {
        Self::Global(command.into())
    }
}

impl<T> From<Response> for Box<[Result<T::Attribute, ParseAttributeError<T>>]>
where
    T: ReadableAttribute,
{
    fn from(response: Response) -> Self {
        response.parse::<T>().collect()
    }
}

impl TryFrom<crate::Cluster> for Response {
    type Error = crate::Cluster;

    fn try_from(cluster: crate::Cluster) -> Result<Self, Self::Error> {
        if let crate::Cluster::Global(global::Command::ReadAttributesResponse(response)) = cluster {
            Ok(response)
        } else {
            Err(cluster)
        }
    }
}

impl Deref for Response {
    type Target = BTreeMap<u16, Type>;

    fn deref(&self) -> &Self::Target {
        &self.attribute_values
    }
}

impl IntoIterator for Response {
    type Item = (u16, Type);
    type IntoIter = IntoIter<u16, Type>;

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
                .filter_map(|(attribute_id, result)| result.ok().map(|value| (attribute_id, value)))
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
