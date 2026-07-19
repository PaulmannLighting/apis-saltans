use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use zb_hw::HwResponse;

use crate::Error;

/// A deferred hardware-transmission result using the coordinator error type.
///
/// This future wraps an [`HwResponse`] and converts a hardware error into [`Error::Hardware`].
#[must_use = "futures do nothing unless polled"]
#[derive(Debug)]
pub struct TransmissionResponse {
    inner: HwResponse,
}

impl From<HwResponse> for TransmissionResponse {
    fn from(inner: HwResponse) -> Self {
        Self { inner }
    }
}

impl Future for TransmissionResponse {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.get_mut().inner)
            .poll(context)
            .map_err(Into::into)
    }
}

#[cfg(test)]
mod tests {
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll, Waker};

    use zb_hw::{Error, HwResponse};

    use super::TransmissionResponse;

    #[test]
    fn returns_success_from_the_hardware_response() {
        let hw_response = HwResponse::new(async { Ok::<(), Error>(()) });
        let mut response = pin!(TransmissionResponse::from(hw_response));
        let mut context = Context::from_waker(Waker::noop());

        assert!(matches!(
            response.as_mut().poll(&mut context),
            Poll::Ready(Ok(()))
        ));
    }

    #[test]
    fn converts_the_hardware_error() {
        let hw_response = HwResponse::new(async { Err::<(), _>(Error::NotImplemented) });
        let mut response = pin!(TransmissionResponse::from(hw_response));
        let mut context = Context::from_waker(Waker::noop());

        assert!(matches!(
            response.as_mut().poll(&mut context),
            Poll::Ready(Err(crate::Error::Hardware(Error::NotImplemented)))
        ));
    }
}
