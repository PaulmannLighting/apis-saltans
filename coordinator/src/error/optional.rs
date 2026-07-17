use zb_zcl::Status as ZclStatus;
use zb_zdp::Status as ZdpStatus;

use crate::Error;

/// Converts Zigbee "not found" responses into optional values.
///
/// This trait is implemented for `Result<T, Error>` so callers can distinguish
/// a missing Zigbee resource from transport errors, malformed responses, and
/// other status failures.
pub trait Optional<T> {
    /// Returns `Ok(None)` for Zigbee status values that indicate absence.
    ///
    /// ZCL `NotFound` and ZDP `NoEntry`, `NoMatch`, and `NoDescriptor` are
    /// treated as absent values. Successful results become `Ok(Some(value))`;
    /// every other error is returned unchanged.
    fn optional(self) -> Result<Option<T>, Error>;
}

impl<T> Optional<T> for Result<T, Error> {
    fn optional(self) -> Result<Option<T>, Error> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(
                Error::Zcl(Ok(ZclStatus::NotFound))
                | Error::Zdp(Ok(ZdpStatus::NoEntry | ZdpStatus::NoMatch | ZdpStatus::NoDescriptor)),
            ) => Ok(None),
            Err(other) => Err(other),
        }
    }
}
