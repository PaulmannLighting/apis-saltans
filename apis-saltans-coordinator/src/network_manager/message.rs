use std::collections::BTreeSet;

use apis_saltans_aps::Data;
use apis_saltans_core::Address;
use apis_saltans_hw::RouteError;
use apis_saltans_zcl::{Cluster, Frame};
use macaddr::MacAddr8;
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
        devices: BTreeSet<MacAddr8>,
        /// The sender to send the incoming commands to.
        sender: Sender<Event>,
    },

    /// An incoming ZCL command.
    Command {
        /// The source address of the command.
        src_address: u16,
        /// The payload of the command.
        payload: Box<Data<Frame<Cluster>>>,
    },

    /// A request to resolve a short ID to an IEEE address.
    GetIeeeAddressFromShortId {
        /// The short ID to resolve.
        short_id: u16,
        /// Response channel to send the resolved IEEE address to.
        response: oneshot::Sender<Option<MacAddr8>>,
    },

    /// A request to resolve an IEEE address to a short ID.
    GetShortIdFromIeeeAddress {
        /// The IEEE address to resolve.
        ieee_address: MacAddr8,
        /// Response channel to send the resolved IEEE address to.
        response: oneshot::Sender<Option<u16>>,
    },

    /// A device joined the network.
    DeviceJoined {
        /// The address of the device.
        address: Address,
        /// Whether the rejoin was secured.
        secured: Option<bool>,
    },

    /// Add a new device to the network.
    NewDevice(Device),

    /// Remove a device from the network.
    RemoveDevice(Address),

    /// A routing error.
    RouteError(RouteError),

    /// The network has been opened for joining.
    NetworkOpened,

    /// The network has been closed for joining.
    NetworkClosed,
}
