use zb_core::FullAddress;

pub use self::zcl::Zcl;

mod zcl;

/// An event published by the coordinator.
///
/// Subscribe with [`crate::NetworkManager::subscribe`] to receive events published by the network
/// manager.
#[derive(Clone, Debug)]
pub enum Event {
    /// A device joined the network.
    DeviceJoined(FullAddress),

    /// A device left the network.
    DeviceLeft(FullAddress),

    /// A device announced itself on the network.
    DeviceAnnounced(FullAddress),

    /// An unsolicited ZCL command was received.
    Zcl(Zcl),
}
