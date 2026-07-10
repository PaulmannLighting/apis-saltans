use apis_saltans_core::types::Uint8;
use apis_saltans_core::{Cluster, Direction};

pub use self::effect::{DelayedAllOff, DyingLight, Effect};
use crate::macros::zcl_command;

mod effect;

zcl_command! {
    /// Switch a device off.
    OffWithEffect {
        { Cluster::OnOff } => OnOff;
        command_id: 0x40;
        direction: Direction::ClientToServer;
        derive(Copy);
        fields {
            id: Uint8,
            variant: Uint8,
        }

        constructor {
            /// Create a new `OffWithEffect` command with the specified effect ID and effect variant.
            #[must_use]
            pub const fn new(effect: Effect) -> Self {
                Self {
                    id: Uint8::new(effect.discriminant()),
                    variant: Uint8::new(match effect {
                        Effect::DelayedAllOff(delayed_all_off) => delayed_all_off as u8,
                        Effect::DyingLight(dying_light) => dying_light as u8,
                    }),
                }
            }
        }

        getters {
            /// Return the effect of this command.
            ///
            /// # Errors
            ///
            /// Returns a tuple of `(<id>, <variant>)` if the effect ID and/or variant are invalid.
            pub fn effect(self) -> Result<Effect, (Option<u8>, Option<u8>)> {
                (self.id, self.variant).try_into()
            }
        }
    }
}
