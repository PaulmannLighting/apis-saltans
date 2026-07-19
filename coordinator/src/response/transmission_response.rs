use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::sync::oneshot::Receiver;

use crate::Error;

/// A deferred hardware-transmission result for a command with no protocol response.
///
/// The first await on [`crate::Zcl::transmit`] or a command helper such as [`crate::OnOff::on`]
/// queues the command and returns this future. Await this value to confirm that the hardware driver
/// completed the transmission. Dropping it discards only the completion notification; it does not
/// cancel a command that has already been queued.
#[must_use = "futures do nothing unless polled"]
#[derive(Debug)]
pub struct TransmissionResponse {
    transmission_rx: Receiver<Result<(), zb_hw::Error>>,
}

impl From<Receiver<Result<(), zb_hw::Error>>> for TransmissionResponse {
    fn from(transmission_rx: Receiver<Result<(), zb_hw::Error>>) -> Self {
        Self { transmission_rx }
    }
}

impl Future for TransmissionResponse {
    type Output = Result<(), Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        match Pin::new(&mut this.transmission_rx).poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Ok(result)) => Poll::Ready(result.map_err(Into::into)),
            Poll::Ready(Err(error)) => Poll::Ready(Err(error.into())),
        }
    }
}
