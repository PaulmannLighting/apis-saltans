use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Ballast information attribute for the `Ballast Configuration` cluster.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, FromPrimitive)]
#[repr(u16)]
pub enum BallastInformationAttribute {
    /// Physical minimum level of the ballast.
    PhysicalMinLevel = 0x0000,
    /// Physical maximum level of the ballast.
    PhysicalMaxLevel = 0x0001,
    /// Status of the ballast.
    BallastStatus = 0x0002,
}

impl TryFrom<u16> for BallastInformationAttribute {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_u16(value).ok_or(value)
    }
}
