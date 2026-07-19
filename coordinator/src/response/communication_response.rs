use std::fmt::Debug;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::Error;
use crate::response::InternalCommunicationResponse;

/// A deferred, typed response to a ZCL or ZDP request.
///
/// The first await on a communication method queues the request and returns this future. Awaiting
/// this future then waits for the hardware transmission to complete, waits for the correlated raw
/// response of type `T`, and converts it to `U` with [`TryFrom`]. Transmission and channel failures
/// are returned as [`Error`]; a failed conversion is returned as [`Error::InvalidResponseType`].
/// Internally, `InternalCommunicationResponse` enforces the ordering of the two receive channels.
///
/// Use the protocol-specific [`crate::ZclResponse`] and [`crate::ZdpResponse`] aliases in public
/// API signatures.
#[must_use = "futures do nothing unless polled"]
#[derive(Debug)]
pub struct CommunicationResponse<T, U> {
    internal: InternalCommunicationResponse<T>,
    target_type: PhantomData<U>,
}

impl<T, U> From<InternalCommunicationResponse<T>> for CommunicationResponse<T, U> {
    fn from(internal: InternalCommunicationResponse<T>) -> Self {
        Self {
            internal,
            target_type: PhantomData,
        }
    }
}

impl<T, U> Future for CommunicationResponse<T, U>
where
    T: Unpin,
    U: TryFrom<T, Error: Debug> + Unpin,
{
    type Output = Result<U, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();

        match Pin::new(&mut this.internal).poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(result) => Poll::Ready(result.and_then(|raw| {
                raw.try_into().map_err(|error| {
                    Error::InvalidResponseType(format!("Received invalid response: {error:?}"))
                })
            })),
        }
    }
}
