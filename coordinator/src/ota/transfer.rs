use tokio::sync::mpsc::UnboundedSender;
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

/// Reports a terminal background-transfer failure to the OTA server actor.
#[derive(Clone, Copy, Debug)]
pub(super) struct TransferEvent {
    pub(super) key: TransferKey,
    pub(super) error: UpdateError,
}

/// An input selected by the OTA server's combined message and transfer-event loop.
pub(super) enum ServerEvent {
    /// A message received from the server's bounded actor channel, or channel closure.
    Message(Option<Message>),
    /// A terminal failure reported by a background transfer task.
    Transfer(TransferEvent),
}

/// Report a background transfer failure if the OTA server is still running.
pub(super) fn report_failure(
    events: &UnboundedSender<TransferEvent>,
    key: TransferKey,
    error: UpdateError,
) {
    events
        .send(TransferEvent { key, error })
        .unwrap_or_else(drop);
}
