//! Zigbee Network (NWK) Layer implementation.

use tokio::sync::mpsc::{Receiver, Sender};

pub use self::actor::Actor;
pub use self::coordinator::Coordinator;
pub use self::demux::Demux;
pub use self::error::Error;
pub use self::event::{Command, Event};
pub use self::message::{FoundNetwork, Network, ScannedChannel};
pub use self::ncp::Ncp;
pub use self::transmission::{Frame, Metadata};
pub use self::waiter::Waiter;
pub use self::zcl::{
    Attributes, Binding, ColorControl, DeviceProxy, EndpointProxy, OnOff, Transceiver, Transmitter,
    ZclTransceiver,
};

mod actor;
mod coordinator;
mod demux;
mod error;
mod event;
mod message;
mod ncp;
pub mod smarthomelib;
mod transceiver;
mod transmission;
mod waiter;
mod zcl;

/// Type alias for the NWK layer proxy sender.
pub type ZigbeeTransmitter = Sender<message::Message>;

/// Type alias for the NWK layer actor receiver.
pub type ZigbeeReceiver = Receiver<message::Message>;
