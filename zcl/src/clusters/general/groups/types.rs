use zigbee::types::Uint16;

/// A list of group IDs.
pub type GroupList = heapless::Vec<Uint16, { (u8::MAX - 1) as usize }>;
