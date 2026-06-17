//! Zigbee address types.

use core::fmt::{self, Display};

use macaddr::MacAddr8;

/// Zigbee device addressing modes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Address {
    ieee_address: MacAddr8,
    short_id: u16,
}

impl Address {
    /// Create a new address.
    #[must_use]
    pub const fn new(ieee_address: MacAddr8, short_id: u16) -> Self {
        Self {
            ieee_address,
            short_id,
        }
    }

    /// Return the IEEE address.
    #[must_use]
    pub const fn ieee_address(&self) -> MacAddr8 {
        self.ieee_address
    }

    /// Return the short ID.
    #[must_use]
    pub const fn short_id(&self) -> u16 {
        self.short_id
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({:#06X})", self.ieee_address, self.short_id)
    }
}
