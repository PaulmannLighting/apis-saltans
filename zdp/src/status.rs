use std::fmt::{Display, LowerHex, UpperHex};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

/// Status codes returned by various ZDP services.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, FromPrimitive)]
#[repr(u8)]
pub enum Status {
    /// Operation was successful.
    Success = 0x00,
    /// Invalid request type.
    InvalidRequestType = 0x80,
    /// Device not found.
    DeviceNotFound = 0x81,
    /// Invalid endpoint.
    InvalidEndpoint = 0x82,
    /// Device is not active.
    NotActive = 0x83,
    /// Operation not supported.
    NotSupported = 0x84,
    /// Operation timed out.
    Timeout = 0x85,
    /// No match found.
    NoMatch = 0x86,
    /// No entry found.
    NoEntry = 0x88,
    /// No descriptor found.
    NoDescriptor = 0x89,
    /// Insufficient space.
    InsufficientSpace = 0x8A,
    /// Operation not permitted.
    NotPermitted = 0x8B,
    /// Table is full.
    TableFull = 0x8C,
    /// Not authorized.
    NotAuthorized = 0x8D,
    /// Device binding table is full.
    DeviceBindingTableFull = 0x8E,
    /// Invalid index.
    InvalidIndex = 0x8F,
    /// Frame too large.
    FrameTooLarge = 0x90,
    /// Bad key negotiation method.
    BadKeyNegotiationMethod = 0x91,
    /// Temporary failure.
    TemporaryFailure = 0x92,
}

impl Status {
    /// Returns a string representation of the status code.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Success => "SUCCESS",
            Self::InvalidRequestType => "INV_REQUESTTYPE",
            Self::DeviceNotFound => "DEVICE_NOT_FOUND",
            Self::InvalidEndpoint => "INVALID_EP",
            Self::NotActive => "NOT_ACTIVE",
            Self::NotSupported => "NOT_SUPPORTED",
            Self::Timeout => "TIMEOUT",
            Self::NoMatch => "NO_MATCH",
            Self::NoEntry => "NO_ENTRY",
            Self::NoDescriptor => "NO_DESCRIPTOR",
            Self::InsufficientSpace => "INSUFFICIENT_SPACE",
            Self::NotPermitted => "NOT_PERMITTED",
            Self::TableFull => "TABLE_FULL",
            Self::NotAuthorized => "NOT_AUTHORIZED",
            Self::DeviceBindingTableFull => "DEVICE_BINDING_TABLE_FULL",
            Self::InvalidIndex => "INVALID_INDEX",
            Self::FrameTooLarge => "FRAME_TOO_LARGE",
            Self::BadKeyNegotiationMethod => "BAD_KEY_NEGOTIATION_METHOD",
            Self::TemporaryFailure => "TEMPORARY_FAILURE",
        }
    }

    /// Formats the result of a parsed status code.
    ///
    /// # Errors
    ///
    /// Returns a formatting error if the formatter fails.
    pub(crate) fn fmt_result(
        f: &mut std::fmt::Formatter<'_>,
        maybe_status: Result<Self, u8>,
    ) -> std::fmt::Result {
        match maybe_status {
            Ok(status) => write!(f, "{status} ({:#04X})", status as u8),
            Err(code) => write!(f, "RESERVED ({code:#04X})"),
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl UpperHex for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        UpperHex::fmt(&(*self as u8), f)
    }
}

impl LowerHex for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        LowerHex::fmt(&(*self as u8), f)
    }
}

impl From<Status> for u8 {
    fn from(value: Status) -> Self {
        value as Self
    }
}

impl TryFrom<u8> for Status {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value).ok_or(value)
    }
}
