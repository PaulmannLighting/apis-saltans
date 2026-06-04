//! Zigbee transmitter API.
//!
//! This library provides a fully abstracted interface to expose an interface to communicate with
//! a Zigbee transmitter regardless of the underlying hardware.
//!
//! TODO: This shall replace `zigbee-nwk`.

pub use transmitter::Transmitter;

mod binding;
mod discovery;
mod mux;
mod network_manager;
mod transmitter;
mod zcl_message;
