use std::collections::HashMap;
use std::task::{Context, Poll};

use log::warn;
use tokio::task::{Id, JoinError, JoinSet};

use super::{UpdateError, UpdateResult};

#[cfg(test)]
mod tests;

/// Raw task result, including cancellation of the tracked future.
type OperationCompletion = Result<(Id, Option<OperationResult>), JoinError>;

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
struct OperationResult {
    generation: u64,
    outcome: OperationOutcome,
}

impl Operations {
    /// Normalize task completion into its effect on the current offer.
    ///
    /// An empty operation set stays pending so the transfer can await its inbox.
    pub(super) fn poll_finished(
        &mut self,
        context: &mut Context<'_>,
        generation: u64,
    ) -> Poll<OperationOutcome> {
        match self.tasks.poll_join_next_with_id(context) {
            Poll::Ready(Some(completion)) => Poll::Ready(
                self.finish(completion, generation)
                    .map_or(OperationOutcome::Continue, OperationOutcome::Complete),
            ),
            Poll::Ready(None) | Poll::Pending => Poll::Pending,
        }
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
    fn finish(&mut self, operation: OperationCompletion, generation: u64) -> Option<UpdateResult> {
        match operation {
            Ok((task_id, operation)) => {
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
            Err(error) if error.is_cancelled() => {
                self.generations.remove(&error.id());
                None
            }
            Err(error) => {
                let operation_generation = self.generations.remove(&error.id());
                if operation_generation != Some(generation) {
                    return None;
                }
                warn!("OTA transfer operation failed: {error}");
                Some(Err(UpdateError::TransferTask))
            }
        }
    }
}
