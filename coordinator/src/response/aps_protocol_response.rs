use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::sync::oneshot::Receiver;

use crate::Error;
use crate::aps::TransmissionResponse;
use crate::correlation::Cancellation;

/// Combined APS transmission and correlated protocol response.
///
/// This future first waits for the deferred APS transmission and then for the correlated protocol
/// response. Public callers receive a [`crate::CommunicationResponse`], which additionally
/// converts the raw value to the command's declared response type.
#[must_use = "futures do nothing unless polled"]
#[derive(Debug)]
pub struct ApsProtocolResponse<T> {
    transmission: Option<TransmissionResponse>,
    response: Receiver<Result<T, Error>>,
    cancellation: Cancellation,
}

impl<T> ApsProtocolResponse<T> {
    /// Create a response from its deferred APS result and protocol correlation channel.
    pub const fn new(
        transmission: TransmissionResponse,
        response: Receiver<Result<T, Error>>,
        cancellation: Cancellation,
    ) -> Self {
        Self {
            transmission: Some(transmission),
            response,
            cancellation,
        }
    }
}

impl<T> Future for ApsProtocolResponse<T> {
    type Output = Result<T, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        if let Some(transmission) = &mut this.transmission {
            match Pin::new(transmission).poll(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Ok(())) => {
                    this.transmission = None;
                }
                Poll::Ready(Err(error)) => return Poll::Ready(Err(error.into())),
            }
        }

        match Pin::new(&mut this.response).poll(cx) {
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

    use tokio::sync::oneshot::channel;

    use super::{ApsProtocolResponse, TransmissionResponse};
    use crate::correlation::{Cancellation, Key};

    const CLUSTER_ID: u16 = 2;
    const PROFILE_ID: u16 = 3;
    const RESPONSE: u8 = 42;
    const SEQUENCE: u8 = 1;
    const SHORT_ID: u16 = 1;
    const APS_COUNTER: u8 = 1;

    #[test]
    fn waits_for_the_protocol_response() {
        let (transmission_sender, transmission_receiver) = channel();
        let (sender, receiver) = channel();
        assert!(transmission_sender.send(Ok(())).is_ok());
        let mut response = pin!(response(transmission_receiver, receiver));
        let mut context = Context::from_waker(Waker::noop());

        assert!(matches!(
            response.as_mut().poll(&mut context),
            Poll::Pending
        ));
        assert!(sender.send(Ok(RESPONSE)).is_ok());
        assert!(matches!(
            response.as_mut().poll(&mut context),
            Poll::Ready(Ok(RESPONSE))
        ));
    }

    #[test]
    fn waits_for_transmission_before_protocol_response() {
        let (transmission_sender, transmission_receiver) = channel();
        let (protocol_sender, protocol_receiver) = channel();
        let mut response = pin!(response(transmission_receiver, protocol_receiver));
        let mut context = Context::from_waker(Waker::noop());

        assert!(protocol_sender.send(Ok(RESPONSE)).is_ok());
        assert!(matches!(
            response.as_mut().poll(&mut context),
            Poll::Pending
        ));
        assert!(transmission_sender.send(Ok(())).is_ok());
        assert!(matches!(
            response.as_mut().poll(&mut context),
            Poll::Ready(Ok(RESPONSE))
        ));
    }

    #[test]
    fn returns_transmission_failure_before_protocol_response() {
        let (transmission_sender, transmission_receiver) = channel();
        let (_protocol_sender, protocol_receiver) = channel::<Result<u8, crate::Error>>();
        let mut response = pin!(response(transmission_receiver, protocol_receiver));
        let mut context = Context::from_waker(Waker::noop());

        assert!(
            transmission_sender
                .send(Err(zb_hw::TransmissionError::Timeout.into()))
                .is_ok()
        );
        assert!(matches!(
            response.as_mut().poll(&mut context),
            Poll::Ready(Err(crate::Error::Hardware(zb_hw::Error::Transmission(
                zb_hw::TransmissionError::Timeout
            ))))
        ));
    }

    fn response<T>(
        transmission: tokio::sync::oneshot::Receiver<Result<(), zb_hw::Error>>,
        protocol: tokio::sync::oneshot::Receiver<Result<T, crate::Error>>,
    ) -> ApsProtocolResponse<T> {
        let cancellation = Cancellation::test_new(key(), drop);
        let (aps_inbox, _aps_messages) = tokio::sync::mpsc::channel(1);
        ApsProtocolResponse::new(
            TransmissionResponse::test_new(transmission, APS_COUNTER, aps_inbox.downgrade()),
            protocol,
            cancellation,
        )
    }

    fn key() -> Key {
        Key::new(
            SHORT_ID,
            zb_core::Endpoint::Data,
            CLUSTER_ID,
            PROFILE_ID,
            None,
            SEQUENCE,
        )
    }
}
