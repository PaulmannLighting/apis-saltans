use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use log::debug;
use tokio::runtime::Handle;
use tokio::sync::mpsc::WeakSender;
use tokio::sync::mpsc::error::TrySendError;
use tokio::sync::oneshot::Receiver;

use super::{Message, TransmissionToken};

#[derive(Debug)]
struct Cancellation {
    token: TransmissionToken,
    inbox: WeakSender<Message>,
    runtime: Option<Handle>,
    armed: bool,
}

impl Cancellation {
    fn new(token: TransmissionToken, inbox: WeakSender<Message>) -> Self {
        Self {
            token,
            inbox,
            runtime: Handle::try_current().ok(),
            armed: true,
        }
    }

    const fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for Cancellation {
    fn drop(&mut self) {
        if self.armed {
            let Some(inbox) = self.inbox.upgrade() else {
                return;
            };
            match inbox.try_send(Message::Cancel { token: self.token }) {
                Ok(()) => {}
                Err(TrySendError::Full(message)) => {
                    let Some(runtime) = &self.runtime else {
                        debug!(
                            "Failed to enqueue APS transmission cancellation: runtime unavailable"
                        );
                        return;
                    };
                    runtime.spawn(async move {
                        inbox.send(message).await.unwrap_or_else(|error| {
                            debug!("Failed to enqueue APS transmission cancellation: {error}");
                        });
                    });
                }
                Err(TrySendError::Closed(_)) => {
                    debug!("Failed to enqueue APS transmission cancellation: actor unavailable");
                }
            }
        }
    }
}

/// Deferred result of an APS transmission.
///
/// The future resolves after the hardware backend accepts or rejects the frame. If the frame
/// requests an APS acknowledgement, acceptance is followed by the corresponding acknowledged
/// transmission result.
#[must_use = "futures do nothing unless polled"]
#[derive(Debug)]
pub struct TransmissionResponse {
    response: Receiver<Result<(), zb_hw::Error>>,
    cancellation: Cancellation,
}

impl TransmissionResponse {
    /// Create a deferred APS transmission response.
    pub(crate) fn new(
        response: Receiver<Result<(), zb_hw::Error>>,
        token: TransmissionToken,
        inbox: WeakSender<Message>,
    ) -> Self {
        Self {
            response,
            cancellation: Cancellation::new(token, inbox),
        }
    }

    #[cfg(test)]
    pub(crate) fn test_new(
        response: Receiver<Result<(), zb_hw::Error>>,
        counter: u8,
        inbox: WeakSender<Message>,
    ) -> Self {
        Self::new(
            response,
            TransmissionToken {
                counter,
                generation: super::INITIAL_GENERATION,
            },
            inbox,
        )
    }
}

impl Future for TransmissionResponse {
    type Output = Result<(), zb_hw::Error>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        match Pin::new(&mut this.response).poll(context) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(result)) => {
                this.cancellation.disarm();
                Poll::Ready(result)
            }
            Poll::Ready(Err(error)) => Poll::Ready(Err(error.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll, Waker};

    use tokio::sync::mpsc::channel;

    use super::{Message, TransmissionResponse};

    const APS_COUNTER: u8 = 1;

    #[test]
    fn accepted_response_is_ready() {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        assert!(sender.send(Ok(())).is_ok());
        let (inbox, _messages) = channel(1);
        let mut response = pin!(TransmissionResponse::test_new(
            receiver,
            APS_COUNTER,
            inbox.downgrade(),
        ));
        let mut context = Context::from_waker(Waker::noop());

        assert!(matches!(
            response.as_mut().poll(&mut context),
            Poll::Ready(Ok(()))
        ));
    }

    #[test]
    fn acknowledged_response_is_deferred() {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        let (inbox, _messages) = channel(1);
        let mut response = pin!(TransmissionResponse::test_new(
            receiver,
            APS_COUNTER,
            inbox.downgrade(),
        ));
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

    #[test]
    fn dropping_pending_response_requests_cancellation() {
        let (_sender, receiver) = tokio::sync::oneshot::channel();
        let (inbox, mut messages) = channel(1);
        let response = TransmissionResponse::test_new(receiver, APS_COUNTER, inbox.downgrade());

        drop(response);

        assert!(matches!(
            messages.try_recv(),
            Ok(Message::Cancel { token }) if token.counter == APS_COUNTER
        ));
    }
}
