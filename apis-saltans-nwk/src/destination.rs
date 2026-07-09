use apis_saltans_core::{Broadcast, Device, Endpoint, GroupId};

/// Zigbee destination used by outgoing NWK transmissions.
///
/// Device and broadcast destinations carry both the NWK address selector and
/// the APS endpoint that should receive the payload. Group destinations carry
/// only the group identifier because group membership is endpoint-local on each
/// receiving node.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Destination {
    /// Send to one device short address and endpoint.
    Device {
        /// Device short address that receives the frame.
        device: Device,

        /// APS endpoint on the receiving device.
        endpoint: Endpoint,
    },

    /// Send to a Zigbee broadcast receiver set and APS endpoint.
    Broadcast {
        /// NWK broadcast address selecting the receiving nodes.
        address: Broadcast,

        /// APS endpoint selector used by each receiving node.
        endpoint: Endpoint,
    },

    /// Send to all members of an APS group.
    ///
    /// The group identifier is carried as the destination address and endpoint
    /// selection is resolved by each receiver's group table.
    Group { group: GroupId, endpoint: Endpoint },
}
