//! Zigbee hardware abstraction API.
//!
//! This crate defines the boundary between coordinator-level logic and concrete Zigbee network
//! co-processor (NCP) drivers.
//!
//! No default features are enabled. Enable the API surface needed by the depending crate:
//!
//! - `types` exposes shared handles, errors, events, and scan parameters and results.
//! - `coordinator` adds the caller-facing inherent methods on `NcpHandle` and enables `types`.
//! - `driver` adds the implementor-facing `Driver` trait and protocol re-export modules, and enables
//!   `types`.
//!
//! Event translation and startup wiring are backend concerns; this crate does not prescribe
//! backend configuration or provide an event-translator abstraction.
//!
//! The protocol re-export modules are available only with `driver`. They let driver crates refer to
//! `apis-saltans` protocol types through this crate, for example
//! `apis_saltans_hw::core::IeeeAddress` or `apis_saltans_hw::zdp::SimpleDescriptor`, without adding
//! direct dependencies on each protocol crate.
//!
//! `NcpHandle::transmit` returns after the hardware backend accepts an APS data request. Hardware
//! backends report incoming ASDUs and acknowledged transmission completion asynchronously through
//! [`Event::Apsde`] using [`ApsdeEvent::DataIndication`] and [`ApsdeEvent::DataConfirm`]. Incoming
//! indications include whether the application is responsible for answering a ZDO request.
//!
//! Every `Driver` implementation must provide the NCP's local application endpoints through
//! `Driver::get_endpoints`. Each endpoint is represented by a complete
//! `zb_zdp::SimpleDescriptor`; coordinator code retrieves the same descriptors through
//! `NcpHandle::get_endpoints`.
#![cfg_attr(docsrs, feature(doc_cfg))]

#[cfg(feature = "types")]
#[cfg_attr(docsrs, doc(cfg(feature = "types")))]
pub use zb_aps::TxOptions;

#[cfg(feature = "driver")]
#[cfg_attr(docsrs, doc(cfg(feature = "driver")))]
pub use self::common::Driver;
#[cfg(feature = "types")]
#[cfg_attr(docsrs, doc(cfg(feature = "types")))]
pub use self::common::{
    ApsdeEvent, Channel, ChannelMask, DeviceEvent, Error, Event, FoundNetwork, NcpHandle,
    NetworkDescriptor, NetworkEvent, Operation, RouteError, ScanDuration, ScannedChannel,
    TransmissionError, WeakNcpHandle,
};
#[cfg(feature = "driver")]
#[cfg_attr(docsrs, doc(cfg(feature = "driver")))]
pub use self::reexports::{aps, core, zdp};

mod common;
mod reexports;
