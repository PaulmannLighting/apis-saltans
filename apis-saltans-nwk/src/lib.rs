//! Network-layer support types for Zigbee.
//!
//! This crate provides small, transport-neutral value types used to describe
//! Zigbee network-layer sources, per-frame metadata, and envelopes that attach
//! that information to an arbitrary payload.
//!
//! The crate is `no_std` and can optionally derive `serde` and `le-stream`
//! serialization implementations through the `serde` and `le-stream` features.

#![no_std]

pub use self::rx::{Envelope, Metadata, Source};
pub use self::tx::Destination;

mod rx;
mod tx;
