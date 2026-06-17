use std::collections::BTreeMap;

use zigbee::{Address, ClusterId, Endpoint};

use crate::Endpoint as EndpointInfo;

/// Messages received by the binding management actor.
#[derive(Debug)]
pub enum Message {
    /// Information that a certain device has been updated.
    DeviceDiscovered {
        /// The address of the device that has been updated.
        address: Address,
        /// The new device endpoints, keyed by endpoint ID.
        endpoints: BTreeMap<Endpoint, EndpointInfo>,
    },

    /// Signal that an endpoint has been bound to a cluster.
    EndpointBound {
        /// The address of the device that the endpoint belongs to.
        address: Address,
        /// The endpoint that has been bound.
        endpoint: Endpoint,
        /// The cluster that has been bound.
        cluster: ClusterId,
    },
}
