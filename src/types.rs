//! Common types used across the protocol.

pub use u24::U24;

mod u24;

/// A string type, which can be up to 16 bytes long.
pub type String16 = heapless::String<16>;
/// A string type, which can be up to 32 bytes long.
pub type String32 = heapless::String<32>;
