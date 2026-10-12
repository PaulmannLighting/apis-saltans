//! Ownership and supervision of coordinator tasks.

use tokio::sync::watch::{Receiver, Sender, channel};
use tokio::task::{AbortHandle, JoinHandle, JoinSet};
use tokio_util::sync::CancellationToken;
use tokio_util::task::TaskTracker;

use crate::event::{DroppedEvents, EventSink};

tokio::task_local! {
    static TASKS: TaskGroup;
}

/// Why a managed coordinator stopped.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeExit {
    /// Shutdown was requested explicitly or by dropping the runtime handle.
    Shutdown,

    /// The hardware event source closed.
    HardwareEventStreamClosed,

    /// A protocol actor ended unexpectedly.
    ActorStopped(&'static str),

    /// A coordinator task panicked.
    TaskPanicked,
}

/// Observable coordinator lifecycle state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeStatus {
    /// Actors are running.
    Running,

    /// Shutdown has started and owned tasks are being cancelled.
    Stopping(RuntimeExit),

    /// All tracked tasks have finished dropping their resources.
    Stopped(RuntimeExit),
}

/// Owns a coordinator runtime independently of its cloneable command handles.
///
/// Dropping this handle requests shutdown. Use `shutdown().await` to also wait for cleanup,
/// or `wait().await` to observe termination. Pending asynchronous image reads are cancelled
/// during cleanup. The externally owned hardware driver is not stopped.
#[derive(Debug)]
pub struct CoordinatorRuntime {
    tasks: TaskGroup,
    status: Receiver<RuntimeStatus>,
    events: EventSink,
    stop_on_drop: bool,
}

#[derive(Clone, Debug)]
struct TaskGroup {
    tracker: TaskTracker,
    cancellation: CancellationToken,
    stop: Sender<Option<RuntimeExit>>,
}

/// Report unwinding separately from task lifetime tracking.
struct PanicGuard(TaskGroup);

impl CoordinatorRuntime {
    /// Create a task group and start its lifecycle supervisor.
    pub(crate) fn new(events: EventSink) -> Self {
        let tasks = TaskGroup::new();
        let (status, receiver) = channel(RuntimeStatus::Running);
        tokio::spawn(tasks.clone().supervise(status));
        Self {
            tasks,
            status: receiver,
            events,
            stop_on_drop: true,
        }
    }

    pub(crate) fn enter<F, T>(&self, start: F) -> T
    where
        F: FnOnce() -> T,
    {
        TASKS.sync_scope(self.tasks.clone(), start)
    }

    /// Continue running without an owning runtime handle, as with legacy startup.
    pub fn detach(mut self) {
        self.stop_on_drop = false;
    }

    /// Read the latest lifecycle state without waiting.
    #[must_use]
    pub fn status(&self) -> RuntimeStatus {
        *self.status.borrow()
    }

    /// Subscribe to lifecycle changes independently of application event delivery.
    #[must_use]
    pub fn subscribe(&self) -> Receiver<RuntimeStatus> {
        self.status.clone()
    }

    /// Snapshot events lost because the application channel was full or closed.
    #[must_use]
    pub fn dropped_events(&self) -> DroppedEvents {
        self.events.dropped_events()
    }

    /// Cancel coordinator work and wait for all owned tasks to finish cleanup.
    pub async fn shutdown(self) -> RuntimeExit {
        self.tasks.request_stop(RuntimeExit::Shutdown);
        self.wait().await
    }

    /// Wait for termination, including cancellation and cleanup of child tasks.
    pub async fn wait(mut self) -> RuntimeExit {
        loop {
            let status = *self.status.borrow_and_update();
            if let RuntimeStatus::Stopped(reason) = status {
                return reason;
            }
            if self.status.changed().await.is_err() {
                return RuntimeExit::TaskPanicked;
            }
        }
    }
}

impl Drop for CoordinatorRuntime {
    fn drop(&mut self) {
        if self.stop_on_drop {
            self.tasks.request_stop(RuntimeExit::Shutdown);
        }
    }
}

impl TaskGroup {
    /// Create task tracking, cancellation, and termination notification state.
    fn new() -> Self {
        Self {
            tracker: TaskTracker::new(),
            cancellation: CancellationToken::new(),
            stop: Sender::new(None),
        }
    }

    /// Publish lifecycle transitions and await task cleanup after a stop request.
    async fn supervise(self, status: Sender<RuntimeStatus>) {
        let reason = self.wait_for_stop().await;
        status.send_replace(RuntimeStatus::Stopping(reason));
        self.cancellation.cancel();
        self.tracker.close();
        self.tracker.wait().await;
        status.send_replace(RuntimeStatus::Stopped(reason));
    }

