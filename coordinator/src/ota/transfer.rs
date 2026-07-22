use tokio::sync::mpsc::UnboundedSender;
use zb_core::destination::Device;

use super::{Message, UpdateError};

#[derive(Clone, Copy, Debug)]
pub(super) struct TransferKey {
    pub(super) destination: Device,
    pub(super) generation: u64,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct TransferEvent {
    pub(super) key: TransferKey,
    pub(super) error: UpdateError,
}

pub(super) enum ServerEvent {
    Message(Option<Message>),
    Transfer(TransferEvent),
}

pub(super) fn report_failure(
    events: &UnboundedSender<TransferEvent>,
    key: TransferKey,
    error: UpdateError,
) {
    events
        .send(TransferEvent { key, error })
        .unwrap_or_else(drop);
}
