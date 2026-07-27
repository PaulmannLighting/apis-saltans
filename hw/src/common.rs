#![cfg(feature = "types")]

//! Common hardware abstraction types shared by drivers and coordinators.

#[cfg(feature = "driver")]
pub use self::driver::Driver;
pub use self::error::{Error, Operation, TransmissionError};
pub use self::event::{ApsEvent, DeviceEvent, Event, NetworkEvent, RouteError};
pub use self::message::{
    Channel, ChannelMask, FoundNetwork, NcpHandle, NetworkDescriptor, ScanDuration, ScannedChannel,
    WeakNcpHandle,
};

#[cfg(feature = "driver")]
mod driver;
mod error;
mod event;
pub mod message;
