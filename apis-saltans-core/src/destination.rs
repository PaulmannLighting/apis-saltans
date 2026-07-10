pub use self::broadcast::Broadcast;
pub use self::device::Device;
use crate::GroupId;

mod broadcast;
mod device;

/// Zigbee destination used by outgoing NWK transmissions.
///
/// Device and broadcast destinations carry both the NWK address selector and
/// the APS endpoint that should receive the payload. Group destinations carry
/// only the group identifier because group membership is endpoint-local on each
/// receiving node.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Destination {
    /// Send to one device short address and endpoint.
    Device(Device),

    /// Send to a Zigbee broadcast receiver set and APS endpoint.
    Broadcast(Broadcast),

    /// Send to all members of an APS group.
    ///
    /// The group identifier is carried as the destination address and endpoint
    /// selection is resolved by each receiver's group table.
    Group(GroupId),
}

impl_fmt_enum! {
    Destination {
        Device(Device) => "Device",
        Broadcast(Broadcast) => "Broadcast",
        Group(GroupId) => "Group",
    }
}
