use core::fmt;
use std::error::Error;
use std::fmt::Display;

/// An error that can occur when parsing an APS frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ParseApsFrameError {
    /// Invalid message type.
    InvalidMessageType(u8),

    /// The ZDP frame is invalid.
    ParseZdpFrameError(zdp::ParseFrameError),

    /// The ZCL frame is invalid.
    ParseZclFrameError(zcl::ParseFrameError),

    /// The profile ID is invalid.
    InvalidProfile(u16),
}

impl Display for ParseApsFrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidMessageType(msg_type) => write!(f, "Invalid message type: {msg_type}"),
            Self::ParseZdpFrameError(error) => error.fmt(f),
            Self::ParseZclFrameError(error) => error.fmt(f),
            Self::InvalidProfile(profile) => write!(f, "Invalid profile ID: {profile}"),
        }
    }
}

impl Error for ParseApsFrameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidMessageType(_) | Self::InvalidProfile(_) => None,
            Self::ParseZdpFrameError(error) => Some(error),
            Self::ParseZclFrameError(error) => Some(error),
        }
    }
}
