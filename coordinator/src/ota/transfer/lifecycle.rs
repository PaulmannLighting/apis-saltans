use std::time::Duration;

use tokio::sync::mpsc::WeakSender;
use tokio::sync::oneshot;
use tokio::task::AbortHandle;
use tokio::time::sleep;

use super::{Deadline, TransferMessage, UpdateTimeouts};

const INITIAL_GENERATION: u64 = 0;
const GENERATION_STEP: u64 = 1;

/// Owns cancellation and deadlines for the current offer generation.
pub(super) struct Lifecycle {
    sender: WeakSender<TransferMessage>,
    timeouts: UpdateTimeouts,
    cancellation: Option<oneshot::Receiver<()>>,
    cancellation_task: Option<AbortHandle>,
    discovery_deadline_task: Option<AbortHandle>,
    inactivity_deadline_task: Option<AbortHandle>,
    total_deadline_task: Option<AbortHandle>,
    generation: u64,
}

impl Lifecycle {
    /// Prepare a generation whose timers start when the transfer starts running.
    pub(super) const fn new(
        sender: WeakSender<TransferMessage>,
        timeouts: UpdateTimeouts,
        cancellation: oneshot::Receiver<()>,
    ) -> Self {
        Self {
            sender,
            timeouts,
            cancellation: Some(cancellation),
            cancellation_task: None,
            discovery_deadline_task: None,
            inactivity_deadline_task: None,
            total_deadline_task: None,
            generation: INITIAL_GENERATION,
        }
    }

    /// Identity used to reject queued lifecycle messages and operations from previous offers.
    pub(super) const fn generation(&self) -> u64 {
        self.generation
    }

    /// Stop previous timers and start a fresh generation with its own cancellation signal.
    pub(super) fn replace(
        &mut self,
        timeouts: UpdateTimeouts,
        cancellation: oneshot::Receiver<()>,
    ) {
        self.abort();
        self.generation = self.generation.wrapping_add(GENERATION_STEP);
        self.timeouts = timeouts;
        self.cancellation = Some(cancellation);
        self.start();
    }

    /// Start cancellation, discovery, and total-transfer lifecycle tasks for this generation.
    pub(super) fn start(&mut self) {
        let generation = self.generation;
        let cancellation = self
            .cancellation
            .take()
            .expect("a new OTA generation retains its cancellation receiver");
        self.cancellation_task = Some(spawn_cancellation(
            self.sender.clone(),
            generation,
            cancellation,
        ));
        self.discovery_deadline_task = Some(spawn_deadline(
            self.sender.clone(),
            generation,
            Deadline::Discovery,
            self.timeouts.discovery(),
        ));
        self.total_deadline_task = Some(spawn_deadline(
            self.sender.clone(),
            generation,
            Deadline::TotalTransfer,
            self.timeouts.total_transfer(),
        ));
    }

    /// Move from discovery into transfer activity and reset the inactivity deadline.
    pub(super) fn record_activity(&mut self) {
        abort_task(&mut self.discovery_deadline_task);
        abort_task(&mut self.inactivity_deadline_task);
        self.inactivity_deadline_task = Some(spawn_deadline(
            self.sender.clone(),
            self.generation,
            Deadline::BlockInactivity,
            self.timeouts.block_inactivity(),
        ));
    }

    /// Abort every lifecycle task owned by the current generation.
    pub(super) fn abort(&mut self) {
        abort_task(&mut self.cancellation_task);
        abort_task(&mut self.discovery_deadline_task);
        abort_task(&mut self.inactivity_deadline_task);
        abort_task(&mut self.total_deadline_task);
    }
}

impl Drop for Lifecycle {
    fn drop(&mut self) {
        self.abort();
    }
}

fn spawn_cancellation(
    sender: WeakSender<TransferMessage>,
    generation: u64,
    cancellation: oneshot::Receiver<()>,
) -> AbortHandle {
    let task = crate::runtime::spawn(async move {
        let _result = cancellation.await;
        let Some(sender) = sender.upgrade() else {
            return;
        };
        let _result = sender.send(TransferMessage::Cancel { generation }).await;
    });
    task.abort_handle()
}

fn spawn_deadline(
    sender: WeakSender<TransferMessage>,
    generation: u64,
    deadline: Deadline,
    duration: Duration,
) -> AbortHandle {
    let task = crate::runtime::spawn(async move {
        sleep(duration).await;
        let Some(sender) = sender.upgrade() else {
            return;
        };
        let _result = sender
            .send(TransferMessage::Deadline {
                generation,
                deadline,
            })
            .await;
    });
    task.abort_handle()
}

fn abort_task(task: &mut Option<AbortHandle>) {
    if let Some(task) = task.take() {
        task.abort();
    }
}
