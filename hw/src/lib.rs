//! Zigbee hardware abstraction API.
//!
//! This crate defines the boundary between coordinator-level logic and concrete Zigbee network
//! co-processor (NCP) drivers. The `driver-use` feature exposes the types needed to construct and
//! run an existing backend. The `driver` feature includes `driver-use` and adds implementor-facing
//! traits for hardware backends. The `coordinator` feature exposes caller-facing handle extensions
//! for coordinator code. Shared data, events, and errors are exported when at least one public API
//! feature is enabled.

#[cfg(any(feature = "coordinator", feature = "driver"))]
pub use self::common::{Datagram, Event, FoundNetwork, Metadata, Network, ScannedChannel};
#[cfg(any(feature = "coordinator", feature = "driver-use"))]
pub use self::common::{Error, NcpHandle, RouteError, WeakNcpHandle};
#[cfg(feature = "coordinator")]
pub use self::coordinator::*;
#[cfg(feature = "driver")]
pub use self::driver::{Backend, Driver, EventTranslator, Initialize, bridge};
#[cfg(feature = "driver-use")]
pub use self::driver_use::{Builder, StartedHardware};
#[cfg(feature = "driver")]
pub use self::reexports::{aps, core, nwk, zdp};

mod common;
mod coordinator;
mod driver;
mod driver_use;
#[cfg(feature = "driver")]
pub mod reexports;
