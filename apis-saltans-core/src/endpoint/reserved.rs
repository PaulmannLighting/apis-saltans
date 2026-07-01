use core::fmt;
use core::fmt::{Display, LowerHex, UpperHex};

/// A Zigbee reserved endpoint ID.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd, PartialEq)]
#[repr(transparent)]
pub struct Reserved(pub(crate) u8);

impl Reserved {
    /// The minimum valid reserved endpoint ID.
    pub const MIN: u8 = 241;

    /// The maximum valid reserved endpoint ID.
    pub const MAX: u8 = 254;
}

impl Display for Reserved {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl LowerHex for Reserved {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        LowerHex::fmt(&self.0, f)
    }
}

impl UpperHex for Reserved {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        UpperHex::fmt(&self.0, f)
    }
}

impl From<Reserved> for u8 {
    fn from(endpoint: Reserved) -> Self {
        endpoint.0
    }
}
