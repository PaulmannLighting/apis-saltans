use zigbee::types::{Uint8, Uint16};

/// A list of group IDs.
pub type GroupList = heapless::Vec<Uint16, { Uint8::MAX.as_usize() }>;
