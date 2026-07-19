use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::sync::oneshot::Receiver;
use zb_hw::Error;

/// Crate-internal communication response that sequences transmission before protocol reception.
///
/// The raw response channel is not polled until transmission has completed successfully. Public
/// callers receive a [`crate::CommunicationResponse`], which additionally converts the raw value
/// to the command's declared response type.
#[must_use = "futures do nothing unless polled"]
#[derive(Debug)]
pub struct InternalCommunicationResponse<T> {
    transmission_rx: Receiver<Result<(), Error>>,
    response: Receiver<T>,
}

impl<T> InternalCommunicationResponse<T> {
    pub const fn new(transmission_rx: Receiver<Result<(), Error>>, response: Receiver<T>) -> Self {
        Self {
            transmission_rx,
            response,
        }
    }
}

impl<T> Future for InternalCommunicationResponse<T> {
    type Output = Result<T, crate::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if !this.transmission_rx.is_terminated() {
            match Pin::new(&mut this.transmission_rx).poll(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Ok(Ok(()))) => {}
                Poll::Ready(Ok(Err(error))) => return Poll::Ready(Err(error.into())),
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
