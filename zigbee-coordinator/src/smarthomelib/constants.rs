use std::time::Duration;

use zigbee::types::Uint16;

/// Maximum duration that can be represented by a `Uint16`.
pub const MAX_UINT16_DURATION: Duration = Duration::from_millis(Uint16::MAX.as_u16() as u64 * 100);
