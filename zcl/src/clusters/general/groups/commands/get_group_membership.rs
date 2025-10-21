use core::iter::Chain;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::types::{Uint8, Uint16};

use crate::clusters::general::groups::CLUSTER_ID;
use crate::clusters::general::groups::types::GroupList;
use crate::{Cluster, Command};

/// Command to request the membership of a device in multiple groups.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GetGroupMembership {
    groups: GroupList,
}

impl GetGroupMembership {
    /// Creates a new `GetGroupMembership` command with the specified group count and list.
    #[must_use]
    pub const fn new(groups: GroupList) -> Self {
        Self { groups }
    }

    /// Return the groups the sender is a member of.
    #[must_use]
    pub fn groups(&self) -> &[Uint16] {
        self.groups.as_ref()
    }
}

impl AsRef<[Uint16]> for GetGroupMembership {
    fn as_ref(&self) -> &[Uint16] {
        self.groups()
    }
}

impl Cluster for GetGroupMembership {
    const ID: u16 = CLUSTER_ID;
}

impl Command for GetGroupMembership {
    const ID: u8 = 0x02;
}

impl FromLeStream for GetGroupMembership {
    fn from_le_stream<I>(mut bytes: I) -> Option<Self>
    where
        I: Iterator<Item = u8>,
    {
        let group_count = Uint8::from_le_stream(&mut bytes)?;
        let mut groups = GroupList::new();

        let Ok(size) = u8::try_from(group_count) else {
            return None;
        };

        for _ in 0..size {
            groups.push(Uint16::from_le_stream(&mut bytes)?).ok()?;
        }

        Some(Self { groups })
    }
}

impl ToLeStream for GetGroupMembership {
    type Iter = Chain<<Uint8 as ToLeStream>::Iter, <GroupList as ToLeStream>::Iter>;

    fn to_le_stream(self) -> Self::Iter {
        Uint8::try_from(self.groups.len())
            .expect("GroupList size always fits into a Uint8.")
            .to_le_stream()
            .chain(self.groups.to_le_stream())
    }
}
