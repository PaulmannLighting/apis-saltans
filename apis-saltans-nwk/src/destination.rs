use apis_saltans_core::{Broadcast, Device, GroupId};

/// Zigbee destination address.
///
/// This type models only the address selector used to reach receiving nodes or
/// groups. APS endpoint selection is represented separately by APS-layer types.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Destination {
    /// A single allocated device short address.
    Device(Device),

    /// An APS group identifier.
    Group(GroupId),

    /// A Zigbee broadcast short address.
    Broadcast(Broadcast),
}

impl Destination {
    /// Return the raw 16-bit destination address value.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        match self {
            Self::Device(device) => device.as_u16(),
            Self::Group(group) => group.as_u16(),
            Self::Broadcast(broadcast) => broadcast.as_u16(),
        }
    }
}

impl From<Device> for Destination {
    fn from(device: Device) -> Self {
        Self::Device(device)
    }
}

impl From<GroupId> for Destination {
    fn from(group: GroupId) -> Self {
        Self::Group(group)
    }
}

impl From<Broadcast> for Destination {
    fn from(broadcast: Broadcast) -> Self {
        Self::Broadcast(broadcast)
    }
}

impl From<Destination> for u16 {
    fn from(destination: Destination) -> Self {
        destination.as_u16()
    }
}
