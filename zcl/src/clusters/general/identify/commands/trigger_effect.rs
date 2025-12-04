use le_stream::ToLeStream;
use zigbee::Direction;

pub use self::effect_identifier::EffectIdentifier;
pub use self::effect_variant::EffectVariant;
use crate::clusters::general::identify::CLUSTER_ID;
use crate::{Cluster, Command};

mod effect_identifier;
mod effect_variant;

/// Trigger an effect on a device.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, ToLeStream)]
pub struct TriggerEffect {
    identifier: u8,
    variant: u8,
}

impl TriggerEffect {
    /// Create a new `TriggerEffect` command.
    #[must_use]
    pub fn new(identifier: EffectIdentifier, variant: EffectVariant) -> Self {
        Self {
            identifier: identifier.into(),
            variant: variant.into(),
        }
    }

    /// Return the effect identifier.
    ///
    /// # Errors
    ///
    /// Returns the raw identifier if it cannot be converted to an `EffectIdentifier`.
    pub fn identifier(self) -> Result<EffectIdentifier, u8> {
        EffectIdentifier::try_from(self.identifier)
    }

    /// Return the effect variant.
    ///
    /// # Errors
    ///
    /// Returns the raw variant if it cannot be converted to an `EffectVariant`.
    pub fn variant(self) -> Result<EffectVariant, u8> {
        EffectVariant::try_from(self.variant)
    }
}

impl Cluster for TriggerEffect {
    const ID: u16 = CLUSTER_ID;
}

impl Command for TriggerEffect {
    const ID: u8 = 0x40;
    const DIRECTION: Direction = Direction::ClientToServer;
}
