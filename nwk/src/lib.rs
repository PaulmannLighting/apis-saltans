//! Zigbee Network (NWK) Layer implementation.

pub use actor::{Actor, Proxy};
pub use device_proxy::DeviceProxyExt;
use tokio::sync::mpsc::{Receiver, Sender};
pub use {aps, zcl, zigbee};

pub use self::error::Error;
pub use self::nlme::Nlme;

/// Type alias for the NWK layer proxy sender.
pub type ProxySender<T> = Sender<actor::Message<T>>;

/// Type alias for the NWK layer actor receiver.
pub type ActorReceiver<T> = Receiver<actor::Message<T>>;

mod actor;
mod device_proxy;
mod endpoint_proxy;
mod error;
mod nlme;
