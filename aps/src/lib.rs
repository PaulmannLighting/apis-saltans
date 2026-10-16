//! APS frame and data-service definitions and utilities.
//!
//! This crate is `no_std`. Its default storage is bounded and allocation-free.
//! Enable `alloc` to retain `bytes::Bytes` payloads and dynamically allocated reassembly.
//! [`Payload`] selects the owned payload representation. Without `alloc`, the
//! `ZIGBEE_APS_*_CAPACITY` build-time environment variables configure storage limits.
//!
//! This crate models Zigbee APS data, command, and acknowledgement frames. It
//! also models the [`apsde`] service primitives and provides [`Assembler`] for
//! rebuilding fragmented APS data frames from network-layer envelopes.
//!
//! APS headers preserve endpoint bytes from incoming frames and expose fallible
//! endpoint getters. This lets callers distinguish valid
//! [`Endpoint`](zb_core::Endpoint) values from reserved endpoint IDs
//! without losing the original protocol value.

#![no_std]

#[cfg(any(feature = "alloc", test))]
extern crate alloc;

#[cfg(feature = "alloc")]
use {const_env as _, heapless as _};

pub use self::apsde::TxOptions;
pub use self::broadcast::Broadcast;
pub use self::capacity::{CapacityError, CapacityKind};
#[cfg(not(feature = "alloc"))]
pub use self::capacity::{
    FRAGMENT_CAPACITY, FRAGMENTS_CAPACITY, PAYLOAD_CAPACITY, TRANSACTIONS_CAPACITY,
};
pub use self::frame::acknowledgement::Frame as Acknowledgement;
pub use self::frame::command::Frame as Command;
pub use self::frame::data::{
    self, Assembler, AssemblyError, FragmentationError, Frame as Data, Unicast,
};
pub use self::frame::{
    AckFmt, Control, DeliveryMode, Destination, Extended, ExtendedControl, Fragmentation,
    FrameType, WeakDestination,
};
pub use self::payload::Payload;

pub mod apsde;
mod broadcast;
mod capacity;
mod frame;
mod payload;
