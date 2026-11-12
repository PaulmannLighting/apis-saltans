use std::num::NonZeroU32;
use std::time::Duration;

use bytes::Bytes;
use tokio::runtime::Builder;
use tokio::sync::oneshot::{Receiver, Sender, channel};
use zb_aps::apsde::DataRequest;
use zb_core::IeeeAddress;
use zb_core::short_id::Device;
use zb_zdp::SimpleDescriptor;

use super::{ACTOR_CAPACITY, FakeDriver, PAN_ID};
use crate::common::backup::tests::snapshot;
use crate::common::message::Message;
use crate::{
    BackupError, ChannelMask, Driver, Error, FoundNetwork, NetworkBackup, NetworkDump,
    NetworkRestore, Operation, RestoreFailureState, RestoreOptions, RestoreReport, ScanDuration,
    ScannedChannel,
};

const ORIGINAL_MESSAGE_SIZE: usize = 72;

#[derive(Default)]
struct BackupDriver {
    common: FakeDriver,
    restored: Option<NetworkBackup>,
    validated: bool,
    reject: bool,
    fail_write: bool,
    started: Option<Sender<()>>,
    continue_restore: Option<Receiver<()>>,
}

impl Driver for BackupDriver {
    async fn get_endpoints(&self) -> Result<Box<[SimpleDescriptor]>, Error> {
        self.common.get_endpoints().await
    }
    async fn get_pan_id(&mut self) -> Result<u16, Error> {
        self.common.get_pan_id().await
    }
    async fn get_ieee_address(&mut self) -> Result<IeeeAddress, Error> {
        self.common.get_ieee_address().await
    }
    async fn scan_networks(
        &mut self,
        mask: ChannelMask,
        duration: ScanDuration,
    ) -> Result<Vec<FoundNetwork>, Error> {
        self.common.scan_networks(mask, duration).await
    }
    async fn scan_channels(
        &mut self,
        mask: ChannelMask,
        duration: ScanDuration,
    ) -> Result<Vec<ScannedChannel>, Error> {
        self.common.scan_channels(mask, duration).await
    }
    async fn allow_joins(&mut self, duration: Duration) -> Result<Duration, Error> {
        self.common.allow_joins(duration).await
    }
    async fn route_request(&mut self, radius: u8) -> Result<(), Error> {
        self.common.route_request(radius).await
    }
    async fn short_id_to_ieee_address(&mut self, short: Device) -> Result<IeeeAddress, Error> {
        self.common.short_id_to_ieee_address(short).await
    }
    async fn ieee_address_to_short_id(&mut self, ieee: IeeeAddress) -> Result<Device, Error> {
        self.common.ieee_address_to_short_id(ieee).await
    }
    async fn transmit(&mut self, request: DataRequest<Bytes>, counter: u8) -> Result<(), Error> {
        self.common.transmit(request, counter).await
    }
}

impl NetworkDump for BackupDriver {
    async fn dump_network(&mut self) -> Result<NetworkBackup, Error> {
        Ok(snapshot())
    }
}

impl NetworkRestore for BackupDriver {
    async fn validate_restore(
        &mut self,
        _backup: &NetworkBackup,
        _options: RestoreOptions,
    ) -> Result<(), Error> {
        self.validated = true;
        if self.reject {
            return Err(BackupError::Incompatible("test target cannot restore extensions").into());
        }
        Ok(())
    }

    async fn restore_network(
        &mut self,
        backup: NetworkBackup,
        _options: RestoreOptions,
    ) -> Result<RestoreReport, Error> {
        assert!(self.validated, "target validation must precede mutation");
        if let Some(started) = self.started.take() {
            started.send(()).unwrap();
        }
        if let Some(gate) = self.continue_restore.take() {
            gate.await.unwrap();
        }
        self.restored = Some(backup.clone());
        if self.fail_write {
            return Err(BackupError::restore_failed(
                RestoreFailureState::PossiblyModified,
                std::io::Error::other("test write failure"),
            )
            .into());
        }
        Ok(RestoreReport::from_verified_backup(&backup))
    }
}

fn options() -> RestoreOptions {
    RestoreOptions::new(NonZeroU32::MIN, NonZeroU32::MIN)
}

#[test]
fn unextended_driver_needs_no_optional_trait_implementations() {
    Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(async {
            let (handle, actor) = FakeDriver::default().into_actor(ACTOR_CAPACITY);
            let task = tokio::spawn(actor);
            let capabilities = handle.backup_capabilities().await.unwrap();
            assert!(!capabilities.can_dump());
            assert!(!capabilities.can_restore());
            assert!(matches!(
                handle.dump_network().await,
                Err(Error::Unsupported(Operation::DumpNetwork))
            ));
            assert!(matches!(
                handle.restore_network(snapshot(), options()).await,
                Err(Error::Unsupported(Operation::RestoreNetwork))
            ));
            assert_eq!(handle.get_pan_id().await.unwrap(), PAN_ID);
            let weak = handle.downgrade();
            drop(handle);
            task.await.unwrap();
            assert!(weak.upgrade().is_none());
        });
}

