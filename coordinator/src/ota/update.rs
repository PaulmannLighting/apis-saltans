use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use zb_aps::apsde::IndividualEndpoint;
use zb_core::FullAddress;

use super::{Image, Message, UpdateResult, UpdateTimeouts};
use crate::Error;

/// Cancellation behavior exposed by a coordinator-managed OTA update future.
pub trait CancellableOtaUpdate {
    /// Cancel the update and release its transfer resources.
    ///
    /// Dropping the update future has the same effect.
    fn cancel(self);
}

/// Cancellation-aware future for one coordinator-managed OTA update.
#[must_use = "dropping this future cancels the OTA update"]
pub struct Update {
    future: Pin<Box<dyn Future<Output = Result<(), Error>> + Send>>,
    cancellation: Option<oneshot::Sender<()>>,
}

impl Update {
    /// Create an update future that submits one offer when first polled.
    pub(crate) fn new(
        sender: Sender<Message>,
        target: FullAddress,
        target_endpoint: IndividualEndpoint,
        source_endpoint: IndividualEndpoint,
        image: Image,
        timeouts: UpdateTimeouts,
    ) -> Self {
        let (completion, result) = oneshot::channel::<UpdateResult>();
        let (cancellation, cancelled) = oneshot::channel();
        let future = Box::pin(async move {
            sender
                .send(Message::Update {
                    target,
                    target_endpoint,
                    source_endpoint,
                    image,
                    timeouts,
                    cancellation: cancelled,
                    completion,
                })
                .await?;
            result.await??;
            Ok(())
        });

        Self {
            future,
            cancellation: Some(cancellation),
        }
    }
}

impl CancellableOtaUpdate for Update {
    fn cancel(mut self) {
        if let Some(cancellation) = self.cancellation.take() {
            let _result = cancellation.send(());
        }
    }
}

impl Future for Update {
    type Output = Result<(), Error>;

    fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<Self::Output> {
        self.future.as_mut().poll(context)
    }
}
