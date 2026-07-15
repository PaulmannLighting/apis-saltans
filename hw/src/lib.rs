//! Zigbee hardware abstraction API.
//!
//! This crate defines the boundary between coordinator-level logic and concrete Zigbee network
//! co-processor (NCP) drivers.
//!
//! No default features are enabled. Enable exactly the API surface needed by the depending crate:
//!
//! - `coordinator` exposes `Ncp`, `NcpHandle`, `WeakNcpHandle`, common errors, hardware
//!   events, local endpoint cluster summaries, scan results, and transmit datagram types for
//!   coordinator and application code that sends commands to a running NCP actor.
//! - `driver` exposes `Backend`, `Driver`, `EventTranslator`, `bridge`, shared driver/coordinator
//!   data types, local endpoint cluster summaries, command handles, common errors, and the `aps`,
//!   `core`, `nwk`, and `zdp` protocol re-export modules for hardware backend implementations.
//!
//! The protocol re-export modules are available only with `driver`. They let driver crates refer to
//! APIS Saltans protocol types through this crate, for example `apis_saltans_hw::core::IeeeAddress`
//! or `apis_saltans_hw::zdp::SimpleDescriptor`, without adding direct dependencies on each
//! protocol crate.

#[cfg(any(feature = "coordinator", feature = "driver"))]
pub use self::common::{
    Clusters, Datagram, Error, Event, FoundNetwork, Metadata, NcpHandle, Network, RouteError,
    ScannedChannel, WeakNcpHandle,
};
#[cfg(feature = "coordinator")]
pub use self::coordinator::*;
#[cfg(feature = "driver")]
pub use self::driver::{Backend, Driver, EventTranslator, bridge};
#[cfg(feature = "driver")]
pub use self::reexports::{aps, core, nwk, zdp};

mod common;
mod coordinator;
mod driver;
#[cfg(feature = "driver")]
pub mod reexports;
