use apis_saltans_core::{ClusterId, Direction};

pub use self::effect_identifier::EffectIdentifier;
pub use self::effect_variant::EffectVariant;
use crate::macros::zcl_command;

mod effect_identifier;
mod effect_variant;

zcl_command! {
    /// Trigger an effect on a device.
    TriggerEffect {
        { ClusterId::Identify } => Identify;
        command_id: 0x40;
        direction: Direction::ClientToServer;
        => super::TriggerEffect;
        derive(Copy);
        fields {
            identifier: u8,
            variant: u8,
        }

        constructor {
            /// Create a new `TriggerEffect` command.
            #[must_use]
            pub fn new(identifier: EffectIdentifier, variant: EffectVariant) -> Self {
                Self {
                    identifier: identifier.into(),
                    variant: variant.into(),
                }
            }
        }

        getters {
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
    }
}
