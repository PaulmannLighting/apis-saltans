pub use delayed_all_off::DelayedAllOff;
pub use dying_light::DyingLight;
use repr_discriminant::ReprDiscriminant;
use zigbee::types::Uint8;

mod delayed_all_off;
mod dying_light;

/// Effects.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
#[derive(ReprDiscriminant)]
pub enum Effect {
    /// Delayed all off effect.
    DelayedAllOff(DelayedAllOff) = 0x00,
    /// Dying light effect.
    DyingLight(DyingLight) = 0x01,
}

impl TryFrom<(Uint8, Uint8)> for Effect {
    type Error = (Option<u8>, Option<u8>);

    fn try_from((id, variant): (Uint8, Uint8)) -> Result<Self, Self::Error> {
        let Ok(id_u8) = u8::try_from(id) else {
            return Err((None, variant.try_into().ok()));
        };

        let Ok(variant) = u8::try_from(variant) else {
            return Err((Some(id_u8), None));
        };

        Self::try_from((id_u8, variant)).map_err(|(id, variant)| (Some(id), Some(variant)))
    }
}

impl TryFrom<(u8, u8)> for Effect {
    type Error = (u8, u8);

    fn try_from((id, variant): (u8, u8)) -> Result<Self, Self::Error> {
        match (id, variant) {
            (0x00, variant) => DelayedAllOff::try_from(variant)
                .map_err(|variant| (id, variant))
                .map(Self::DelayedAllOff),
            (0x01, variant) => DyingLight::try_from(variant)
                .map_err(|variant| (id, variant))
                .map(Self::DyingLight),
            _ => Err((id, variant)),
        }
    }
}
