use num_derive::FromPrimitive;

/// Ballast information attributes for the `Ballast Configuration` cluster.
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
