#![cfg(any(feature = "coordinator", feature = "driver-use"))]

//! Common hardware abstraction types shared by drivers and coordinators.

pub use self::datagram::Datagram;
#[cfg(any(feature = "coordinator", feature = "driver"))]
pub use self::datagram::Metadata;
pub use self::error::Error;
pub use self::event::{Event, RouteError};
#[cfg(any(feature = "coordinator", feature = "driver"))]
pub use self::message::{FoundNetwork, Message, Network, ScannedChannel};
pub use self::message::{NcpHandle, WeakNcpHandle};

mod datagram;
mod error;
mod event;
mod message;
