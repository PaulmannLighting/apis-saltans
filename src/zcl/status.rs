use deprecated::Deprecated;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

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

impl TryFrom<u8> for Status {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value)
            .or_else(|| Deprecated::from_u8(value).map(Into::into))
            .ok_or(value)
    }
}

impl From<Status> for u8 {
    fn from(value: Status) -> Self {
        value as Self
    }
}
