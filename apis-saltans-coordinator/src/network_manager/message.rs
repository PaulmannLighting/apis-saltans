use std::collections::BTreeSet;

use apis_saltans_aps::Data;
use apis_saltans_core::{FullAddress, IeeeAddress, short_id};
use apis_saltans_hw::RouteError;
use apis_saltans_nwk::Source;
use apis_saltans_zcl::{Cluster, Frame};
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

use super::Device;
use crate::Event;

/// Messages received by the network management actor.
#[derive(Debug)]
pub enum Message {
    /// Subscribe to incoming ZCL commands.
    SubscribeToIncomingCommands {
        /// The source addresses of the devices to listen to.
        ///
        /// An empty set means that all devices will be listened to.
        devices: BTreeSet<IeeeAddress>,
        /// The sender to send the incoming commands to.
        sender: Sender<Event>,
    },

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
    NewDevice(Device),

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
