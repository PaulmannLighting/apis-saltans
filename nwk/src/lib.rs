//! Zigbee Network (NWK) Layer implementation.

use tokio::sync::mpsc::{Receiver, Sender};

pub use self::actor::Actor;
pub use self::error::Error;
pub use self::event::{Command, Event};
pub use self::frame::Frame;
pub use self::message::{FoundNetwork, Network, ScannedChannel};
pub use self::nlme::Nlme;
pub use self::proxy::Proxy;
pub use self::waiter::Waiter;

mod actor;
mod device_proxy;
mod endpoint_proxy;
mod error;
mod event;
mod frame;
mod message;
mod nlme;
mod proxy;
mod waiter;
mod zcl_proxy;

/// Type alias for the NWK layer proxy sender.
pub type ZigbeeTransmitter = Sender<message::Message>;

/// Type alias for the NWK layer actor receiver.
pub type ZigbeeReceiver = Receiver<message::Message>;
