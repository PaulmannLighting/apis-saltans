//! Zigbee Network (NWK) Layer implementation.

pub use actor::{Actor, Proxy};
use tokio::sync::mpsc::{Receiver, Sender};
pub use {aps, zcl, zigbee};

pub use self::error::Error;
pub use self::nlme::Nlme;

/// Type alias for the NWK layer proxy sender.
pub type ProxySender = Sender<actor::Message>;

/// Type alias for the NWK layer actor receiver.
pub type ActorReceiver = Receiver<actor::Message>;

mod actor;
mod device_proxy;
mod endpoint_proxy;
mod error;
mod nlme;
