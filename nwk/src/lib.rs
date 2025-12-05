//! Zigbee Network (NWK) Layer implementation.

pub use actor::Actor;
use tokio::sync::mpsc::{Receiver, Sender};

pub use self::error::Error;
pub use self::nlme::Nlme;
pub use self::proxy::Proxy;

/// Type alias for the NWK layer proxy sender.
pub type ProxySender = Sender<actor::Message>;

/// Type alias for the NWK layer actor receiver.
pub type ActorReceiver = Receiver<actor::Message>;

mod actor;
mod device_proxy;
mod endpoint_proxy;
mod error;
mod message;
mod nlme;
mod proxy;
mod zcl_proxy;
