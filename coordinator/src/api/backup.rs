//! Core network backup operations with per-actor runtime capability checks.

pub use zb_hw::{
    BackupCapabilities, BackupError, BackupExtension, BackupSource, DeviceBackup,
    Error as HardwareError, LinkKeyState, NETWORK_BACKUP_VERSION, NETWORK_KEY_LENGTH,
    NetworkBackup, NetworkKey, NetworkKeyState, NetworkParameters, Operation as HardwareOperation,
    RestoreFailureState, RestoreOptions, RestoreReport,
};

use crate::{Coordinator, Error};

/// Export core Zigbee network identity and security state through the hardware actor.
///
/// Capabilities are checked at runtime. Snapshots exclude application-level information and contain
/// secrets; protect serialized backups at rest and in transit. Consistent capture during network
/// operation is the backend's responsibility.
pub trait Backup {
    /// Discover independently registered dump and restore support.
    /// Compatibility with individual backups is still validated during restoration.
    ///
    /// # Errors
    /// Returns a hardware error if the driver actor is unavailable.
    fn backup_capabilities(&self)
    -> impl Future<Output = Result<BackupCapabilities, Error>> + Send;

    /// Capture a complete core network snapshot while the coordinator is running.
    ///
    /// # Errors
    /// Returns a hardware unsupported-operation error when dump was not registered, or a hardware
    /// error if consistent capture fails. Backend error sources are preserved.
    fn dump_network(&self) -> impl Future<Output = Result<NetworkBackup, Error>> + Send;
}

impl Backup for Coordinator {
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
