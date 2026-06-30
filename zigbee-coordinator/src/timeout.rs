//! Response timeout handling.

use std::time::Duration;

use const_env::env_item;

use crate::Error;

/// Timeout for ZCL responses.
#[env_item("ZIGBEE_COORDINATOR_ZCL_RESPONSE_TIMEOUT_SECS")]
const ZCL_RESPONSE_TIMEOUT_SECS: u64 = 10;
const ZCL_RESPONSE_TIMEOUT: Duration = Duration::from_secs(ZCL_RESPONSE_TIMEOUT_SECS);

/// Extension trait to add a timeout while waiting for a future.
pub trait Timeout<S, E> {
    /// Wait for the future to complete, or timeout.
    fn timeout(self, timeout: Duration) -> impl Future<Output = Result<S, Error>>;
}

impl<T, S, E> Timeout<S, E> for T
where
    T: IntoFuture<Output = Result<S, E>>,
    Error: From<E>,
{
    async fn timeout(self, timeout: Duration) -> Result<S, Error> {
        Ok(tokio::time::timeout(timeout, self).await??)
    }
}
