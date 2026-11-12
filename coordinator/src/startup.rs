//! Preparation and optional restoration before coordinator protocol actors start.

use tokio::sync::mpsc::{Receiver, Sender};
use zb_core::node::Descriptor;
use zb_hw::NcpHandle;

use crate::{
    Backup, BackupCapabilities, Coordinator, CoordinatorConfig, CoordinatorRuntime, Error, Event,
    HardwareError, HardwareOperation, NetworkBackup, Restore, RestoreOptions, RestoreReport,
};

/// Owns coordinator startup inputs before protocol actors are created.
///
/// This object is not cloneable. Starting consumes it, so restoration is exposed only before
/// protocol startup. `T` and `K` retain the hardware event timestamp and key-handle types; hardware
/// capabilities are checked at runtime. An unsuccessful or cancelled restore blocks startup until
/// a subsequent explicit restore succeeds. After cancellation, verify hardware state before retrying:
/// dropping the response future does not cancel an already queued hardware restore.
///
/// ```no_run
/// use apis_saltans_coordinator::{CoordinatorStartup, Error, NetworkBackup, Restore, RestoreOptions, RestoreReport};
///
/// async fn prepare(mut startup: CoordinatorStartup, backup: NetworkBackup, options: RestoreOptions)
///     -> Result<(CoordinatorStartup, RestoreReport), Error>
/// {
///     let report = startup.restore_network(backup, options).await?;
///     // Resume hardware via its backend, then consume startup to start protocol actors.
///     Ok((startup, report))
/// }
/// ```
///
/// The driver actor must already be running. The backend owns hardware network stop/resume; keep
/// the target network stopped for restoration, then resume it through backend startup before
/// calling [`Self::start_managed`] or [`Self::start`]. Other owners of cloned hardware handles must
/// not start a competing coordinator or mutate the target during restoration.
#[must_use = "start the coordinator or drop the preparation to release its inputs"]
#[derive(Debug)]
pub struct CoordinatorStartup<T = (), K = ()> {
    ncp: NcpHandle,
    descriptor: Descriptor,
    hw_events: Receiver<zb_hw::Event<T, K>>,
    events_out: Sender<Event>,
    config: CoordinatorConfig,
    restore_incomplete: bool,
}

impl<T, K> CoordinatorStartup<T, K>
where
    T: Send + 'static,
    K: Send + 'static,
{
    /// Retain startup inputs without starting any coordinator protocol actors.
    /// The external driver actor must be polled independently.
    pub const fn new(
        ncp: NcpHandle,
        descriptor: Descriptor,
        hw_events: Receiver<zb_hw::Event<T, K>>,
        events_out: Sender<Event>,
        config: CoordinatorConfig,
    ) -> Self {
        Self {
            ncp,
            descriptor,
            hw_events,
            events_out,
            config,
            restore_incomplete: false,
        }
    }

    /// Consume the prepared inputs and start a coordinator with an owning runtime handle.
    ///
    /// This starts protocol actors only; it does not resume the hardware network. After a restore,
    /// backend startup must resume the verified network deliberately before calling this method.
    ///
    /// # Errors
    /// Returns [`Error::RestoreIncomplete`] if a requested restore has not completed successfully,
    /// or a hardware startup error. No protocol actors are created when restoration is incomplete.
    pub fn start_managed(self) -> Result<(Coordinator, CoordinatorRuntime), Error> {
        if self.restore_incomplete {
            return Err(Error::RestoreIncomplete);
        }
        Ok(Coordinator::start_managed(
            self.ncp,
            self.descriptor,
            self.hw_events,
            self.events_out,
            self.config,
        )?)
    }

    /// Consume the prepared inputs and start a detached coordinator.
    /// Use [`Self::start_managed`] when explicit runtime ownership is needed.
    ///
    /// # Errors
    /// Returns the same errors as [`Self::start_managed`].
    pub fn start(self) -> Result<Coordinator, Error> {
        let (coordinator, runtime) = self.start_managed()?;
        runtime.detach();
        Ok(coordinator)
    }
}

impl<T, K> Backup for CoordinatorStartup<T, K>
where
    T: Send + 'static,
    K: Send + 'static,
{
    async fn backup_capabilities(&self) -> Result<BackupCapabilities, Error> {
        Ok(self.ncp.backup_capabilities().await?)
    }

    async fn dump_network(&self) -> Result<NetworkBackup, Error> {
        if !self.ncp.backup_capabilities().await?.can_dump() {
            return Err(HardwareError::Unsupported(HardwareOperation::DumpNetwork).into());
        }
        Ok(self.ncp.dump_network().await?)
    }
}

impl<T, K> Restore for CoordinatorStartup<T, K>
where
    T: Send + 'static,
    K: Send + 'static,
{
    async fn restore_network(
        &mut self,
        backup: NetworkBackup,
        options: RestoreOptions,
    ) -> Result<RestoreReport, Error> {
        // Mark before the first await so cancellation cannot turn a requested restore into a skip.
        self.restore_incomplete = true;
        if !self.ncp.backup_capabilities().await?.can_restore() {
            return Err(HardwareError::Unsupported(HardwareOperation::RestoreNetwork).into());
        }
        let report = self.ncp.restore_network(backup, options).await?;
        self.restore_incomplete = false;
        Ok(report)
    }
}
