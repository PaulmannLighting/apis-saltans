//! Zigbee Network (NWK) Layer implementation.

use tokio::sync::mpsc::{Receiver, Sender};

pub use self::actor::Actor;
pub use self::aps::{Frame, Metadata};
pub use self::error::Error;
pub use self::event::{Command, Event};
pub use self::message::{FoundNetwork, Network, ScannedChannel};
pub use self::network_manager::NetworkManager;
pub use self::proxy::Proxy;
pub use self::waiter::Waiter;
pub use self::zcl::{Attributes, Binding, ColorControl, OnOff};

mod actor;
mod aps;
mod error;
mod event;
mod message;
mod network_manager;
mod proxies;
mod proxy;
mod waiter;
mod zcl;

/// Type alias for the NWK layer proxy sender.
pub type ZigbeeTransmitter = Sender<message::Message>;

/// Type alias for the NWK layer actor receiver.
pub type ZigbeeReceiver = Receiver<message::Message>;
