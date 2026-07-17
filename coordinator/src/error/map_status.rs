use zb_zcl::Status as ZclStatus;
use zb_zdp::Status as ZdpStatus;

use crate::Error;

/// Maps a successful Zigbee status into a caller-provided value.
///
/// This extension trait is implemented for parsed ZCL and ZDP status values,
/// and for the `Result<Status, u8>` values returned by fallible status parsing.
/// Success produces the value returned by the closure. Any non-success status,
/// including an unknown raw status byte, is converted into [`Error`].
pub trait MapStatus {
    /// Returns `Ok(f())` when the status is `Success`.
    ///
    /// Non-success statuses are returned as [`Error::Zcl`] or [`Error::Zdp`],
    /// depending on the status protocol. For fallibly parsed statuses, unknown
    /// raw status bytes are preserved in the returned error.
    fn map_success<T>(self, f: impl FnOnce() -> T) -> Result<T, Error>;
}

impl MapStatus for ZclStatus {
    fn map_success<T>(self, f: impl FnOnce() -> T) -> Result<T, Error> {
        match self {
            Self::Success => Ok(f()),
            other => Err(Ok(other).into()),
        }
    }
}

impl MapStatus for Result<ZclStatus, u8> {
    fn map_success<T>(self, f: impl FnOnce() -> T) -> Result<T, Error> {
        match self {
            Ok(ZclStatus::Success) => Ok(f()),
            other => Err(other.into()),
        }
    }
}

impl MapStatus for ZdpStatus {
    fn map_success<T>(self, f: impl FnOnce() -> T) -> Result<T, Error> {
        match self {
            Self::Success => Ok(f()),
            other => Err(Ok(other).into()),
        }
    }
}

impl MapStatus for Result<ZdpStatus, u8> {
    fn map_success<T>(self, f: impl FnOnce() -> T) -> Result<T, Error> {
        match self {
            Ok(ZdpStatus::Success) => Ok(f()),
            other => Err(other.into()),
        }
    }
}
