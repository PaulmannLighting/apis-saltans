use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::sync::oneshot::Receiver;

use crate::Error;
use crate::aps::TransmissionResponse;

/// Combined APS transmission and correlated protocol response.
///
/// This future first waits for the deferred APS transmission and then for the correlated protocol
/// response. Public callers receive a [`crate::CommunicationResponse`], which additionally
/// converts the raw value to the command's declared response type.
#[must_use = "futures do nothing unless polled"]
#[derive(Debug)]
pub struct ApsProtocolResponse<T> {
    transmission: Option<TransmissionResponse>,
    response: Receiver<T>,
}

impl<T> ApsProtocolResponse<T> {
    /// Create a response from its deferred APS result and protocol correlation channel.
    pub const fn new(transmission: TransmissionResponse, response: Receiver<T>) -> Self {
        Self {
            transmission: Some(transmission),
            response,
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

    use super::{ApsProtocolResponse, TransmissionResponse};

    const RESPONSE: u8 = 42;

    #[test]
    fn waits_for_the_protocol_response() {
        let (sender, receiver) = channel();
        let mut response = pin!(ApsProtocolResponse::new(
            TransmissionResponse::new(None),
            receiver,
        ));
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
    fn waits_for_transmission_before_protocol_response() {
        let (transmission_sender, transmission_receiver) = channel();
        let (protocol_sender, protocol_receiver) = channel();
        let mut response = pin!(ApsProtocolResponse::new(
            TransmissionResponse::new(Some(transmission_receiver)),
            protocol_receiver,
        ));
        let mut context = Context::from_waker(Waker::noop());

        assert_eq!(protocol_sender.send(RESPONSE), Ok(()));
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
        let (_protocol_sender, protocol_receiver) = channel::<u8>();
        let mut response = pin!(ApsProtocolResponse::new(
            TransmissionResponse::new(Some(transmission_receiver)),
            protocol_receiver,
        ));
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
}
