#![cfg(any(feature = "coordinator", feature = "driver"))]

//! Common hardware abstraction types shared by drivers and coordinators.

pub use self::datagram::{Datagram, Metadata};
pub use self::error::Error;
pub use self::event::{Event, RouteError};
pub use self::message::{FoundNetwork, Message, NcpHandle, Network, ScannedChannel, WeakNcpHandle};

mod datagram;
mod error;
mod event;
mod message;
