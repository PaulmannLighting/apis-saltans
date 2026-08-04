//! State used while serving incoming ZDP requests.

use tokio::sync::mpsc::WeakSender;
use zb_aps::apsde::NetworkAddress;
use zb_core::node::Descriptor;
use zb_hw::NcpHandle;
use zb_zdp::Command;

use super::Message;
use crate::aps::Aps;

/// Cloneable context used by bounded background ZDP request-serving operations.
#[derive(Clone, Debug)]
pub(super) struct Server {
    pub(super) ncp: NcpHandle,
    pub(super) aps: Aps,
    pub(super) descriptor: Descriptor,
    pub(super) inbox: WeakSender<Message>,
}

/// A received ZDP request that may require asynchronous NCP or APS work.
#[derive(Debug)]
pub(super) struct ServerRequest {
    pub(super) source: NetworkAddress,
    pub(super) request_was_broadcast: bool,
    pub(super) sequence: u8,
    pub(super) command: Command,
}
