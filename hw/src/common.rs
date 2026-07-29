#![cfg(feature = "types")]

//! Common hardware abstraction types shared by drivers and coordinators.

#[cfg(feature = "driver")]
pub use self::driver::Driver;
pub use self::error::{Error, Operation, TransmissionError};
pub use self::event::{ApsdeEvent, DeviceEvent, Event, NetworkEvent, RouteError};
pub use self::message::{
    Channel, ChannelMask, FoundNetwork, NetworkDescriptor, ScanDuration, ScannedChannel,
};
pub use self::ncp_handle::{NcpHandle, WeakNcpHandle};

#[cfg(feature = "driver")]
mod driver;
mod error;
mod event;
pub mod message;
mod ncp_handle;
