use num_enum::{IntoPrimitive, TryFromPrimitive};

/// Ballast information attribute for the `Ballast Configuration` cluster.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, IntoPrimitive, PartialEq, TryFromPrimitive)]
#[num_enum(error_type(name = u16, constructor = core::convert::identity))]
#[repr(u16)]
pub enum BallastInformationAttribute {
    /// Physical minimum level of the ballast.
    PhysicalMinLevel = 0x0000,
    /// Physical maximum level of the ballast.
    PhysicalMaxLevel = 0x0001,
    /// Status of the ballast.
    BallastStatus = 0x0002,
}
