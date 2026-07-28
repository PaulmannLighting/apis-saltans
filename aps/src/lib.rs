//! APS frame and data-service definitions and utilities.
//!
//! This crate models Zigbee APS data, command, and acknowledgement frames. It
//! also models the [`apsde`] service primitives and provides [`Assembler`] for
//! rebuilding fragmented APS data frames from network-layer envelopes.
//!
//! APS headers preserve endpoint bytes from incoming frames and expose fallible
//! endpoint getters. This lets callers distinguish valid
//! [`Endpoint`](zb_core::Endpoint) values from reserved endpoint IDs
//! without losing the original protocol value.

pub use self::apsde::TxOptions;
pub use self::broadcast::Broadcast;
pub use self::frame::acknowledgement::Frame as Acknowledgement;
pub use self::frame::command::Frame as Command;
pub use self::frame::data::{self, Assembler, Frame as Data, Unicast};
pub use self::frame::{
    AckFmt, Control, DeliveryMode, Destination, Extended, ExtendedControl, Fragmentation, FrameType,
};

pub mod apsde;
mod broadcast;
mod frame;
