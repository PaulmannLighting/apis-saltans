use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::sync::oneshot::Receiver;

/// Deferred result of an APS transmission.
///
/// A response without a receiver represents a transmission that does not request an APS
/// acknowledgement and completes immediately. Otherwise, the future resolves when the hardware
/// reports the acknowledged transmission result.
#[must_use = "futures do nothing unless polled"]
#[derive(Debug)]
pub struct TransmissionResponse {
    response: Option<Receiver<Result<(), zb_hw::Error>>>,
}

impl TransmissionResponse {
    /// Create a deferred APS transmission response.
    pub(crate) const fn new(response: Option<Receiver<Result<(), zb_hw::Error>>>) -> Self {
        Self { response }
    }
}

impl Future for TransmissionResponse {
    type Output = Result<(), zb_hw::Error>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        let Some(response) = &mut this.response else {
            return Poll::Ready(Ok(()));
        };

        match Pin::new(response).poll(context) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(result)) => Poll::Ready(result),
            Poll::Ready(Err(error)) => Poll::Ready(Err(error.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll, Waker};

    use super::TransmissionResponse;

    #[test]
    fn unacknowledged_response_is_ready() {
        let mut response = pin!(TransmissionResponse::new(None));
        let mut context = Context::from_waker(Waker::noop());

        assert!(matches!(
            response.as_mut().poll(&mut context),
            Poll::Ready(Ok(()))
        ));
    }

    #[test]
    fn acknowledged_response_is_deferred() {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let mut response = pin!(TransmissionResponse::new(Some(receiver)));
        let mut context = Context::from_waker(Waker::noop());

        assert!(matches!(
            response.as_mut().poll(&mut context),
            Poll::Pending
        ));
        assert!(sender.send(Ok(())).is_ok());
        assert!(matches!(
            response.as_mut().poll(&mut context),
            Poll::Ready(Ok(()))
        ));
    }
}
