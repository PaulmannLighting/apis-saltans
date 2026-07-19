use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::sync::oneshot::Receiver;
use zb_hw::HwResponse;

use crate::Error;

/// Crate-internal communication response that sequences transmission before protocol reception.
///
/// The raw response channel is not polled until the [`HwResponse`] has completed successfully. Public
/// callers receive a [`crate::CommunicationResponse`], which additionally converts the raw value
/// to the command's declared response type.
#[must_use = "futures do nothing unless polled"]
#[derive(Debug)]
pub struct InternalCommunicationResponse<T> {
    hw_response: Option<HwResponse>,
    response: Receiver<T>,
}

impl<T> InternalCommunicationResponse<T> {
    pub const fn new(hw_response: HwResponse, response: Receiver<T>) -> Self {
        Self {
            hw_response: Some(hw_response),
            response,
        }
    }
}

impl<T> Future for InternalCommunicationResponse<T> {
    type Output = Result<T, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if let Some(hw_response) = this.hw_response.as_mut() {
            match Pin::new(hw_response).poll(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Ok(())) => this.hw_response = None,
                Poll::Ready(Err(error)) => return Poll::Ready(Err(error.into())),
            }
        }

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
    use zb_hw::{Error, HwResponse};

    use super::InternalCommunicationResponse;

    const RESPONSE: u8 = 42;

    #[test]
    fn waits_for_the_protocol_response_after_transmission() {
        let (sender, receiver) = channel();
        let hw_response = HwResponse::new(async { Ok::<(), Error>(()) });
        let mut response = pin!(InternalCommunicationResponse::new(hw_response, receiver));
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

    #[test]
    fn returns_a_hardware_error_without_waiting_for_the_protocol_response() {
        let (_sender, receiver) = channel::<u8>();
        let hw_response = HwResponse::new(async { Err::<(), _>(Error::NotImplemented) });
        let mut response = pin!(InternalCommunicationResponse::new(hw_response, receiver));
        let mut context = Context::from_waker(Waker::noop());

        assert!(matches!(
            response.as_mut().poll(&mut context),
            Poll::Ready(Err(crate::Error::Hardware(Error::NotImplemented)))
        ));
    }
}
