use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::Error;

/// A deferred hardware-operation result.
///
/// Hardware drivers return this type when starting an operation whose completion can be observed
/// later. Awaiting it drives the driver-provided future to completion and maps its error into the
/// common hardware [`Error`] type.
///
/// The response is intentionally opaque so callers can compose it with protocol-level response
/// futures without depending on a driver's concrete completion mechanism.
#[must_use = "futures do nothing unless polled"]
pub struct HwResponse {
    inner: Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>,
}

impl HwResponse {
    /// Wrap a deferred hardware future.
    ///
    /// The future's error is converted into the common hardware [`Error`] type when the response is
    /// awaited.
    pub fn new<T>(future: impl Future<Output = Result<(), T>> + Send + 'static) -> Self
    where
        T: Into<Error>,
    {
        Self {
            inner: Box::pin(async move { future.await.map_err(Into::into) }),
        }
    }
}

impl fmt::Debug for HwResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.debug_struct("HwResponse").finish_non_exhaustive()
    }
}

impl Future for HwResponse {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        self.get_mut().inner.as_mut().poll(context)
    }
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll, Waker};

    use super::HwResponse;
    use crate::Error;

    #[test]
    fn delegates_polling_to_inner_future() {
        let mut response = pin!(HwResponse::new(async { Ok::<(), Error>(()) }));
        let mut context = Context::from_waker(Waker::noop());

        assert!(matches!(
            response.as_mut().poll(&mut context),
            Poll::Ready(Ok(()))
        ));
    }

    #[test]
    fn debug_hides_the_inner_future() {
        let response = HwResponse::new(async { Ok::<(), Error>(()) });

        assert_eq!(format!("{response:?}"), "HwResponse { .. }");
    }
}
