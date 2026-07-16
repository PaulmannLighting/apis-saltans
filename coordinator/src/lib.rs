//! Zigbee transceiver API.
//!
//! This library provides a fully abstracted interface to expose an interface to communicate with
//! a Zigbee transceiver regardless of the underlying hardware.
//!
//! The application supplies a `tokio::sync::mpsc::Sender<Event>` at startup to receive coordinator
//! [`Event`] values. Discovery, binding, address resolution, and persistence are application-owned
//! workflows built from traits such as [`Node`], [`Endpoints`], [`Binding`],
//! [`AddressTranslation`], [`Zcl`], and [`Zdp`].

use const_env::env_item;

pub use self::api::{
    AddressTranslation, Attributes, Binding, Clusters, ColorControl, Endpoints, FoundNetwork,
    Groups, Joining, Level, LocalNode, Node, OnOff, ReadAttributeResult, Routing, ScannedChannel,
    Scanning, SimpleDescriptor, WriteAttributeResult, Zcl, Zdp,
};
pub use self::coordinator::Coordinator;
pub use self::error::Error;
pub use self::event::{Device, Event, Network, NetworkError};

mod api;
mod coordinator;
mod error;
mod event;
mod index;
mod mux;
mod zcl;
mod zdp;

/// The delay between retries, in seconds.
#[env_item("ZIGBEE_COORDINATOR_MPSC_CHANNEL_SIZE")]
const MPSC_CHANNEL_SIZE: usize = 128;
