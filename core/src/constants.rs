//! Common constants.

use crate::types::Uint8;

/// Number of deciseconds per millisecond.
pub const DECI_SECONDS_PER_MILLISECOND: u64 = 100;

/// Maximum size of a byte-sized array.
pub const U8_CAPACITY: usize = Uint8::MAX.into_inner() as usize;
