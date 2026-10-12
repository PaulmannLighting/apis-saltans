//! Shared delivery of protocol lifecycle messages through actor inboxes.

use std::time::Duration;

use log::debug;
use tokio::runtime::Handle;
use tokio::sync::mpsc::WeakSender;
use tokio::sync::mpsc::error::TrySendError;
use tokio::time::sleep;

use super::{Cancellation, Token};
use crate::runtime::spawn;

#[cfg(test)]
mod tests;

/// Actor messages that can represent cancellation of a protocol transaction.
pub trait CancellationMessage
where
    Self: Sized + Send + 'static,
    Self::Key: Copy + Send + 'static,
{
    /// Protocol-specific identity carried by lifecycle messages.
    type Key;

    /// Construct a cancellation message carrying the original transaction token.
    fn from_cancellation(token: Token<Self::Key>) -> Self;

    /// Create a cancellation guard that queues its token when dropped.
    ///
    /// A full inbox defers delivery to the current runtime. The guard retains only a weak
    /// sender until cancellation, so an outstanding response cannot keep its actor alive.
    fn cancellation(
        inbox: WeakSender<Self>,
        token: Token<Self::Key>,
        protocol: &'static str,
    ) -> Cancellation {
        let runtime = Handle::current();
        Cancellation::new(token, move |token| {
            let Some(inbox) = inbox.upgrade() else {
                return;
            };
            match inbox.try_send(Self::from_cancellation(token)) {
                Ok(()) => {}
                Err(TrySendError::Full(message)) => {
                    runtime.spawn(async move {
                        inbox.send(message).await.unwrap_or_else(|error| {
                            debug!("Failed to enqueue {protocol} response cancellation: {error}");
                        });
                    });
                }
                Err(TrySendError::Closed(_)) => {
                    debug!("Failed to enqueue {protocol} response cancellation: actor unavailable");
                }
            }
        })
    }
}

/// Queue a lifecycle message after its deadline without retaining the actor while waiting.
pub fn schedule_timeout<M>(
    inbox: WeakSender<M>,
    duration: Duration,
    message: M,
    description: &'static str,
) where
    M: Send + 'static,
{
    spawn(async move {
        sleep(duration).await;
        let Some(inbox) = inbox.upgrade() else {
            return;
        };
        inbox.send(message).await.unwrap_or_else(|error| {
            debug!("Failed to enqueue {description}: {error}");
        });
    });
}
