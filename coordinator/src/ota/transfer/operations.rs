use std::collections::HashMap;
use std::task::{Context, Poll};

use log::warn;
use tokio::task::{Id, JoinError, JoinSet};

use super::{UpdateError, UpdateResult};

/// Raw task completion retained until its originating generation is checked.
pub(super) type OperationCompletion = Option<Result<(Id, Option<OperationResult>), JoinError>>;

/// Retains task identities until their results can be matched to the originating offer.
#[derive(Default)]
pub(super) struct Operations {
    tasks: JoinSet<Option<OperationResult>>,
    generations: HashMap<Id, u64>,
}

/// Effect of a completed reply on the active update.
pub(super) enum OperationOutcome {
    /// The update remains active.
    Continue,

    /// A successful or failed terminal update result.
    Complete(UpdateResult),
}

/// Reply outcome tagged with the offer that initiated it.
pub(super) struct OperationResult {
    generation: u64,
    outcome: OperationOutcome,
}

impl Operations {
    /// Whether any operation still needs to be collected.
    pub(super) fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Poll a completion without changing its generation attribution.
    pub(super) fn poll_finished(&mut self, context: &mut Context<'_>) -> Poll<OperationCompletion> {
        self.tasks.poll_join_next_with_id(context)
    }

    /// Abort work while retaining generation attribution until each completion is collected.
    pub(super) fn abort(&mut self) {
        self.tasks.abort_all();
    }

    /// Spawn a generation-tagged operation owned by this destination transfer.
    pub(super) fn spawn<T>(&mut self, generation: u64, operation: T)
    where
        T: Future<Output = OperationOutcome> + Send + 'static,
    {
        let task = crate::runtime::spawn_in(&mut self.tasks, async move {
            OperationResult {
                generation,
                outcome: operation.await,
            }
        });
        self.generations.insert(task.id(), generation);
    }

    /// Retire completed operations, ignoring outcomes from replaced update generations.
    pub(super) fn finish(
        &mut self,
        operation: OperationCompletion,
        generation: u64,
    ) -> Option<UpdateResult> {
        match operation {
            Some(Ok((task_id, operation))) => {
                self.generations.remove(&task_id);
                let operation = operation?;
                if operation.generation != generation {
                    return None;
                }
                match operation.outcome {
                    OperationOutcome::Continue => None,
                    OperationOutcome::Complete(result) => Some(result),
                }
            }
            Some(Err(error)) if error.is_cancelled() => {
                self.generations.remove(&error.id());
                None
            }
            Some(Err(error)) => {
                let operation_generation = self.generations.remove(&error.id());
                if operation_generation != Some(generation) {
                    return None;
                }
                warn!("OTA transfer operation failed: {error}");
                Some(Err(UpdateError::TransferTask))
            }
            None => None,
        }
    }
}
