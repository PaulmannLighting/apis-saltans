//! Time cluster implementation.

pub use self::attribute::{TimeStatus, readable, writable};

mod attribute;

const CLUSTER_ID: u16 = 0x000A;
