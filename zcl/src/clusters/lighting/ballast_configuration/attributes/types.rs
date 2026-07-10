use apis_saltans_core::types::Type;
use bitflags::bitflags;

/// Lamp alarm mode attribute.
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(transparent)
)]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LampAlarmMode(u8);

bitflags! {
    impl LampAlarmMode: u8 {
        /// Alarm when the lamp burn hours trip point is reached.
        const LAMP_BURN_HOURS = 0b0000_0001;
    }
}

impl From<LampAlarmMode> for Type {
    fn from(value: LampAlarmMode) -> Self {
        Self::Map8(value.bits())
    }
}

impl TryFrom<Type> for LampAlarmMode {
    type Error = Type;

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        if let Type::Map8(value) = value {
            Ok(Self::from_bits_retain(value))
        } else {
            Err(value)
        }
    }
}
