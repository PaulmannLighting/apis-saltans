//! Zigbee hardware abstraction API.
//!
//! This crate defines the boundary between coordinator-level logic and concrete Zigbee network
//! co-processor (NCP) drivers. The `driver` feature exposes implementor-facing traits for hardware
//! backends, while the `coordinator` feature exposes caller-facing handle extensions for
//! coordinator code. Shared data, events, errors, and the internal actor message protocol are always
//! compiled.

#[cfg(any(feature = "coordinator", feature = "driver", feature = "driver-use"))]
pub use self::common::{
    Datagram, Error, Event, FoundNetwork, Metadata, NcpHandle, Network, RouteError, ScannedChannel,
};
#[cfg(feature = "coordinator")]
pub use self::coordinator::*;
#[cfg(feature = "driver")]
pub use self::driver::{Backend, Driver, EventTranslator, Initialize, bridge};
#[cfg(feature = "driver-use")]
pub use self::driver_use::{Builder, StartedHardware};

mod common;
mod coordinator;
mod driver;
mod driver_use;
