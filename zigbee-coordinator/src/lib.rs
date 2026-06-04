//! Zigbee transmitter API.
//!
//! This library provides a fully abstracted interface to expose an interface to communicate with
//! a Zigbee transmitter regardless of the underlying hardware.
//!
//! TODO: This shall replace `zigbee-nwk`.

pub use coordinator::Coordinator;

mod binding;
mod coordinator;
mod discovery;
mod mux;
mod network_manager;
mod transmitter;
