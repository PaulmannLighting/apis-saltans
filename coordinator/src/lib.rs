//! Zigbee transceiver API.
//!
//! This library provides a fully abstracted interface to expose an interface to communicate with
//! a Zigbee transceiver regardless of the underlying hardware.
//!
//! The application supplies a `tokio::sync::mpsc::Sender<Event>` at startup to receive coordinator
//! [`Event`] values. Discovery, binding, address resolution, and persistence are application-owned
//! workflows built from traits such as [`Node`], [`Endpoints`], [`Binding`],
//! [`AddressTranslation`], [`Zcl`], and [`Zdp`].
//!
//! ZCL and ZDP sends use deferred response futures. The first await queues an operation and returns
//! either [`zb_hw::HwResponse`] or a protocol-specific [`ZclResponse`] or [`ZdpResponse`]. Await
//! that returned future to observe hardware completion and, for communication requests, receive
//! the converted protocol response.

use const_env::env_item;

pub use self::api::{
    AddressTranslation, Attributes, Binding, Clusters, ColorControl, Endpoints, FoundNetwork,
    Groups, Joining, Level, LocalNode, Node, OnOff, ReadAttributeResult, Routing, ScannedChannel,
    Scanning, SimpleDescriptor, WriteAttributeResult, Zcl, ZclResponse, Zdp, ZdpResponse,
};
pub use self::coordinator::Coordinator;
pub use self::error::{Error, Optional, StatusExt};
pub use self::event::{Device, Event, Network, NetworkError};
pub use self::response::CommunicationResponse;

mod api;
mod coordinator;
mod error;
mod event;
mod index;
mod mux;
mod response;
mod zcl;
mod zdp;

/// The delay between retries, in seconds.
#[env_item("ZIGBEE_COORDINATOR_MPSC_CHANNEL_SIZE")]
const MPSC_CHANNEL_SIZE: usize = 128;
