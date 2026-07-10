use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use zb_core::types::{Type, Uint8};

/// Mechanism used for compensating color or color intensity drift over time.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive)]
#[repr(u8)]
pub enum DriftCompensation {
    /// No drift compensation.
    None = 0x00,
    /// Other or unknown drift compensation.
    Other = 0x01,
    /// Temperature monitoring.
    Temperature = 0x02,
    /// Optical luminance monitoring and feedback.
    OpticalLuminance = 0x03,
    /// Optical color monitoring and feedback.
    OpticalColor = 0x04,
}

impl TryFrom<u8> for DriftCompensation {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}

impl From<DriftCompensation> for Type {
    fn from(value: DriftCompensation) -> Self {
        Self::Enum8(Uint8::new(value as u8))
    }
}

impl TryFrom<Uint8> for DriftCompensation {
    type Error = Uint8;

    fn try_from(value: Uint8) -> Result<Self, Self::Error> {
        Self::try_from(value.into_inner()).map_err(|_| value)
    }
}

impl TryFrom<Type> for DriftCompensation {
    type Error = Type;

    fn try_from(value: Type) -> Result<Self, Self::Error> {
        if let Type::Enum8(value) = value {
            Self::try_from(value).map_err(Type::Enum8)
        } else {
            Err(value)
        }
    }
}
