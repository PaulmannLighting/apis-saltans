use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use self::deprecated::Deprecated;

mod deprecated;

/// Available ZCL status codes.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, FromPrimitive)]
#[repr(u8)]
pub enum Status {
    /// Indicates the command was successful.
    Success = 0x00,
    /// Indicates the command failed.
    Failure = 0x01,
    /// Indicates the command is not authorized.
    NotAuthorized = 0x7e,
    /// Indicates the command was malformed.
    MalformedCommand = 0x80,
    /// Indicates the cluster command is not supported.
    UnsupportedCommand = 0x81,
    /// Indicates the field in the command is invalid.
    InvalidField = 0x85,
    /// Indicates the attribute is unsupported.
    UnsupportedAttribute = 0x86,
    /// Indicates the value of the attribute is invalid.
    InvalidValue = 0x87,
    /// Indicates the attribute is read-only.
    ReadOnly = 0x88,
    /// Indicates there is insufficient space to perform the operation.
    InsufficientSpace = 0x89,
    /// Indicates the requested entry was not found.
    NotFound = 0x8b,
    /// Indicates the attribute is unreportable.
    UnreportableAttribute = 0x8c,
    /// Indicates the data type of the attribute is invalid.
    InvalidDataType = 0x8d,
    /// Indicates the selector is invalid.
    InvalidSelector = 0x8e,
    /// Indicates the command timed out.
    Timeout = 0x94,
    /// Indicates the command was aborted.
    Abort = 0x95,
    /// Indicates the image is invalid.
    InvalidImage = 0x96,
    /// Indicates the device is waiting for data.
    WaitForData = 0x97,
    /// Indicates no image is available.
    NoImageAvailable = 0x98,
    /// Indicates more image data is required.
    RequireMoreImage = 0x99,
    /// Indicates the notification is pending.
    NotificationPending = 0x9a,
    /// Indicates that the cluster is unsupported.
    UnsupportedCluster = 0xc3,
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

impl From<Status> for u8 {
    fn from(value: Status) -> Self {
        value as Self
    }
}

impl TryFrom<u8> for Status {
    type Error = u8;

    /// Attempts to convert a `u8` value into a `Status`.
    ///
    /// If the value is deprecated, it will be automatically converted to the corresponding new `Status` value.
    ///
    /// # Errors
    ///
    /// Returns the original `u8` value if it does not correspond to a valid `Status` or a deprecated status.
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value)
            .or_else(|| Deprecated::from_u8(value).map(Into::into))
            .ok_or(value)
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
            assert_eq!(Status::try_from(value), Err(value));
        }
    }
}
