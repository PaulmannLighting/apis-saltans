//! Optional core network backup dispatch and actor construction.

use std::marker::PhantomData;
use std::num::NonZeroUsize;

use super::{Driver, serve};
use crate::{Error, NcpHandle, NetworkBackup, Operation, RestoreOptions, RestoreReport};

/// Optional consistent export of all core network state.
///
/// Implementations must synchronize with autonomous NCP activity. Include required security,
/// Trust Center policies, child records, and stack-specific state in the snapshot or its required
/// extensions. If complete export is impossible, fail rather than returning a partial backup.
pub trait NetworkDump
where
    Self: Driver,
{
    /// Capture a consistent snapshot, retaining the coordinator's network identity.
    ///
    /// # Errors
    /// Returns an error if complete export is unsupported or snapshot capture fails.
    fn dump_network(&mut self) -> impl Future<Output = Result<NetworkBackup, Error>> + Send;
}

/// Optional restoration of a coordinator's core network state.
///
/// The actor prepares normalized counters, validates the target, then applies the prepared backup.
/// Direct callers of this trait must perform those same steps. Hardware startup owns stopping and
/// subsequently starting the network; neither successful nor failed restore starts it.
pub trait NetworkRestore
where
    Self: Driver,
{
    /// Validate a prepared snapshot against an offline target, without mutating network storage.
    ///
    /// Check stopped state, IEEE-address restoration, key import, capacity, all required extensions,
    /// firmware compatibility, and extension counter advancement before the first write. Do not
    /// silently ignore unknown extensions. Counters in `backup` are already advanced; `options`
    /// describes the requested advances for opaque extension counters only.
    ///
    /// # Errors
    /// Returns a backup incompatibility, invalid-state, or backend read error before mutation.
    fn validate_restore(
        &mut self,
        backup: &NetworkBackup,
        options: RestoreOptions,
    ) -> impl Future<Output = Result<(), Error>> + Send;

    /// Persist and verify the validated snapshot while keeping the target network stopped.
    ///
    /// Normalized counters are already advanced and must not be advanced again. Prevent external
    /// state changes between validation and writing. Persist and read back the required state before
    /// returning a report. On hardware failure return `BackupError::RestoreFailed` with the mutation
    /// boundary, retaining backend diagnostics. Keep partial state offline. Actor task termination
    /// may interrupt this operation; dropping the caller's response receiver does not.
    ///
    /// # Errors
    /// Returns a restore failure identifying whether persistent state may have changed.
    fn restore_network(
        &mut self,
        backup: NetworkBackup,
        options: RestoreOptions,
    ) -> impl Future<Output = Result<RestoreReport, Error>> + Send;
}

/// Zero-sized marker for an unregistered optional operation.
#[derive(Debug)]
pub struct BackupDisabled;

/// Zero-sized marker forwarding an optional operation to its driver trait.
#[derive(Debug)]
pub struct BackupEnabled;

/// Builds a single driver actor with independently registered backup operations.
///
/// Existing `Driver::into_actor` is equivalent to building with both operations disabled.
#[must_use = "build the actor and continuously poll its driving future"]
#[derive(Debug)]
pub struct ActorBuilder<D, Dump = BackupDisabled, Restore = BackupDisabled> {
    driver: D,
    capacity: NonZeroUsize,
    policies: PhantomData<(Dump, Restore)>,
}

impl<D> ActorBuilder<D>
where
    D: Driver,
{
    pub(super) const fn new(driver: D, capacity: NonZeroUsize) -> Self {
        Self {
            driver,
            capacity,
            policies: PhantomData,
        }
    }
}

impl<D, Dump, Restore> ActorBuilder<D, Dump, Restore> {
    /// Register consistent network export. Requires the driver's optional dump trait.
    pub fn with_network_dump(self) -> ActorBuilder<D, BackupEnabled, Restore>
    where
        D: NetworkDump,
    {
        ActorBuilder {
            driver: self.driver,
            capacity: self.capacity,
            policies: PhantomData,
        }
    }

    /// Register network restoration. Requires the driver's optional restore trait.
    pub fn with_network_restore(self) -> ActorBuilder<D, Dump, BackupEnabled>
    where
        D: NetworkRestore,
    {
        ActorBuilder {
            driver: self.driver,
            capacity: self.capacity,
            policies: PhantomData,
        }
    }
}

impl<D, Dump, Restore> ActorBuilder<D, Dump, Restore>
where
    D: Driver,
    Dump: DumpDispatch<D>,
    Restore: RestoreDispatch<D>,
{
    /// Return the opaque handle and unspawned future owning this driver.
    /// The future resolves with the driver when every strong handle has been dropped.
    pub fn build(self) -> (NcpHandle, impl Future<Output = D> + Send) {
        let (handle, receiver) = NcpHandle::channel(self.capacity);
        (handle, serve::<D, Dump, Restore>(self.driver, receiver))
    }
}

// Public within this private module so public builder bounds are usable without exposing custom
// dispatch implementations. Only the two marker types above can be named through the crate API.
/// Internal static dump dispatch selected by actor construction.
pub trait DumpDispatch<D>: Send {
    const ENABLED: bool;
    fn dump(driver: &mut D) -> impl Future<Output = Result<NetworkBackup, Error>> + Send;
}

#[expect(
    clippy::unused_async_trait_impl,
    reason = "trait implementations use async fn consistently"
)]
impl<D> DumpDispatch<D> for BackupDisabled
where
    D: Driver,
{
    const ENABLED: bool = false;
    async fn dump(_driver: &mut D) -> Result<NetworkBackup, Error> {
        Err(Error::Unsupported(Operation::DumpNetwork))
    }
}

impl<D> DumpDispatch<D> for BackupEnabled
where
    D: NetworkDump,
{
    const ENABLED: bool = true;
    async fn dump(driver: &mut D) -> Result<NetworkBackup, Error> {
        let backup = driver.dump_network().await?;
        backup.validate()?;
        Ok(backup)
    }
}

/// Internal static restore dispatch selected by actor construction.
pub trait RestoreDispatch<D>: Send {
    const ENABLED: bool;
    fn restore(
        driver: &mut D,
        backup: NetworkBackup,
        options: RestoreOptions,
    ) -> impl Future<Output = Result<RestoreReport, Error>> + Send;
}

#[expect(
    clippy::unused_async_trait_impl,
    reason = "trait implementations use async fn consistently"
)]
impl<D> RestoreDispatch<D> for BackupDisabled
where
    D: Driver,
{
    const ENABLED: bool = false;
    async fn restore(
        _driver: &mut D,
        _backup: NetworkBackup,
        _options: RestoreOptions,
    ) -> Result<RestoreReport, Error> {
        Err(Error::Unsupported(Operation::RestoreNetwork))
    }
}

impl<D> RestoreDispatch<D> for BackupEnabled
where
    D: NetworkRestore,
{
    const ENABLED: bool = true;
    async fn restore(
        driver: &mut D,
        backup: NetworkBackup,
        options: RestoreOptions,
    ) -> Result<RestoreReport, Error> {
        backup.validate()?;
        let prepared = backup.prepare_restore(options)?;
        driver.validate_restore(&prepared, options).await?;
        driver.restore_network(prepared, options).await
    }
}
