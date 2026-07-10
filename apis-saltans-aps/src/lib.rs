//! APS frame definitions and utilities.
//!
//! This crate models Zigbee APS data, command, and acknowledgement frames. It
//! also provides [`Assembler`] for rebuilding fragmented APS data frames from
//! network-layer envelopes.
//!
//! APS headers preserve endpoint bytes from incoming frames and expose fallible
//! endpoint getters. This lets callers distinguish valid
//! [`Endpoint`](apis_saltans_core::Endpoint) values from reserved endpoint IDs
//! without losing the original protocol value.

pub use self::broadcast::Broadcast;
pub use self::frame::acknowledgement::Frame as Acknowledgement;
pub use self::frame::command::Frame as Command;
pub use self::frame::data::{self, Assembler, Frame as Data, Unicast};
pub use self::frame::{
    AckFmt, Control, DeliveryMode, Destination, Extended, ExtendedControl, Fragmentation, FrameType,
};

mod broadcast;
mod frame;
