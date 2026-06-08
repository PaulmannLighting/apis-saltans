//! Zigbee transceiver API.
//!
//! This library provides a fully abstracted interface to expose an interface to communicate with
//! a Zigbee transceiver regardless of the underlying hardware.
//!
//! TODO: This shall replace `zigbee-nwk`.

pub use self::api::{ColorControl, OnOff, ReadAttributes};
pub use self::coordinator::Coordinator;
pub use self::error::Error;

mod api;
mod binding;
mod coordinator;
mod discovery;
mod error;
mod expect;
mod mux;
mod network_manager;
mod timeout;
mod transceiver;
