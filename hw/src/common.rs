#![cfg(any(feature = "coordinator", feature = "driver"))]

//! Common hardware abstraction types shared by drivers and coordinators.

pub use self::clusters::Clusters;
pub use self::datagram::{Datagram, Metadata};
pub use self::error::Error;
pub use self::event::{Event, RouteError};
pub use self::hw_response::HwResponse;
pub use self::message::{FoundNetwork, Message, NcpHandle, Network, ScannedChannel, WeakNcpHandle};

mod clusters;
mod datagram;
mod error;
mod event;
mod hw_response;
mod message;
