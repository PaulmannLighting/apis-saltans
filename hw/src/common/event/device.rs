use zb_core::FullAddress;

/// Device membership events emitted by the hardware layer.
///
/// Each event carries a [`FullAddress`] so consumers receive both the IEEE
/// address and the current NWK short address for the affected device.
#[derive(Clone, Debug)]
pub enum DeviceEvent {
    /// A new device has joined the network.
    Joined(FullAddress),

    /// A known device has rejoined the network.
    Rejoined {
        /// Complete address of the rejoining device.
        address: FullAddress,

        /// Whether the rejoining was secured.
        secured: bool,
    },

    /// A device has left the network.
    Left(FullAddress),
}