    /// Observe the first stop reason, including requests made before supervision starts.
    async fn wait_for_stop(&self) -> RuntimeExit {
        let mut requested = self.stop.subscribe();
        loop {
            let reason = *requested.borrow_and_update();
            if let Some(reason) = reason {
                return reason;
            }
            if requested.changed().await.is_err() {
                return RuntimeExit::Shutdown;
            }
        }
    }

    /// Retain the first termination reason and notify the supervisor.
    fn request_stop(&self, reason: RuntimeExit) {
        self.stop.send_if_modified(|current| {
            if current.is_some() {
                false
            } else {
                *current = Some(reason);
                true
            }
        });
    }

    /// Register synchronously, before spawning, so shutdown includes unpolled tasks.
    fn track<F>(&self, future: F) -> impl Future<Output = Option<F::Output>> + Send + 'static
    where
        F: Future + Send + 'static,
        F::Output: Send + 'static,
    {
        let scope = self.clone();
        let cancellation = self.cancellation.clone();
        let guard = PanicGuard(self.clone());
        self.tracker.track_future(async move {
            let _guard = guard;
            TASKS
                .scope(scope, cancellation.run_until_cancelled(future))
                .await
        })
    }
}

impl Drop for PanicGuard {
    fn drop(&mut self) {
        if std::thread::panicking() {
            self.0.request_stop(RuntimeExit::TaskPanicked);
        }
    }
}

/// Spawn a child inheriting the current coordinator's task ownership.
///
/// The task returns `None` when coordinator shutdown cancels its future.
pub fn spawn<F>(future: F) -> JoinHandle<Option<F::Output>>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let Ok(group) = TASKS.try_with(Clone::clone) else {
        return tokio::spawn(async move { Some(future.await) });
    };
    tokio::spawn(group.track(future))
}

/// Spawn a main actor, terminating its coordinator if the actor exits unexpectedly.
pub fn spawn_actor<F>(name: &'static str, future: F) -> JoinHandle<Option<()>>
where
    F: Future<Output = ()> + Send + 'static,
{
    spawn(async move {
        future.await;
        request_stop(RuntimeExit::ActorStopped(name));
    })
}

/// Request termination of the coordinator owning the current task.
pub fn request_stop(reason: RuntimeExit) {
    let _result = TASKS.try_with(|group| group.request_stop(reason));
}

/// Track tasks whose results are collected by an actor-owned `JoinSet`.
pub fn spawn_in<F>(set: &mut JoinSet<Option<F::Output>>, future: F) -> AbortHandle
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let Ok(group) = TASKS.try_with(Clone::clone) else {
        return set.spawn(async move { Some(future.await) });
    };
    set.spawn(group.track(future))
}

#[cfg(test)]
mod tests {
    use std::future::pending;
    use std::time::Duration;

    use tokio::runtime::Runtime;
    use tokio::sync::{mpsc, oneshot};
    use tokio::task::JoinSet;
    use tokio::time::timeout;

    use super::{CoordinatorRuntime, RuntimeExit, RuntimeStatus, spawn, spawn_actor, spawn_in};
    use crate::event::EventSink;
    use crate::{Event, Network};

    const CAPACITY: usize = 1;
    const DEADLINE: Duration = Duration::from_secs(2);

    #[test]
    fn shutdown_cancels_nested_tasks_and_closes_their_resources() {
        Runtime::new().unwrap().block_on(async {
            let (events, _received) = mpsc::channel(CAPACITY);
            let runtime = CoordinatorRuntime::new(EventSink::new(events));
            let (ready, started) = oneshot::channel();
            let (resource, mut receiver) = mpsc::channel::<()>(CAPACITY);
            runtime.enter(|| {
                spawn(async move {
                    let mut children = JoinSet::new();
                    spawn_in(&mut children, async move {
                        let _resource = resource;
                        pending::<()>().await;
                    });
                    ready.send(()).unwrap();
                    pending::<()>().await;
                })
            });
            started.await.unwrap();
            let mut status = runtime.subscribe();
            assert_eq!(
                timeout(DEADLINE, runtime.shutdown()).await.unwrap(),
                RuntimeExit::Shutdown
            );
            assert_eq!(
                *status.borrow_and_update(),
                RuntimeStatus::Stopped(RuntimeExit::Shutdown)
            );
            assert_eq!(receiver.recv().await, None);
        });
    }

    #[test]
    fn task_panic_stops_the_runtime() {
        Runtime::new().unwrap().block_on(async {
            let (events, _received) = mpsc::channel(CAPACITY);
            let runtime = CoordinatorRuntime::new(EventSink::new(events));
            runtime.enter(|| {
                spawn(async {
                    panic!("test task failure");
                })
            });
            assert_eq!(
                timeout(DEADLINE, runtime.wait()).await.unwrap(),
                RuntimeExit::TaskPanicked
            );
        });
    }

