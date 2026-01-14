//! Zigbee Network (NWK) Layer implementation.

pub use network_manager::NetworkManager;
use tokio::sync::mpsc::{Receiver, Sender};

pub use self::actor::Actor;
pub use self::aps::{Frame, Metadata};
pub use self::error::Error;
pub use self::event::{Command, Event};
pub use self::message::{FoundNetwork, Network, ScannedChannel};
pub use self::proxy::Proxy;
pub use self::waiter::Waiter;

mod actor;
mod aps;
mod binding;
mod device_proxy;
mod endpoint_proxy;
mod error;
mod event;
mod message;
mod network_manager;
mod proxy;
mod waiter;
mod zcl_proxy;
mod zdp_proxy;

/// Type alias for the NWK layer proxy sender.
pub type ZigbeeTransmitter = Sender<message::Message>;

/// Type alias for the NWK layer actor receiver.
pub type ZigbeeReceiver = Receiver<message::Message>;
