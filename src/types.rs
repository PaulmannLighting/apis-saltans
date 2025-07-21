//! Common types used across the protocol.

use le_stream::{ByteSizedVec, WordSizedVec};
pub use u24::U24;

mod u24;
mod u40;
mod u48;

/// A string type, which can be up to 16 bytes long.
pub type String16 = heapless::String<16>;

/// A string type, which can be up to 32 bytes long.
pub type String32 = heapless::String<32>;

/// An octet string, with a capacity of [`u8::MAX`].
pub type OctStr = ByteSizedVec<u8>;

/// An octet string, with a capacity of [`u16::MAX`].
pub type OctStr16 = WordSizedVec<u8>;

/// A string type, which can be up to [`u8::MAX`] bytes long.
pub type ByteSizedStr = heapless::String<{ u8::MAX as usize }>;

/// A string type, which can be up to [`u16::MAX`] bytes long.
pub type WordSizedStr = heapless::String<{ u16::MAX as usize }>;
