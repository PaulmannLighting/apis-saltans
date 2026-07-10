use zb_core::FullAddress;
use zb_core::node::MacCapabilityFlags;

/// Message sent to the discovery actor.
#[derive(Debug)]
pub enum Message {
    /// A device has been announced.
    DeviceAnnounced {
        /// The address of the device.
        address: FullAddress,
        /// The capabilities of the device.
        capabilities: MacCapabilityFlags,
    },

    /// An administrative discovery sent by the network manager.
    AdministrativeDiscovery(FullAddress),
}
