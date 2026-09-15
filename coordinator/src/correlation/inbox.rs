//! Shared delivery of protocol lifecycle messages through actor inboxes.

use std::time::Duration;

use log::debug;
use tokio::runtime::Handle;
use tokio::spawn;
use tokio::sync::mpsc::WeakSender;
use tokio::sync::mpsc::error::TrySendError;
use tokio::time::sleep;

use super::{Cancellation, Token};

#[cfg(test)]
mod tests;

/// Create a cancellation guard that queues its token when dropped.
///
/// A full inbox defers delivery to the current runtime. The guard retains only a weak
/// sender until cancellation, so an outstanding response cannot keep its actor alive.
pub fn cancellation<M>(
    inbox: WeakSender<M>,
    token: Token,
    message: fn(Token) -> M,
    protocol: &'static str,
) -> Cancellation
where
    M: Send + 'static,
{
    let runtime = Handle::current();
    Cancellation::new(token, move |token| {
        let Some(inbox) = inbox.upgrade() else {
            return;
        };
        match inbox.try_send(message(token)) {
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
