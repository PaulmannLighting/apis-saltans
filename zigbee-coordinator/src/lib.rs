//! Zigbee transmitter API.
//!
//! This library provides a fully abstracted interface to expose an interface to communicate with
//! a Zigbee transmitter regardless of the underlying hardware.
//!
//! TODO: This shall replace `zigbee-nwk`.

pub use self::api::{ColorControl, OnOff};
pub use self::coordinator::Coordinator;

mod api;
mod binding;
mod coordinator;
mod discovery;
mod mux;
mod network_manager;
mod transmitter;
