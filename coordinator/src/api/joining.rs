use std::time::Duration;

use apis_saltans_hw::Ncp;

use crate::{Coordinator, Error};

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
    fn allow_joining(
        &self,
        duration: Duration,
    ) -> impl Future<Output = Result<Duration, Error>> + Send;
}

impl Joining for Coordinator {
    async fn allow_joining(&self, duration: Duration) -> Result<Duration, Error> {
        Ok(self.ncp.allow_joins(duration).await?)
    }
}
