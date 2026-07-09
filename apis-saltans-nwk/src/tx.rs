use apis_saltans_core::{
    BroadcastAddress, BroadcastEndpoint, Cluster, ClusterSpecific, Device, Endpoint, GroupId,
    Profiled,
};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Message<T> {
    destination: Destination,
    profile_id: u16,
    cluster_id: u16,
    payload: T,
}

impl<T> Message<T> {
    pub const fn new(
        destination: Destination,
        profile_id: u16,
        cluster_id: u16,
        payload: T,
    ) -> Self {
        Self {
            destination,
            profile_id,
            cluster_id,
            payload,
        }
    }

    #[must_use]
    pub fn cluster_specific(destination: Destination, payload: T) -> Self
    where
        T: ClusterSpecific + Profiled,
    {
        Self::new(destination, T::PROFILE.into(), T::ID, payload)
    }

    #[must_use]
    pub fn global(destination: Destination, cluster: Cluster, payload: T) -> Self
    where
        T: Profiled,
    {
        Self::new(destination, T::PROFILE.into(), cluster.as_u16(), payload)
    }
}

/// Zigbee destination used by outgoing NWK transmissions.
///
/// Device and broadcast destinations carry both the NWK address selector and
/// the APS endpoint that should receive the payload. Group destinations carry
/// only the group identifier because group membership is endpoint-local on each
/// receiving node.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        address: BroadcastAddress,

        /// APS endpoint selector used by each receiving node.
        endpoint: BroadcastEndpoint,
    },

    /// Send to all members of an APS group.
    ///
    /// The group identifier is carried as the destination address and endpoint
    /// selection is resolved by each receiver's group table.
    Group(GroupId),
}
