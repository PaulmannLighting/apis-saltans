use core::fmt::{self, Display, LowerHex, UpperHex};

use crate::{Application, Endpoint};

/// Endpoint selector for outgoing broadcast transmissions.
///
/// Zigbee broadcast delivery can target either a normal application endpoint on
/// every receiving node or the endpoint-wide broadcast selector (`0xff`). This
/// type excludes the ZDO data endpoint and reserved endpoint range, so callers
/// can express only endpoint values that are meaningful for application-level
/// broadcast delivery.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Broadcast {
    /// Broadcast to the given application endpoint on each receiving node.
    Application(Application),

    /// Broadcast to the Zigbee endpoint broadcast selector (`0xff`).
    #[default]
    Broadcast,
}

impl Display for Broadcast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Endpoint::from(*self).fmt(f)
    }
}

impl From<Broadcast> for Endpoint {
    fn from(broadcast: Broadcast) -> Self {
        match broadcast {
            Broadcast::Application(application) => Self::Application(application),
            Broadcast::Broadcast => Self::Broadcast,
        }
    }
}

impl LowerHex for Broadcast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Endpoint::from(*self).fmt(f)
    }
}

impl UpperHex for Broadcast {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Endpoint::from(*self).fmt(f)
    }
}
