//! Response timeout handling.

use std::time::Duration;

use const_env::env_item;
use tokio::time::error::Elapsed;

/// Timeout for ZCL responses.
#[env_item("ZIGBEE_COORDINATOR_ZCL_RESPONSE_TIMEOUT_SECS")]
const ZCL_RESPONSE_TIMEOUT_SECS: u64 = 3;
const ZCL_RESPONSE_TIMEOUT: Duration = Duration::from_secs(ZCL_RESPONSE_TIMEOUT_SECS);

/// Extension trait to add a timeout while waiting for a future.
pub trait Timeout {
    type Output;

    /// Wait for the future to complete, or timeout.
    fn timeout(self, timeout: Duration) -> impl Future<Output = Result<Self::Output, Elapsed>>;

    /// Wait for a ZCL response, or timeout.
    fn zcl_response_timeout(self) -> impl Future<Output = Result<Self::Output, Elapsed>>
    where
        Self: Sized,
    {
        self.timeout(ZCL_RESPONSE_TIMEOUT)
    }
}

impl<T> Timeout for T
where
    T: IntoFuture,
{
    type Output = <Self as IntoFuture>::Output;

    async fn timeout(self, timeout: Duration) -> Result<Self::Output, Elapsed> {
        tokio::time::timeout(timeout, self).await
    }
}
