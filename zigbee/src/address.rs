//! Zigbee address types.

use macaddr::MacAddr8;

/// Zigbee device addressing modes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Address {
    /// IEEE address.
    IeeeAddress(MacAddr8),

    /// Short address.
    ShortAddress(u16),
}
