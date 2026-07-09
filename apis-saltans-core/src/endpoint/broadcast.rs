use core::fmt::{self, Display, LowerHex, UpperHex};

use crate::{Application, Endpoint};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Broadcast {
    Application(Application),
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
