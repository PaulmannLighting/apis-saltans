//! Ownership and supervision of coordinator tasks.

use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use tokio::sync::{Notify, watch};
use tokio::task::{AbortHandle, JoinHandle, JoinSet};

use crate::event::{DroppedEvents, EventSink};

const TASK_ID_STEP: u64 = 1;

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
/// or `wait().await` to observe termination. Already-running blocking image reads must finish
/// before cleanup completes. The externally owned hardware driver is not stopped.
#[derive(Debug)]
pub struct CoordinatorRuntime {
    tasks: TaskGroup,
    status: watch::Receiver<RuntimeStatus>,
    events: EventSink,
    stop_on_drop: bool,
}

#[derive(Clone, Debug)]
struct TaskGroup(Arc<GroupState>);

#[derive(Debug)]
struct GroupState {
    registry: Mutex<Registry>,
    changed: Notify,
    stop: watch::Sender<Option<RuntimeExit>>,
}

#[derive(Debug, Default)]
struct Registry {
    next: u64,
    closed: bool,
    tasks: BTreeMap<u64, Option<AbortHandle>>,
}

struct Registration {
    group: TaskGroup,
    id: u64,
}

impl CoordinatorRuntime {
    pub(crate) fn new(events: EventSink) -> Self {
        let (stop, mut requested) = watch::channel(None);
        let tasks = TaskGroup(Arc::new(GroupState {
            registry: Mutex::new(Registry::default()),
            changed: Notify::new(),
            stop,
        }));
        let (status, receiver) = watch::channel(RuntimeStatus::Running);
        let group = tasks.clone();
        tokio::spawn(async move {
            let reason = loop {
                let reason = *requested.borrow_and_update();
                if let Some(reason) = reason {
                    break reason;
                }
                if requested.changed().await.is_err() {
                    break RuntimeExit::Shutdown;
                }
            };
            status.send_replace(RuntimeStatus::Stopping(reason));
            group.close();
            group.join().await;
            status.send_replace(RuntimeStatus::Stopped(reason));
        });
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
    pub fn subscribe(&self) -> watch::Receiver<RuntimeStatus> {
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
    fn register(&self) -> Registration {
        let mut registry = self.0.registry.lock().expect("task registry lock");
        let id = registry.next;
        registry.next = registry.next.wrapping_add(TASK_ID_STEP);
        assert!(
            registry.tasks.insert(id, None).is_none(),
            "task ID exhausted"
        );
        drop(registry);
        Registration {
            group: self.clone(),
            id,
        }
    }

    fn attach(&self, id: u64, handle: AbortHandle) {
        let mut registry = self.0.registry.lock().expect("task registry lock");
        if registry.closed {
            handle.abort();
        }
        if let Some(entry) = registry.tasks.get_mut(&id) {
            *entry = Some(handle);
        }
    }

    fn request_stop(&self, reason: RuntimeExit) {
        self.0.stop.send_if_modified(|current| {
            if current.is_some() {
                false
            } else {
                *current = Some(reason);
                true
            }
        });
    }

    fn close(&self) {
        let mut registry = self.0.registry.lock().expect("task registry lock");
        registry.closed = true;
        for task in registry.tasks.values().flatten() {
            task.abort();
        }
    }

    async fn join(&self) {
        loop {
            let changed = self.0.changed.notified();
            if self
                .0
                .registry
                .lock()
                .expect("task registry lock")
                .tasks
                .is_empty()
            {
                return;
            }
            changed.await;
        }
    }
}

impl Drop for Registration {
    fn drop(&mut self) {
        if std::thread::panicking() {
            self.group.request_stop(RuntimeExit::TaskPanicked);
        }
        self.group
            .0
            .registry
            .lock()
            .expect("task registry lock")
            .tasks
            .remove(&self.id);
        self.group.0.changed.notify_one();
    }
}

/// Spawn a child inheriting the current coordinator's task ownership.
pub fn spawn<F>(future: F) -> JoinHandle<F::Output>
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let Ok(group) = TASKS.try_with(Clone::clone) else {
        return tokio::spawn(future);
    };
    let registration = group.register();
    let id = registration.id;
    let scope = group.clone();
    let task = tokio::spawn(async move {
        let _registration = registration;
        TASKS.scope(scope, future).await
    });
    group.attach(id, task.abort_handle());
    task
}

/// Spawn a main actor, terminating its coordinator if the actor exits unexpectedly.
pub fn spawn_actor<F>(name: &'static str, future: F) -> JoinHandle<()>
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
pub fn spawn_in<F>(set: &mut JoinSet<F::Output>, future: F) -> AbortHandle
where
    F: Future + Send + 'static,
    F::Output: Send + 'static,
{
    let Ok(group) = TASKS.try_with(Clone::clone) else {
        return set.spawn(future);
    };
    let registration = group.register();
    let id = registration.id;
    let scope = group.clone();
    let task = set.spawn(async move {
        let _registration = registration;
        TASKS.scope(scope, future).await
    });
    group.attach(id, task.clone());
    task
}

/// Track blocking reads too; running reads finish naturally even during shutdown.
pub fn spawn_blocking<F, T>(operation: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    let Ok(group) = TASKS.try_with(Clone::clone) else {
        return tokio::task::spawn_blocking(operation);
    };
    let registration = group.register();
    let id = registration.id;
    let task = tokio::task::spawn_blocking(move || {
        let _registration = registration;
        operation()
    });
    group.attach(id, task.abort_handle());
    task
}

#[cfg(test)]
mod tests {
    use std::future::pending;
    use std::time::Duration;

    use tokio::runtime::Runtime;
    use tokio::sync::{mpsc, oneshot};
    use tokio::task::JoinSet;
    use tokio::time::timeout;

    use super::{
        CoordinatorRuntime, RuntimeExit, RuntimeStatus, spawn, spawn_actor, spawn_blocking,
        spawn_in,
    };
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
    fn shutdown_waits_for_a_running_blocking_read() {
        Runtime::new().unwrap().block_on(async {
            let (events, _received) = mpsc::channel(CAPACITY);
            let runtime = CoordinatorRuntime::new(EventSink::new(events));
            let (release, blocked) = std::sync::mpsc::channel();
            let (ready, started) = oneshot::channel();
            runtime.enter(|| {
                spawn_blocking(move || {
                    ready.send(()).unwrap();
                    blocked.recv_timeout(DEADLINE).unwrap();
                })
            });
            started.await.unwrap();
            let mut status = runtime.subscribe();
            let shutdown = tokio::spawn(runtime.shutdown());
            while matches!(*status.borrow_and_update(), RuntimeStatus::Running) {
                status.changed().await.unwrap();
            }
            assert!(!shutdown.is_finished());
            release.send(()).unwrap();
            assert_eq!(
                timeout(DEADLINE, shutdown).await.unwrap().unwrap(),
                RuntimeExit::Shutdown
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
}
