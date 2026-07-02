//! Response timeout handling.

use std::time::Duration;

use crate::Error;

/// Extension trait to add a timeout while waiting for a future.
pub trait Timeout<S, E> {
    /// Wait for the future to complete, or timeout.
    fn timeout(self, timeout: Duration) -> impl Future<Output = Result<S, Error>> + Send;
}

impl<T, S, E> Timeout<S, E> for T
where
    T: IntoFuture<Output = Result<S, E>, IntoFuture: Send> + Send,
    Error: From<E>,
{
    async fn timeout(self, timeout: Duration) -> Result<S, Error> {
        Ok(tokio::time::timeout(timeout, self).await??)
    }
}
