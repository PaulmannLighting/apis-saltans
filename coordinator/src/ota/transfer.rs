use tokio::sync::mpsc::WeakSender;
use zb_core::destination::Device;

use super::{Message, UpdateError};

/// Identifies one generation of the update scheduled for an endpoint.
///
/// The generation prevents a late background result from completing a newer update for the same
/// endpoint.
#[derive(Clone, Copy, Debug)]
pub(super) struct TransferKey {
    pub(super) destination: Device,
    pub(super) generation: u64,
}

/// Report a background transfer failure if the OTA server is still running.
pub(super) async fn report_failure(
    messages: &WeakSender<Message>,
    key: TransferKey,
    error: UpdateError,
) {
    let Some(messages) = messages.upgrade() else {
        return;
    };
    messages
        .send(Message::TransferFailed {
            target: key.destination,
            generation: key.generation,
            error,
        })
        .await
        .unwrap_or_else(drop);
}
