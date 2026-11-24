use crate::{Error, NetworkBackup, RestoreOptions, RestoreReport};

/// Restore core network state before coordinator protocol startup.
///
/// Implemented by [`crate::CoordinatorStartup`], not a running [`crate::Coordinator`]. Support is
/// checked at runtime. Restoration requires an offline hardware network and preserves the original
/// coordinator identity. A complete restore leaves the network stopped. With
/// [`RestoreOptions::allow_rejoin`], a backend can recover a rejoin backup by forming the network.
/// The report tells the caller whether backend startup is required.
///
/// ```compile_fail
/// use apis_saltans_coordinator::{Coordinator, NetworkBackup, Restore, RestoreOptions};
///
/// async fn restore_running(coordinator: &mut Coordinator, backup: NetworkBackup, options: RestoreOptions) {
///     coordinator.restore_network(backup, options).await.unwrap();
/// }
/// ```
pub trait Restore {
    /// Restore and verify a core network snapshot using explicit outgoing counter advances.
    ///
    /// Callers must choose advances sufficient for traffic since capture, including earlier
    /// restores. No finite advance makes an arbitrarily stale backup safe. A complete restore
    /// retains incoming counters. Do not pre-advance the snapshot: hardware dispatch handles normalized counters.
    ///
    /// A failed or cancelled attempt blocks startup until an explicit restore succeeds. Dropping
    /// the response future may leave a hardware restore in progress with an unknown outcome; verify
    /// before retrying. An opted-in rejoin restore may already have started the hardware network.
    /// Coordinator startup starts the protocol actors only.
    ///
    /// # Errors
    /// Returns [`Error::Hardware`] for unsupported operations, invalid state, incompatible backups,
    /// counter exhaustion, unavailable actors, or hardware failures. Underlying error sources and
    /// the hardware mutation boundary are preserved. Failed restoration never starts protocol actors.
    fn restore_network(
        &mut self,
        backup: NetworkBackup,
        options: RestoreOptions,
    ) -> impl Future<Output = Result<RestoreReport, Error>> + Send;
}