#[test]
fn registers_dump_and_restore_independently() {
    Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(async {
            let (handle, actor) = BackupDriver::default()
                .actor_builder(ACTOR_CAPACITY)
                .with_network_dump()
                .build();
            let task = tokio::spawn(actor);
            let caps = handle.backup_capabilities().await.unwrap();
            assert!(caps.can_dump());
            assert!(!caps.can_restore());
            assert_eq!(handle.dump_network().await.unwrap(), snapshot());
            assert!(matches!(
                handle.restore_network(snapshot(), options()).await,
                Err(Error::Unsupported(Operation::RestoreNetwork))
            ));
            drop(handle);
            assert!(task.await.unwrap().restored.is_none());

            let (handle, actor) = BackupDriver::default()
                .actor_builder(ACTOR_CAPACITY)
                .with_network_restore()
                .build();
            let task = tokio::spawn(actor);
            let caps = handle.backup_capabilities().await.unwrap();
            assert!(!caps.can_dump());
            assert!(caps.can_restore());
            assert!(matches!(
                handle.dump_network().await,
                Err(Error::Unsupported(Operation::DumpNetwork))
            ));
            let report = handle.restore_network(snapshot(), options()).await.unwrap();
            let expected = snapshot().prepare_restore(options()).unwrap();
            assert_eq!(report, RestoreReport::from_verified_backup(&expected));
            drop(handle);
            assert_eq!(task.await.unwrap().restored, Some(expected));
        });
}

#[test]
fn combined_registration_preserves_existing_commands_and_backup_round_trip() {
    Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(async {
            let (handle, actor) = BackupDriver::default()
                .actor_builder(ACTOR_CAPACITY)
                .with_network_restore()
                .with_network_dump()
                .build();
            let task = tokio::spawn(actor);
            let caps = handle.backup_capabilities().await.unwrap();
            assert!(caps.can_dump() && caps.can_restore());
            let backup = handle.dump_network().await.unwrap();
            handle.restore_network(backup, options()).await.unwrap();
            assert_eq!(handle.get_pan_id().await.unwrap(), PAN_ID);
            drop(handle);
            assert_eq!(
                task.await.unwrap().restored,
                Some(snapshot().prepare_restore(options()).unwrap())
            );
        });
}

#[test]
fn incompatibility_and_counter_exhaustion_prevent_mutation() {
    Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(async {
            let driver = BackupDriver {
                reject: true,
                ..BackupDriver::default()
            };
            let (handle, actor) = driver
                .actor_builder(ACTOR_CAPACITY)
                .with_network_restore()
                .build();
            let task = tokio::spawn(actor);
            assert!(matches!(
                handle.restore_network(snapshot(), options()).await,
                Err(Error::Backup(BackupError::Incompatible(_)))
            ));
            drop(handle);
            let driver = task.await.unwrap();
            assert!(driver.validated);
            assert!(driver.restored.is_none());

            let (handle, actor) = BackupDriver::default()
                .actor_builder(ACTOR_CAPACITY)
                .with_network_restore()
                .build();
            let task = tokio::spawn(actor);
            let exhausted =
                RestoreOptions::new(NonZeroU32::new(u32::MAX).unwrap(), NonZeroU32::MIN);
            assert!(matches!(
                handle.restore_network(snapshot(), exhausted).await,
                Err(Error::Backup(BackupError::CounterExhausted))
            ));
            drop(handle);
            let driver = task.await.unwrap();
            assert!(!driver.validated);
            assert!(driver.restored.is_none());
        });
}

#[test]
fn reports_partial_hardware_failure_and_actor_unavailability() {
    Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(async {
            let driver = BackupDriver {
                fail_write: true,
                ..BackupDriver::default()
            };
            let (handle, actor) = driver
                .actor_builder(ACTOR_CAPACITY)
                .with_network_restore()
                .build();
            let task = tokio::spawn(actor);
            assert!(matches!(
                handle.restore_network(snapshot(), options()).await,
                Err(Error::Backup(BackupError::RestoreFailed {
                    state: RestoreFailureState::PossiblyModified,
                    ..
                }))
            ));
            task.abort();
            assert!(task.await.is_err());
            assert!(matches!(
                handle.backup_capabilities().await,
                Err(Error::ActorUnavailable)
            ));
            assert!(matches!(
                handle.dump_network().await,
                Err(Error::ActorUnavailable)
            ));
            assert!(matches!(
                handle.restore_network(snapshot(), options()).await,
                Err(Error::ActorUnavailable)
            ));
        });
}

#[test]
fn dropping_restore_response_does_not_cancel_writes() {
    Builder::new_current_thread()
        .build()
        .unwrap()
        .block_on(async {
            let (started, wait_started) = channel();
            let (resume, gate) = channel();
            let driver = BackupDriver {
                started: Some(started),
                continue_restore: Some(gate),
                ..BackupDriver::default()
            };
            let (handle, actor) = driver
                .actor_builder(ACTOR_CAPACITY)
                .with_network_restore()
                .build();
            let task = tokio::spawn(actor);
            let caller = {
                let handle = handle.clone();
                tokio::spawn(async move { handle.restore_network(snapshot(), options()).await })
            };
            wait_started.await.unwrap();
            caller.abort();
            assert!(caller.await.is_err());
            resume.send(()).unwrap();
            assert_eq!(handle.get_pan_id().await.unwrap(), PAN_ID);
            drop(handle);
            assert!(task.await.unwrap().restored.is_some());
        });
}

#[cfg(target_pointer_width = "64")]
#[test]
fn backup_requests_do_not_increase_mailbox_slot_size() {
    assert_eq!(size_of::<Message>(), ORIGINAL_MESSAGE_SIZE);
}
