use std::future::{pending, poll_fn};
use std::task::Poll;
use std::time::Duration;

use tokio::runtime::Builder;
use tokio::time::timeout;

use super::{OperationOutcome, Operations, UpdateError};

const PREVIOUS_GENERATION: u64 = 0;
const CURRENT_GENERATION: u64 = 1;
const DEADLINE: Duration = Duration::from_secs(1);

#[test]
fn stale_successes_and_panics_do_not_finish_the_replacement() {
    Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap()
        .block_on(async {
            for panic in [false, true] {
                let mut operations = Operations::default();
                operations.spawn(PREVIOUS_GENERATION, async move {
                    assert!(!panic, "previous offer task panicked");
                    OperationOutcome::Complete(Err(UpdateError::Aborted))
                });
                let outcome = timeout(
                    DEADLINE,
                    poll_fn(|context| operations.poll_finished(context, CURRENT_GENERATION)),
                )
                .await
                .unwrap();
                assert!(matches!(outcome, OperationOutcome::Continue));
                assert!(operations.generations.is_empty());

                operations.spawn(CURRENT_GENERATION, async {
                    OperationOutcome::Complete(Ok(()))
                });
                let outcome = timeout(
                    DEADLINE,
                    poll_fn(|context| operations.poll_finished(context, CURRENT_GENERATION)),
                )
                .await
                .unwrap();
                assert!(matches!(outcome, OperationOutcome::Complete(Ok(()))));
            }
        });
}

#[test]
fn aborted_operations_retire_without_ending_the_update() {
    Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap()
        .block_on(async {
            let mut operations = Operations::default();
            operations.spawn(PREVIOUS_GENERATION, pending());
            operations.abort();
            let outcome = timeout(
                DEADLINE,
                poll_fn(|context| operations.poll_finished(context, CURRENT_GENERATION)),
            )
            .await
            .unwrap();
            assert!(matches!(outcome, OperationOutcome::Continue));
            assert!(operations.generations.is_empty());
            let empty = poll_fn(|context| {
                Poll::Ready(operations.poll_finished(context, CURRENT_GENERATION))
            })
            .await;
            assert!(empty.is_pending());
        });
}

#[test]
fn a_current_operation_panic_fails_the_update() {
    Builder::new_current_thread()
        .enable_time()
        .build()
        .unwrap()
        .block_on(async {
            let mut operations = Operations::default();
            operations.spawn(CURRENT_GENERATION, async {
                panic!("current offer task panicked")
            });
            let outcome = timeout(
                DEADLINE,
                poll_fn(|context| operations.poll_finished(context, CURRENT_GENERATION)),
            )
            .await
            .unwrap();
            assert!(matches!(
                outcome,
                OperationOutcome::Complete(Err(UpdateError::TransferTask))
            ));
            assert!(operations.generations.is_empty());
        });
}
