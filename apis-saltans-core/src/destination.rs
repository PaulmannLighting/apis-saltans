use crate::{Endpoint, GroupId, endpoint, short_id};

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

/// Device destination with a short address and APS endpoint.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Device {
    device: short_id::Device,
    endpoint: Endpoint,
}

impl Device {
    #[must_use]
    pub const fn new(device: short_id::Device, endpoint: Endpoint) -> Self {
        Self { device, endpoint }
    }

    #[must_use]
    pub const fn device(&self) -> short_id::Device {
        self.device
    }

    #[must_use]
    pub const fn endpoint(&self) -> Endpoint {
        self.endpoint
    }
}

/// Broadcast destination with a broadcast short address and broadcast endpoint.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Broadcast {
    address: short_id::Broadcast,
    endpoint: endpoint::Broadcast,
}

impl Broadcast {
    #[must_use]
    pub const fn new(address: short_id::Broadcast, endpoint: endpoint::Broadcast) -> Self {
        Self { address, endpoint }
    }

    #[must_use]
    pub const fn address(&self) -> short_id::Broadcast {
        self.address
    }

    #[must_use]
    pub const fn endpoint(&self) -> endpoint::Broadcast {
        self.endpoint
    }
}
