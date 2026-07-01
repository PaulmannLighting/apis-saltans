use apis_saltans_core::{Address, ClusterId, Endpoint};

use crate::Device;

/// Messages received by the binding management actor.
#[derive(Debug)]
pub enum Message {
    /// Information that a certain device has been updated.
    DeviceDiscovered(Box<Device>),

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
