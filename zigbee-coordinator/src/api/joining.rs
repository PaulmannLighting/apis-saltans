use std::borrow::Borrow;
use std::time::Duration;

use zigbee_hw::{Ncp, NcpHandle};

use crate::Error;

/// Trait to manage joining the network.
pub trait Joining {
    /// Allow joining for a given duration.
    ///
    /// # Returns
    ///
    /// Returns the actual duration for which joining is allowed.
    /// This may be less than the requested duration if the requested
    /// duration is longer than the maximum allowed duration.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if joining could not be allowed.
    fn allow_joining(&self, duration: Duration) -> impl Future<Output = Result<Duration, Error>>;
}

impl<T> Joining for T
where
    T: Borrow<NcpHandle> + Sync,
{
    async fn allow_joining(&self, duration: Duration) -> Result<Duration, Error> {
        Ok(self.borrow().allow_joins(duration).await?)
    }
}
