use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::sync::oneshot::Receiver;

use crate::Error;

/// Crate-internal receiver for a correlated protocol response.
///
/// The APS transmission has already completed successfully before this future is returned. Public
/// callers receive a [`crate::CommunicationResponse`], which additionally converts the raw value
/// to the command's declared response type.
#[must_use = "futures do nothing unless polled"]
#[derive(Debug)]
pub struct InternalCommunicationResponse<T> {
    response: Receiver<T>,
}

impl<T> InternalCommunicationResponse<T> {
    /// Create a protocol response from its correlation channel.
    pub const fn new(response: Receiver<T>) -> Self {
        Self { response }
    }
}

impl<T> Future for InternalCommunicationResponse<T> {
    type Output = Result<T, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        match Pin::new(&mut this.response).poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(result)) => Poll::Ready(Ok(result)),
            Poll::Ready(Err(error)) => Poll::Ready(Err(error.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll, Waker};

    use tokio::sync::oneshot::channel;

    use super::InternalCommunicationResponse;

    const RESPONSE: u8 = 42;

    #[test]
    fn waits_for_the_protocol_response() {
        let (sender, receiver) = channel();
        let mut response = pin!(InternalCommunicationResponse::new(receiver));
        let mut context = Context::from_waker(Waker::noop());

        assert!(matches!(
            response.as_mut().poll(&mut context),
            Poll::Pending
        ));
        assert_eq!(sender.send(RESPONSE), Ok(()));
        assert!(matches!(
            response.as_mut().poll(&mut context),
            Poll::Ready(Ok(RESPONSE))
        ));
    }
}
