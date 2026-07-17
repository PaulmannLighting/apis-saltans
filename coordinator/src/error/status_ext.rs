use zb_zcl::Status as ZclStatus;
use zb_zdp::Status as ZdpStatus;

use crate::Error;

/// Converts Zigbee status values into coordinator results.
///
/// This extension trait is implemented for parsed ZCL and ZDP status values,
/// and for the `Result<Status, u8>` values returned by fallible status parsing.
/// Success becomes `Ok(())`. Any non-success status, including an unknown raw
/// status byte, is converted into [`Error`].
pub trait StatusExt {
    /// Returns `Ok(())` when the status is `Success`.
    ///
    /// Non-success statuses are returned as [`Error::Zcl`] or [`Error::Zdp`],
    /// depending on the status protocol. For fallibly parsed statuses, unknown
    /// raw status bytes are preserved in the returned error. Callers can use
    /// [`Result::map`] on the returned value to attach a success payload.
    fn ensure_success(self) -> Result<(), Error>;
}

impl StatusExt for ZclStatus {
    fn ensure_success(self) -> Result<(), Error> {
        match self {
            Self::Success => Ok(()),
            other => Err(Ok(other).into()),
        }
    }
}

impl StatusExt for Result<ZclStatus, u8> {
    fn ensure_success(self) -> Result<(), Error> {
        match self {
            Ok(ZclStatus::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}

impl StatusExt for ZdpStatus {
    fn ensure_success(self) -> Result<(), Error> {
        match self {
            Self::Success => Ok(()),
            other => Err(Ok(other).into()),
        }
    }
}

impl StatusExt for Result<ZdpStatus, u8> {
    fn ensure_success(self) -> Result<(), Error> {
        match self {
            Ok(ZdpStatus::Success) => Ok(()),
            other => Err(other.into()),
        }
    }
}
