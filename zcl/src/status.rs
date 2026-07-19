use std::fmt::{Formatter, LowerHex, UpperHex};

use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;

use self::deprecated::Deprecated;

mod deprecated;

/// Available ZCL status codes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(
    Clone, Copy, Debug, Eq, Error, Hash, IntoPrimitive, Ord, PartialEq, PartialOrd, TryFromPrimitive,
)]
#[num_enum(error_type(name = u8, constructor = core::convert::identity))]
#[repr(u8)]
pub enum Status {
    /// Indicates the command was successful.
    #[error("SUCCESS")]
    #[num_enum(alternatives = [0x8a, 0xc4])]
    Success = 0x00,

    /// Indicates the command failed.
    #[error("FAILURE")]
    #[num_enum(alternatives = [0x90, 0x91, 0x93, 0xc0, 0xc1])]
    Failure = 0x01,

    /// Indicates the command is not authorized.
    #[error("NOT_AUTHORIZED")]
    #[num_enum(alternatives = [0x8f])]
    NotAuthorized = 0x7e,

    /// Indicates the command was malformed.
    #[error("MALFORMED_COMMAND")]
    MalformedCommand = 0x80,

    /// Indicates the cluster command is not supported.
    #[error("UNSUP_COMMAND")]
    #[num_enum(alternatives = [0x82, 0x83, 0x84])]
    UnsupportedCommand = 0x81,

    /// Indicates the field in the command is invalid.
    #[error("INVALID_FIELD")]
    InvalidField = 0x85,

    /// Indicates the attribute is unsupported.
    #[error("UNSUPPORTED_ATTRIBUTE")]
    UnsupportedAttribute = 0x86,

    /// Indicates the value of the attribute is invalid.
    #[error("INVALID_VALUE")]
    InvalidValue = 0x87,

    /// Indicates the attribute is readable-only.
    #[error("READ_ONLY")]
    ReadOnly = 0x88,

    /// Indicates there is insufficient space to perform the operation.
    #[error("INSUFFICIENT_SPACE")]
    InsufficientSpace = 0x89,

    /// Indicates the requested entry was not found.
    #[error("NOT_FOUND")]
    NotFound = 0x8b,

    /// Indicates the attribute is unreportable.
    #[error("UNREPORTABLE_ATTRIBUTE")]
    UnreportableAttribute = 0x8c,

    /// Indicates the data type of the attribute is invalid.
    #[error("INVALID_DATA_TYPE")]
    InvalidDataType = 0x8d,

    /// Indicates the selector is invalid.
    #[error("INVALID_SELECTOR")]
    InvalidSelector = 0x8e,

    /// Indicates the command timed out.
    #[error("TIMEOUT")]
    Timeout = 0x94,

    /// Indicates the command was aborted.
    #[error("ABORT")]
    Abort = 0x95,

    /// Indicates the image is invalid.
    #[error("INVALID_IMAGE")]
    InvalidImage = 0x96,

    /// Indicates the device is waiting for data.
    #[error("WAIT_FOR_DATA")]
    WaitForData = 0x97,

    /// Indicates no image is available.
    #[error("NO_IMAGE_AVAILABLE")]
    NoImageAvailable = 0x98,

    /// Indicates more image data is required.
    #[error("REQUIRE_MORE_IMAGE")]
    RequireMoreImage = 0x99,

    /// Indicates the notification is pending.
    #[error("NOTIFICATION_PENDING")]
    NotificationPending = 0x9a,

    /// Indicates that the cluster is unsupported.
    #[error("UNSUPPORTED_CLUSTER")]
    UnsupportedCluster = 0xc3,
}

impl Status {
    /// Returns a string representation of the status code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Success => "SUCCESS",
            Self::Failure => "FAILURE",
            Self::NotAuthorized => "NOT_AUTHORIZED",
            Self::MalformedCommand => "MALFORMED_COMMAND",
            Self::UnsupportedCommand => "UNSUP_COMMAND",
            Self::InvalidField => "INVALID_FIELD",
            Self::UnsupportedAttribute => "UNSUPPORTED_ATTRIBUTE",
            Self::InvalidValue => "INVALID_VALUE",
            Self::ReadOnly => "READ_ONLY",
            Self::InsufficientSpace => "INSUFFICIENT_SPACE",
            Self::NotFound => "NOT_FOUND",
            Self::UnreportableAttribute => "UNREPORTABLE_ATTRIBUTE",
            Self::InvalidDataType => "INVALID_DATA_TYPE",
            Self::InvalidSelector => "INVALID_SELECTOR",
            Self::Timeout => "TIMEOUT",
            Self::Abort => "ABORT",
            Self::InvalidImage => "INVALID_IMAGE",
            Self::WaitForData => "WAIT_FOR_DATA",
            Self::NoImageAvailable => "NO_IMAGE_AVAILABLE",
            Self::RequireMoreImage => "REQUIRE_MORE_IMAGE",
            Self::NotificationPending => "NOTIFICATION_PENDING",
            Self::UnsupportedCluster => "UNSUPPORTED_CLUSTER",
        }
    }
}

impl LowerHex for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        LowerHex::fmt(&(*self as u8), f)
    }
}

impl UpperHex for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        UpperHex::fmt(&(*self as u8), f)
    }
}

