use apis_saltans_core::types::Uint8;
use repr_discriminant::ReprDiscriminant;

pub use self::delayed_all_off::DelayedAllOff;
pub use self::dying_light::DyingLight;

mod delayed_all_off;
mod dying_light;

/// Effects.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    expect(clippy::unsafe_derive_deserialize)
)]
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
                .map(Self::DelayedAllOff)
                .map_err(|variant| (id, variant)),
            (0x01, variant) => DyingLight::try_from(variant)
                .map(Self::DyingLight)
                .map_err(|variant| (id, variant)),
            _ => Err((id, variant)),
        }
    }
}

#[cfg(feature = "smarthomelib")]
impl From<smarthomelib::command::Effect> for Effect {
    fn from(effect: smarthomelib::command::Effect) -> Self {
        match effect {
            smarthomelib::command::Effect::DelayedAllOff(delayed_all_off) => {
                Self::DelayedAllOff(match delayed_all_off {
                    smarthomelib::command::DelayedAllOff::Fade => DelayedAllOff::FadeToOff,
                    smarthomelib::command::DelayedAllOff::Instant => DelayedAllOff::NoFade,
                    smarthomelib::command::DelayedAllOff::DimThenFade => DelayedAllOff::DimDown,
                })
            }
            smarthomelib::command::Effect::DyingLight => Self::DyingLight(DyingLight::default()),
        }
    }
}
