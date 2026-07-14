use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use zb_aps::Data;
use zb_core::{FullAddress, IeeeAddress, short_id};
use zb_hw::RouteError;
use zb_nwk::Source;
use zb_zcl::{Cluster, Frame};

use super::Device;
use crate::Event;

/// Messages received by the network management actor.
#[derive(Debug)]
pub enum Message {
    /// Subscribe to events.
    Subscribe(Sender<Event>),

    /// An incoming ZCL command.
    Command {
        /// The NWK source of the command.
        source: Source,
        /// The payload of the command.
        frame: Data<Frame<Cluster>>,
    },

    /// A request to resolve a short ID to an IEEE address.
    GetIeeeAddressFromShortId {
        /// The short ID to resolve.
        short_id: short_id::Device,
        /// Response channel to send the resolved IEEE address to.
        response: oneshot::Sender<Option<IeeeAddress>>,
    },

    /// A request to resolve an IEEE address to a short ID.
    GetShortIdFromIeeeAddress {
        /// The IEEE address to resolve.
        ieee_address: IeeeAddress,
        /// Response channel to send the resolved IEEE address to.
        response: oneshot::Sender<Option<short_id::Device>>,
    },

    /// A device joined the network.
    DeviceJoined {
        /// The address of the device.
        address: FullAddress,
        /// Whether the rejoin was secured.
        secured: Option<bool>,
    },

    /// Add a new device to the network.
    NewDevice {
        /// The address of the device.
        address: FullAddress,
        /// The device that was added.
        device: Device,
    },

    /// Remove a device from the network.
    RemoveDevice(IeeeAddress),

    /// A routing error.
    RouteError(RouteError),

    /// Get devices.
    GetDevices(oneshot::Sender<Box<[Device]>>),

    /// The network has been opened for joining.
    NetworkOpened,

    /// The network has been closed for joining.
    NetworkClosed,
}