impl From<Deprecated> for Status {
    fn from(value: Deprecated) -> Self {
        match value {
            Deprecated::UnsupportedGeneralCommand
            | Deprecated::UnsupportedManufacturerClusterCommand
            | Deprecated::UnsupportedManufacturerGeneralCommand => Self::UnsupportedCommand,
            Deprecated::DuplicateExists | Deprecated::LimitReached => Self::Success,
            Deprecated::WriteOnly => Self::NotAuthorized,
            Deprecated::InconsistentStartupState
            | Deprecated::DefinedOutOfBand
            | Deprecated::ActionDenied
            | Deprecated::HardwareFailure
            | Deprecated::SoftwareFailure => Self::Failure,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_to_u8() {
        assert_eq!(u8::from(Status::Success), 0x00);
        assert_eq!(u8::from(Status::Failure), 0x01);
        assert_eq!(u8::from(Status::NotAuthorized), 0x7e);
        assert_eq!(u8::from(Status::MalformedCommand), 0x80);
        assert_eq!(u8::from(Status::UnsupportedCommand), 0x81);
        assert_eq!(u8::from(Status::InvalidField), 0x85);
        assert_eq!(u8::from(Status::UnsupportedAttribute), 0x86);
        assert_eq!(u8::from(Status::InvalidValue), 0x87);
        assert_eq!(u8::from(Status::ReadOnly), 0x88);
        assert_eq!(u8::from(Status::InsufficientSpace), 0x89);
        assert_eq!(u8::from(Status::NotFound), 0x8b);
        assert_eq!(u8::from(Status::UnreportableAttribute), 0x8c);
        assert_eq!(u8::from(Status::InvalidDataType), 0x8d);
        assert_eq!(u8::from(Status::InvalidSelector), 0x8e);
        assert_eq!(u8::from(Status::Timeout), 0x94);
        assert_eq!(u8::from(Status::Abort), 0x95);
        assert_eq!(u8::from(Status::InvalidImage), 0x96);
        assert_eq!(u8::from(Status::WaitForData), 0x97);
        assert_eq!(u8::from(Status::NoImageAvailable), 0x98);
        assert_eq!(u8::from(Status::RequireMoreImage), 0x99);
        assert_eq!(u8::from(Status::NotificationPending), 0x9a);
        assert_eq!(u8::from(Status::UnsupportedCluster), 0xc3);
    }

    #[test]
    fn status_from_u8() {
        assert_eq!(Status::try_from(0x00), Ok(Status::Success));
        assert_eq!(Status::try_from(0x01), Ok(Status::Failure));
        assert_eq!(Status::try_from(0x7e), Ok(Status::NotAuthorized));
        assert_eq!(Status::try_from(0x80), Ok(Status::MalformedCommand));
        assert_eq!(Status::try_from(0x81), Ok(Status::UnsupportedCommand));
        assert_eq!(Status::try_from(0x85), Ok(Status::InvalidField));
        assert_eq!(Status::try_from(0x86), Ok(Status::UnsupportedAttribute));
        assert_eq!(Status::try_from(0x87), Ok(Status::InvalidValue));
        assert_eq!(Status::try_from(0x88), Ok(Status::ReadOnly));
        assert_eq!(Status::try_from(0x89), Ok(Status::InsufficientSpace));
        assert_eq!(Status::try_from(0x8b), Ok(Status::NotFound));
        assert_eq!(Status::try_from(0x8c), Ok(Status::UnreportableAttribute));
        assert_eq!(Status::try_from(0x8d), Ok(Status::InvalidDataType));
        assert_eq!(Status::try_from(0x8e), Ok(Status::InvalidSelector));
        assert_eq!(Status::try_from(0x94), Ok(Status::Timeout));
        assert_eq!(Status::try_from(0x95), Ok(Status::Abort));
        assert_eq!(Status::try_from(0x96), Ok(Status::InvalidImage));
        assert_eq!(Status::try_from(0x97), Ok(Status::WaitForData));
        assert_eq!(Status::try_from(0x98), Ok(Status::NoImageAvailable));
        assert_eq!(Status::try_from(0x99), Ok(Status::RequireMoreImage));
        assert_eq!(Status::try_from(0x9a), Ok(Status::NotificationPending));
        assert_eq!(Status::try_from(0xc3), Ok(Status::UnsupportedCluster));
    }

    #[test]
    fn deprecated_from_u8() {
        assert_eq!(Status::try_from(0x82), Ok(Status::UnsupportedCommand));
        assert_eq!(Status::try_from(0x83), Ok(Status::UnsupportedCommand));
        assert_eq!(Status::try_from(0x84), Ok(Status::UnsupportedCommand));
        assert_eq!(Status::try_from(0x8a), Ok(Status::Success));
        assert_eq!(Status::try_from(0x8f), Ok(Status::NotAuthorized));
        assert_eq!(Status::try_from(0x90), Ok(Status::Failure));
        assert_eq!(Status::try_from(0x91), Ok(Status::Failure));
        assert_eq!(Status::try_from(0x93), Ok(Status::Failure));
        assert_eq!(Status::try_from(0xc0), Ok(Status::Failure));
        assert_eq!(Status::try_from(0xc1), Ok(Status::Failure));
        assert_eq!(Status::try_from(0xc4), Ok(Status::Success));
    }

    /// Test some invalid `u8` values that should not correspond to any `Status`.
    ///
    /// Note: This does not include deprecated values, which are handled gracefully.
    /// This also does not cover gaps in-between valid values.
    #[test]
    fn some_invalid_from_u8() {
        for value in 0xc5..=u8::MAX {
            assert!(Status::try_from(value).is_err());
        }
    }
}
