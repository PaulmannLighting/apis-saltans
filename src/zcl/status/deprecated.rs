use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use crate::zcl::status::Status;

/// Deprecated ZCL status codes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Deprecated {
    /// Indicates the general command is not supported.
    ///
    /// Use `UnsupportedCommand` instead.
    UnsupportedGeneralCommand = 0x82,
    /// Indicates the manufacturer-specific cluster command is not supported.
    ///
    /// Use `UnsupportedCommand` instead.
    UnsupportedManufacturerClusterCommand = 0x83,
    /// Indicates the manufacturer-specific general command is not supported.
    ///
    /// Use `UnsupportedCommand` instead.
    UnsupportedManufacturerGeneralCommand = 0x84,
    /// Indicates a duplicate entry exists.
    ///
    /// Use `Success` instead.
    DuplicateExists = 0x8a,
    /// Indicates the attribute is write-only.
    ///
    /// Use `NotAuthorized` instead.
    WriteOnly = 0x8f,
    /// Indicates the startup state is inconsistent.
    ///
    /// Use `Failure` instead.
    InconsistentStartupState = 0x90,
    /// Indicates the command was defined out of band.
    ///
    /// Use `Failure` instead.
    DefinedOutOfBand = 0x91,
    /// Indicates the action is denied.
    ///
    /// Use `Failure` instead.
    ActionDenied = 0x93,
    /// Indicates a hardware failure.
    ///
    /// Use `Failure` instead.
    HardwareFailure = 0xc0,
    /// Indicates a software failure.
    ///
    /// Use `Failure` instead.
    SoftwareFailure = 0xc1,
    /// Limit reached for the number of entries in a table.
    ///
    /// Use `Success` instead.
    LimitReached = 0xc4,
}

impl From<Deprecated> for Status {
    fn from(value: Deprecated) -> Self {
        match value {
            Deprecated::UnsupportedGeneralCommand => Self::UnsupportedCommand,
            Deprecated::UnsupportedManufacturerClusterCommand => Self::UnsupportedCommand,
            Deprecated::UnsupportedManufacturerGeneralCommand => Self::UnsupportedCommand,
            Deprecated::DuplicateExists => Self::Success,
            Deprecated::WriteOnly => Self::NotAuthorized,
            Deprecated::InconsistentStartupState => Self::Failure,
            Deprecated::DefinedOutOfBand => Self::Failure,
            Deprecated::ActionDenied => Self::Failure,
            Deprecated::HardwareFailure => Self::Failure,
            Deprecated::SoftwareFailure => Self::Failure,
            Deprecated::LimitReached => Self::Success,
        }
    }
}
