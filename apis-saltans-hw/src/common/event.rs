use apis_saltans_aps::Data;
use apis_saltans_core::IeeeAddress;
use apis_saltans_core::short_id::Device;
use apis_saltans_nwk::Envelope;
use bytes::Bytes;

pub use self::route_error::RouteError;

mod route_error;

/// Events that can occur on the hardware layer.
#[derive(Clone, Debug, Eq, PartialEq)]
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
    DeviceJoined {
        ieee_address: IeeeAddress,
        short_id: Device,
    },

    /// A device has rejoined the network.
    DeviceRejoined {
        ieee_address: IeeeAddress,
        short_id: Device,
        /// Whether the rejoining was secured.
        secured: bool,
    },

    /// A device has left the network.
    DeviceLeft {
        ieee_address: IeeeAddress,
        short_id: Device,
    },

    /// Raw APS data frame received from a NWK source.
    MessageReceived(Envelope<Data<Bytes>>),

    /// A routing error.
    RouteError(RouteError),
}
