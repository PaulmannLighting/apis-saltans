pub use effect_identifier::EffectIdentifier;
pub use effect_variant::EffectVariant;

use crate::zcl::identify::CLUSTER_ID;
use crate::zcl::{Cluster, Command};

mod effect_identifier;
mod effect_variant;

/// Trigger an effect on a device.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TriggerEffect {
    identifier: u8,
    variant: u8,
}

impl TriggerEffect {
    #[must_use]
    pub fn new(identifier: EffectIdentifier, variant: EffectVariant) -> Self {
        Self {
            identifier: identifier.into(),
            variant: variant.into(),
        }
    }

    pub fn identifier(self) -> Result<EffectIdentifier, u8> {
        EffectIdentifier::try_from(self.identifier)
    }

    pub fn variant(self) -> Result<EffectVariant, u8> {
        EffectVariant::try_from(self.variant)
    }
}

impl Cluster for TriggerEffect {
    const ID: u16 = CLUSTER_ID;
}

impl Command for TriggerEffect {
    const ID: u8 = 0x40;
}