    #[test]
    fn actor_exit_is_observable_even_when_events_are_full() {
        Runtime::new().unwrap().block_on(async {
            let (events, _received) = mpsc::channel(CAPACITY);
            let sink = EventSink::new(events);
            sink.emit(Event::Network(Network::Up));
            sink.clone().emit(Event::Network(Network::Down));
            let runtime = CoordinatorRuntime::new(sink);
            assert_eq!(runtime.dropped_events().full, 1);
            runtime.enter(|| spawn_actor("test", async {}));
            assert_eq!(
                timeout(DEADLINE, runtime.wait()).await.unwrap(),
                RuntimeExit::ActorStopped("test")
            );
        });
    }

    #[test]
    fn dropping_the_handle_requests_shutdown() {
        Runtime::new().unwrap().block_on(async {
            let (events, received) = mpsc::channel(CAPACITY);
            let sink = EventSink::new(events);
            drop(received);
            sink.emit(Event::Network(Network::Up));
            let runtime = CoordinatorRuntime::new(sink);
            assert_eq!(runtime.dropped_events().closed, 1);
            let mut status = runtime.subscribe();
            drop(runtime);
            timeout(DEADLINE, async {
                loop {
                    if matches!(
                        *status.borrow_and_update(),
                        RuntimeStatus::Stopped(RuntimeExit::Shutdown)
                    ) {
                        break;
                    }
                    status.changed().await.unwrap();
                }
            })
            .await
            .unwrap();
        });
    }

    struct SpawnDuringCleanup {
        resource: Option<oneshot::Sender<()>>,
    }

    impl Drop for SpawnDuringCleanup {
        fn drop(&mut self) {
            let resource = self.resource.take().unwrap();
            spawn(async move {
                let _resource = resource;
                panic!("a task created during shutdown must not be polled");
            });
        }
    }

    #[test]
    fn shutdown_tracks_children_spawned_during_cleanup() {
        Runtime::new().unwrap().block_on(async {
            let (events, _received) = mpsc::channel(CAPACITY);
            let runtime = CoordinatorRuntime::new(EventSink::new(events));
            let (resource, mut receiver) = oneshot::channel::<()>();
            let (ready, started) = oneshot::channel();
            runtime.enter(|| {
                spawn(async move {
                    let _cleanup = SpawnDuringCleanup {
                        resource: Some(resource),
                    };
                    ready.send(()).unwrap();
                    pending::<()>().await;
                })
            });
            started.await.unwrap();
            assert_eq!(
                timeout(DEADLINE, runtime.shutdown()).await.unwrap(),
                RuntimeExit::Shutdown
            );
            assert!(matches!(
                receiver.try_recv(),
                Err(oneshot::error::TryRecvError::Closed)
            ));
        });
    }

    #[test]
    fn cancellation_before_first_poll_releases_captured_resources() {
        Runtime::new().unwrap().block_on(async {
            let (events, _received) = mpsc::channel(CAPACITY);
            let runtime = CoordinatorRuntime::new(EventSink::new(events));
            runtime.tasks.cancellation.cancel();
            let (resource, mut receiver) = oneshot::channel::<()>();
            let task = runtime.enter(|| {
                spawn(async move {
                    let _resource = resource;
                    panic!("an already cancelled task must not be polled");
                })
            });
            assert_eq!(
                timeout(DEADLINE, runtime.shutdown()).await.unwrap(),
                RuntimeExit::Shutdown
            );
            assert!(matches!(
                receiver.try_recv(),
                Err(oneshot::error::TryRecvError::Closed)
            ));
            assert_eq!(task.await.unwrap(), None);
        });
    }

    #[test]
    fn tracked_tasks_preserve_values_and_explicit_abort() {
        Runtime::new().unwrap().block_on(async {
            let (events, _received) = mpsc::channel(CAPACITY);
            let runtime = CoordinatorRuntime::new(EventSink::new(events));
            let task = runtime.enter(|| spawn(async { RuntimeExit::HardwareEventStreamClosed }));
            assert_eq!(
                task.await.unwrap(),
                Some(RuntimeExit::HardwareEventStreamClosed)
            );
            let task = runtime.enter(|| spawn(pending::<()>()));
            task.abort();
            assert!(task.await.unwrap_err().is_cancelled());
            assert_eq!(runtime.status(), RuntimeStatus::Running);
            assert_eq!(
                timeout(DEADLINE, runtime.shutdown()).await.unwrap(),
                RuntimeExit::Shutdown
            );
        });
    }
}
