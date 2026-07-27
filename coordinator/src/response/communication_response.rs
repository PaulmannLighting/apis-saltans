use std::fmt::Debug;
use std::future::Future;
use std::marker::PhantomData;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::Error;
use crate::response::ApsProtocolResponse;

/// A deferred, typed response to a ZCL or ZDP request.
///
/// Awaiting this future first completes the deferred APS transmission, then waits for the
/// correlated raw response of type `T` and converts it to `U` with [`TryFrom`]. Channel failures
/// are returned as [`Error`]; a failed conversion is returned as [`Error::InvalidResponseType`].
///
/// Use the protocol-specific [`crate::ZclResponse`] and [`crate::ZdpResponse`] aliases in public
/// API signatures.
#[must_use = "futures do nothing unless polled"]
#[derive(Debug)]
pub struct CommunicationResponse<T, U> {
    internal: ApsProtocolResponse<T>,
    target_type: PhantomData<U>,
}

impl<T, U> From<ApsProtocolResponse<T>> for CommunicationResponse<T, U> {
    fn from(internal: ApsProtocolResponse<T>) -> Self {
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
