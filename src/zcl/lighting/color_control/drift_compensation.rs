/// Mechanism used for compensating color or color intensity drift over time.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
        match value {
            0x00 => Ok(Self::None),
            0x01 => Ok(Self::Other),
            0x02 => Ok(Self::Temperature),
            0x03 => Ok(Self::OpticalLuminance),
            0x04 => Ok(Self::OpticalColor),
            other => Err(other),
        }
    }
}
