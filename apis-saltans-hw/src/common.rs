//! Common hardware abstraction types shared by drivers and coordinators.

#[cfg(any(feature = "coordinator", feature = "driver"))]
pub use self::datagram::{Datagram, Metadata};
#[cfg(any(feature = "coordinator", feature = "driver"))]
pub use self::error::Error;
#[cfg(any(feature = "coordinator", feature = "driver"))]
pub use self::event::{Event, RouteError};
#[cfg(any(feature = "coordinator", feature = "driver"))]
pub use self::message::{FoundNetwork, Network, ScannedChannel};

/// A handle on the NCP.
#[cfg(any(feature = "coordinator", feature = "driver"))]
pub type NcpHandle = tokio::sync::mpsc::Sender<message::Message>;

#[cfg(any(feature = "coordinator", feature = "driver"))]
mod datagram;
#[cfg(any(feature = "coordinator", feature = "driver"))]
mod error;
#[cfg(any(feature = "coordinator", feature = "driver"))]
mod event;
#[cfg(any(feature = "coordinator", feature = "driver"))]
pub mod message;
