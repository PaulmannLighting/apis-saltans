use crate::{Application, Endpoint};

pub const BROADCAST: u8 = 0xff;

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

impl Broadcast {
    /// Return the raw endpoint selector.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::Application(application) => application.as_u8(),
            Self::Broadcast => BROADCAST,
        }
    }
}

impl_display_and_hex_via_value!(Broadcast, u8, |value| value.as_u8(), |value, formatter| {
    <Endpoint as core::fmt::Display>::fmt(&Endpoint::from(*value), formatter)
});

impl From<Broadcast> for Endpoint {
    fn from(broadcast: Broadcast) -> Self {
        match broadcast {
            Broadcast::Application(application) => Self::Application(application),
            Broadcast::Broadcast => Self::Broadcast,
        }
    }
}

impl From<Broadcast> for u8 {
    fn from(broadcast: Broadcast) -> Self {
        broadcast.as_u8()
    }
}
