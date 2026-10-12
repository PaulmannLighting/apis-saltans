//! Admission and task ownership for inbound ZDP server operations.

use std::collections::BTreeMap;
use std::mem::take;

use log::{debug, warn};
use tokio::sync::mpsc::WeakSender;
use tokio::task::AbortHandle;

use super::Message;
use super::server::{Server, ServerRequest};
use crate::runtime::spawn;

const INITIAL_ID: u64 = 0;
const ID_STEP: u64 = 1;

/// Bounded operations whose completions return through the ZDP actor inbox.
#[derive(Debug)]
pub(super) struct ServerOperations {
    tasks: BTreeMap<u64, AbortHandle>,
    next_id: u64,
    limit: usize,
}

impl ServerOperations {
    /// Create empty operation tracking with an explicit admission limit.
    pub(super) const fn new(limit: usize) -> Self {
        Self {
            tasks: BTreeMap::new(),
            next_id: INITIAL_ID,
            limit,
        }
    }

    /// Change the admission limit without cancelling existing work.
    pub(super) const fn set_limit(&mut self, limit: usize) {
        self.limit = limit;
    }

    /// Retire a completed operation; late completions after shutdown are harmless.
    pub(super) fn finish(&mut self, id: u64) {
        if self.tasks.remove(&id).is_none() {
            debug!("Ignoring completion for unknown ZDP server operation {id}");
        }
    }

    /// Whether there are no operations awaiting retirement.
    #[cfg(test)]
    pub(super) fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Admit and spawn one request, logging and dropping it when the limit is reached.
    pub(super) fn spawn(
        &mut self,
        server: &Server,
        inbox: WeakSender<Message>,
        request: ServerRequest,
    ) {
        if self.tasks.len() >= self.limit {
            warn!(
                "Discarding ZDP server request because the operation limit of \
                 {} has been reached",
                self.limit
            );
            return;
        }

        let id = self.allocate_id();
        let server = server.clone();
        let task = spawn(async move {
            server.handle(request).await;
            let Some(inbox) = inbox.upgrade() else {
                return;
            };
            inbox
                .send(Message::ServerOperationFinished { id })
                .await
                .unwrap_or_else(|error| {
                    debug!("Failed to retire ZDP server operation: {error}");
                });
        });
        let previous = self.tasks.insert(id, task.abort_handle());
        debug_assert!(previous.is_none());
    }

    /// Abort and forget all operations while retaining the next operation identity.
    pub(super) fn abort(&mut self) {
        for operation in take(&mut self.tasks).into_values() {
            operation.abort();
        }
    }

    fn allocate_id(&mut self) -> u64 {
        loop {
            let id = self.next_id;
            self.next_id = self.next_id.wrapping_add(ID_STEP);
            if !self.tasks.contains_key(&id) {
                return id;
            }
        }
    }
}

impl Drop for ServerOperations {
    fn drop(&mut self) {
        self.abort();
    }
}
