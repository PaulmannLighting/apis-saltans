//! Common hardware abstraction types shared by drivers and coordinators.

#[cfg(any(feature = "coordinator", feature = "driver", feature = "driver-use"))]
pub use self::datagram::{Datagram, Metadata};
#[cfg(any(feature = "coordinator", feature = "driver", feature = "driver-use"))]
pub use self::error::Error;
#[cfg(any(feature = "coordinator", feature = "driver", feature = "driver-use"))]
pub use self::event::{Event, RouteError};
#[cfg(any(feature = "coordinator", feature = "driver", feature = "driver-use"))]
pub use self::message::{FoundNetwork, Network, ScannedChannel};

/// A handle on the NCP.
#[cfg(any(feature = "coordinator", feature = "driver", feature = "driver-use"))]
pub type NcpHandle = tokio::sync::mpsc::Sender<message::Message>;

#[cfg(any(feature = "coordinator", feature = "driver", feature = "driver-use"))]
mod datagram;
#[cfg(any(feature = "coordinator", feature = "driver", feature = "driver-use"))]
mod error;
#[cfg(any(feature = "coordinator", feature = "driver", feature = "driver-use"))]
mod event;
#[cfg(any(feature = "coordinator", feature = "driver", feature = "driver-use"))]
pub mod message;
