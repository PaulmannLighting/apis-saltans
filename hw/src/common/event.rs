use bytes::Bytes;
use zb_aps::Data;
use zb_core::FullAddress;
use zb_nwk::Envelope;

pub use self::route_error::RouteError;

mod route_error;

/// Events emitted by the hardware layer.
///
/// Device membership events carry a [`FullAddress`] so consumers receive both
/// the IEEE address and the current NWK short address for the affected device.
#[derive(Clone, Debug)]
pub enum Event {
    /// The network is up and running.
    NetworkUp,

    /// The network is down.
    NetworkDown,

    /// The network has been opened for new joins.
    NetworkOpened,

    /// The network has been closed for new joins.
    NetworkClosed,

    /// A new device has joined the network.
    DeviceJoined(FullAddress),

    /// A known device has rejoined the network.
    DeviceRejoined {
        /// Complete address of the rejoining device.
        address: FullAddress,

        /// Whether the rejoining was secured.
        secured: bool,
    },

    /// A device has left the network.
    DeviceLeft(FullAddress),

    /// Raw APS data frame received from a NWK source.
    MessageReceived(Envelope<Data<Bytes>>),

    /// Result of an acknowledged APS transmission.
    ///
    /// A successful result contains the APS frame counter assigned to the transmitted frame.
    ApsResponse(Result<u8, crate::Error>),

    /// A routing error.
    RouteError(RouteError),
}
