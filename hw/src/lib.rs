//! Zigbee hardware abstraction API.
//!
//! This crate defines the boundary between coordinator-level logic and concrete Zigbee network
//! co-processor (NCP) drivers.
//!
//! No default features are enabled. Enable exactly the API surface needed by the depending crate:
//!
//! - `coordinator` exposes `Ncp`, `Driver`, `NcpHandle`, `WeakNcpHandle`, common errors, hardware
//!   events, access to the NCP's local simple descriptors, scan results, transmit datagram types,
//!   and the deferred `HwResponse` returned by transmission requests for coordinator and
//!   application code that sends commands to a running NCP actor.
//! - `driver` exposes `Driver`, shared driver/coordinator data types, the required local-endpoint
//!   API, command handles, `HwResponse`, common errors, and the `aps`, `core`, `nwk`, and `zdp`
//!   protocol re-export modules for hardware backend implementations.
//!
//! `Driver` is part of the shared API and is therefore available with either feature. Event
//! translation and startup wiring are backend concerns; this crate does not prescribe backend
//! configuration or provide an event-translator abstraction.
//!
//! The protocol re-export modules are available only with `driver`. They let driver crates refer to
//! APIS Saltans protocol types through this crate, for example `apis_saltans_hw::core::IeeeAddress`
//! or `apis_saltans_hw::zdp::SimpleDescriptor`, without adding direct dependencies on each
//! protocol crate.
//!
//! `Ncp::transmit` is a two-stage operation. Awaiting the method hands the datagram to the driver
//! actor and returns an `HwResponse`; awaiting that response observes completion of the hardware
//! transmission.
//!
//! Every `Driver` implementation must provide the NCP's local application endpoints through
//! `Driver::get_endpoints`. Each endpoint is represented by a complete
//! `zb_zdp::SimpleDescriptor`; coordinator code retrieves the same descriptors through
//! `Ncp::get_endpoints`.

#[cfg(any(feature = "coordinator", feature = "driver"))]
#[cfg_attr(docsrs, doc(cfg(any(feature = "coordinator", feature = "driver"))))]
pub use self::common::{
    Clusters, Datagram, Driver, Error, Event, FoundNetwork, HwResponse, Metadata, NcpHandle,
    Network, RouteError, ScannedChannel, WeakNcpHandle,
};
#[cfg(feature = "coordinator")]
#[cfg_attr(docsrs, doc(cfg(feature = "coordinator")))]
pub use self::coordinator::*;
#[cfg(feature = "driver")]
#[cfg_attr(docsrs, doc(cfg(feature = "driver")))]
pub use self::reexports::{aps, core, nwk, zdp};

mod common;
mod coordinator;
mod reexports;
