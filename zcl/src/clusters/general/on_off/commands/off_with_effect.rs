pub use effect::{DelayedAllOff, DyingLight, Effect};
use le_stream::derive::{FromLeStream, ToLeStream};
use zigbee::types::Uint8;
use zigbee::{Cluster, Command};

use crate::general::on_off::CLUSTER_ID;

mod effect;

/// Switch a device off.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromLeStream, ToLeStream)]
pub struct OffWithEffect {
    id: Uint8,
    variant: Uint8,
}

impl OffWithEffect {
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

    /// Return the effect of this command.
    ///
    /// # Errors
    ///
    /// Returns a tuple of `(<id>, <variant>)` if the effect ID and/or variant are invalid.
    pub fn effect(self) -> Result<Effect, (Option<u8>, Option<u8>)> {
        (self.id, self.variant).try_into()
    }
}

impl Cluster for OffWithEffect {
    const ID: u16 = CLUSTER_ID;
}

impl Command for OffWithEffect {
    const ID: u8 = 0x40;
}
