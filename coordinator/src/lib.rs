//! Zigbee transceiver API.
//!
//! This library provides a fully abstracted interface to expose an interface to communicate with
//! a Zigbee transceiver regardless of the underlying hardware.
//!
//! Use [`NetworkManager::subscribe`] to receive coordinator [`Event`] values. Unsolicited ZCL
//! commands are represented by [`Event::Zcl`] and expose their resolved source through [`Zdp`].

use const_env::env_item;

pub use self::api::{
    Attributes, Binding, ColorControl, Endpoints, Joining, Level, Node, OnOff, ReadAttributeResult,
    WriteAttributeResult, Zcl, Zdp,
};
pub use self::coordinator::Coordinator;
pub use self::error::Error;
pub use self::event::{Device, Event, Network, NetworkError};
pub use self::timeout::Timeout;

mod api;
mod coordinator;
mod error;
mod event;
mod index;
mod mux;
mod timeout;
mod zcl;
mod zdp;

/// The delay between retries, in seconds.
#[env_item("ZIGBEE_COORDINATOR_MPSC_CHANNEL_SIZE")]
const MPSC_CHANNEL_SIZE: usize = 128;
