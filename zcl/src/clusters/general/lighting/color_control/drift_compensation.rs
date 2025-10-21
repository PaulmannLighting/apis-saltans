use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

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
