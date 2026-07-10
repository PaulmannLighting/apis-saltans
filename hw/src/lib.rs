//! Zigbee hardware abstraction API.
//!
//! This crate defines the boundary between coordinator-level logic and concrete Zigbee network
//! co-processor (NCP) drivers.
//!
//! No default features are enabled. Enable exactly the API surface needed by the depending crate:
//!
//! - `coordinator` exposes `Ncp`, `NcpHandle`, `WeakNcpHandle`, common errors, hardware
//!   events, scan results, and transmit datagram types for coordinator and application code that
//!   sends commands to a running NCP actor.
//! - `driver-use` exposes `Builder`, `StartedHardware`, `NcpHandle`, `WeakNcpHandle`, `Error`, and
//!   `RouteError` for integration code that starts an existing backend.
//! - `driver` includes `driver-use` and adds `Backend`, `Driver`, `EventTranslator`, `bridge`,
//!   driver-side data types, and the `aps`, `core`, `nwk`, and `zdp` protocol re-export modules for
//!   hardware backend implementations.
//!
//! The protocol re-export modules are available only with `driver`. They let driver crates refer to
//! APIS Saltans protocol types through this crate, for example `apis_saltans_hw::core::IeeeAddress`
//! or `apis_saltans_hw::zdp::SimpleDescriptor`, without adding direct dependencies on each
//! protocol crate.

#[cfg(any(feature = "coordinator", feature = "driver"))]
pub use self::common::{Datagram, Event, FoundNetwork, Metadata, Network, ScannedChannel};
#[cfg(any(feature = "coordinator", feature = "driver-use"))]
pub use self::common::{Error, NcpHandle, RouteError, WeakNcpHandle};
#[cfg(feature = "coordinator")]
pub use self::coordinator::*;
#[cfg(feature = "driver")]
pub use self::driver::{Backend, Driver, EventTranslator, bridge};
#[cfg(feature = "driver-use")]
pub use self::driver_use::{Builder, Futures};
#[cfg(feature = "driver")]
pub use self::reexports::{aps, core, nwk, zdp};

mod common;
mod coordinator;
mod driver;
mod driver_use;
#[cfg(feature = "driver")]
pub mod reexports;
