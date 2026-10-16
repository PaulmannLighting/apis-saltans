#[cfg(feature = "alloc")]
pub use bytes::Bytes as Payload;

#[cfg(not(feature = "alloc"))]
pub use self::bounded::Payload;

#[cfg(not(feature = "alloc"))]
mod bounded;
