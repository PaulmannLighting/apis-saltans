use std::fmt::{Formatter, LowerHex, UpperHex};

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use thiserror::Error;

pub use self::displayable::Displayable;

mod displayable;

/// Status codes returned by various ZDP services.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq, Hash, FromPrimitive)]
#[repr(u8)]
pub enum Status {
    /// Operation was successful.
    #[error("SUCCESS")]
    Success = 0x00,

    /// Invalid request type.
    #[error("INV_REQUESTTYPE")]
    InvalidRequestType = 0x80,

    /// Device not found.
    #[error("DEVICE_NOT_FOUND")]
    DeviceNotFound = 0x81,

    /// Invalid endpoint.
    #[error("INVALID_EP")]
    InvalidEndpoint = 0x82,

    /// Device is not active.
    #[error("NOT_ACTIVE")]
    NotActive = 0x83,

    /// Operation not supported.
    #[error("NOT_SUPPORTED")]
    NotSupported = 0x84,

    /// Operation timed out.
    #[error("TIMEOUT")]
    Timeout = 0x85,

    /// No match found.
    #[error("NO_MATCH")]
    NoMatch = 0x86,

    /// No entry found.
    #[error("NO_ENTRY")]
    NoEntry = 0x88,

    /// No descriptor found.
    #[error("NO_DESCRIPTOR")]
    NoDescriptor = 0x89,

    /// Insufficient space.
    #[error("INSUFFICIENT_SPACE")]
    InsufficientSpace = 0x8A,

    /// Operation not permitted.
    #[error("NOT_PERMITTED")]
    NotPermitted = 0x8B,

    /// Table is full.
    #[error("TABLE_FULL")]
    TableFull = 0x8C,

    /// Not authorized.
    #[error("NOT_AUTHORIZED")]
    NotAuthorized = 0x8D,

    /// Device binding table is full.
    #[error("DEVICE_BINDING_TABLE_FULL")]
    DeviceBindingTableFull = 0x8E,

    /// Invalid index.
    #[error("INVALID_INDEX")]
    InvalidIndex = 0x8F,

    /// Frame too large.
    #[error("FRAME_TOO_LARGE")]
    FrameTooLarge = 0x90,

    /// Bad key negotiation method.
    #[error("BAD_KEY_NEGOTIATION_METHOD")]
    BadKeyNegotiationMethod = 0x91,

    /// Temporary failure.
    #[error("TEMPORARY_FAILURE")]
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
}

impl UpperHex for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        UpperHex::fmt(&(*self as u8), f)
    }
}

impl LowerHex for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
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
