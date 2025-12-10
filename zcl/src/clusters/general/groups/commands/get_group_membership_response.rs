use core::iter::Chain;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::types::{Uint8, Uint16};
use zigbee::{Cluster, Command, Direction};

use crate::clusters::general::groups::CLUSTER_ID;
use crate::clusters::general::groups::types::GroupList;

/// Represents a response to an `GetGroupMembership` command.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GetGroupMembershipResponse {
    capacity: Uint8,
    groups: GroupList,
}

impl GetGroupMembershipResponse {
    /// Creates a new `GetGroupMembershipResponse` with the given status and group ID.
    #[must_use]
    pub const fn new(capacity: Uint8, groups: GroupList) -> Self {
        Self { capacity, groups }
    }

    /// Return the remaining capacity of the group table.
    #[must_use]
    pub const fn capacity(&self) -> Uint8 {
        self.capacity
    }

    /// Return the groups in the group table.
    #[must_use]
    pub fn groups(&self) -> &[Uint16] {
        &self.groups
    }

    /// Return the group count.
    ///
    /// # Panics
    ///
    /// This function will panic if the amount of groups exceeds [`Uint8::MAX`], which should never happen.
    #[must_use]
    pub fn group_count(&self) -> Uint8 {
        self.groups
            .len()
            .try_into()
            .expect("GroupList size always fits into a Uint8.")
    }
}

impl AsRef<[Uint16]> for GetGroupMembershipResponse {
    fn as_ref(&self) -> &[Uint16] {
        &self.groups
    }
}

impl IntoIterator for GetGroupMembershipResponse {
    type Item = Uint16;
    type IntoIter = <GroupList as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.groups.into_iter()
    }
}

impl Cluster for GetGroupMembershipResponse {
    const ID: u16 = CLUSTER_ID;
}

impl Command for GetGroupMembershipResponse {
    const ID: u8 = 0x02;
    const DIRECTION: Direction = Direction::ServerToClient;
}

impl FromLeStream for GetGroupMembershipResponse {
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let capacity = Uint8::from_le_stream(&mut bytes)?;
        let group_count = Uint8::from_le_stream(&mut bytes)?;
        let mut groups = GroupList::new();

        let Ok(size) = u8::try_from(group_count) else {
            return None;
        };

        for _ in 0..size {
            groups.push(Uint16::from_le_stream(&mut bytes)?).ok()?;
        }

        Some(Self { capacity, groups })
    }
}

impl ToLeStream for GetGroupMembershipResponse {
    type Iter = Chain<
        Chain<<Uint8 as ToLeStream>::Iter, <Uint8 as ToLeStream>::Iter>,
        <GroupList as ToLeStream>::Iter,
    >;

    fn to_le_stream(self) -> Self::Iter {
        self.capacity
            .to_le_stream()
            .chain(self.group_count().to_le_stream())
            .chain(self.groups.to_le_stream())
    }
}
