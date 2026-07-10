use core::fmt;
use std::error::Error;
use std::fmt::Display;

/// An error that can occur when parsing an APS frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum ParseApsFrameError {
    /// The ZCL frame is invalid.
    ParseZclFrameError(zb_zcl::ParseFrameError),

    /// The ZDP frame is invalid.
    ParseZdpFrameError(zb_zdp::ParseFrameError),

    /// The profile ID is invalid.
    InvalidProfile(u16),
}

impl Display for ParseApsFrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseZclFrameError(error) => error.fmt(f),
            Self::ParseZdpFrameError(error) => error.fmt(f),
            Self::InvalidProfile(profile) => write!(f, "Invalid profile ID: {profile}"),
        }
    }
}

impl Error for ParseApsFrameError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::ParseZclFrameError(error) => Some(error),
            Self::ParseZdpFrameError(error) => Some(error),
            Self::InvalidProfile(_) => None,
        }
    }
}

impl From<zb_zcl::ParseFrameError> for ParseApsFrameError {
    fn from(error: zb_zcl::ParseFrameError) -> Self {
        Self::ParseZclFrameError(error)
    }
}

impl From<zb_zdp::ParseFrameError> for ParseApsFrameError {
    fn from(error: zb_zdp::ParseFrameError) -> Self {
        Self::ParseZdpFrameError(error)
    }
}
