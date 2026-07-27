use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::sync::oneshot::Receiver;

/// Deferred result of an APS transmission.
///
/// The future resolves after the hardware backend accepts or rejects the frame. If the frame
/// requests an APS acknowledgement, acceptance is followed by the corresponding acknowledged
/// transmission result.
#[must_use = "futures do nothing unless polled"]
#[derive(Debug)]
pub struct TransmissionResponse {
    response: Receiver<Result<(), zb_hw::Error>>,
}

impl TransmissionResponse {
    /// Create a deferred APS transmission response.
    pub(crate) const fn new(response: Receiver<Result<(), zb_hw::Error>>) -> Self {
        Self { response }
    }
}

impl Future for TransmissionResponse {
    type Output = Result<(), zb_hw::Error>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        match Pin::new(&mut this.response).poll(context) {
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
    fn accepted_response_is_ready() {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        assert!(sender.send(Ok(())).is_ok());
        let mut response = pin!(TransmissionResponse::new(receiver));
        let mut context = Context::from_waker(Waker::noop());

        assert!(matches!(
            response.as_mut().poll(&mut context),
            Poll::Ready(Ok(()))
        ));
    }

    #[test]
    fn acknowledged_response_is_deferred() {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let mut response = pin!(TransmissionResponse::new(receiver));
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
