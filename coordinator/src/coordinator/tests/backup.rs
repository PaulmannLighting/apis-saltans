use std::future::poll_fn;
use std::num::NonZeroU32;
use std::pin::pin;

use tokio::runtime::Runtime;
use tokio::sync::mpsc::channel;
use tokio::sync::oneshot;
use tokio::time::timeout;
use zb_core::IeeeAddress;
use zb_hw::{Driver, NetworkDump, NetworkRestore};

use super::{CAPACITY, DEADLINE, Descriptor, NonZeroUsize, TestDriver};
use crate::{
    Backup, BackupError, BackupSource, Channel, ChannelMask, Coordinator, CoordinatorConfig,
    CoordinatorStartup, DeviceBackup, Error, Event, HardwareError, HardwareOperation, LinkKeyState,
    Network, NetworkBackup, NetworkKey, NetworkKeyState, NetworkParameters, Restore,
    RestoreFailureState, RestoreOptions, RestoreReport,
};

const COORDINATOR: IeeeAddress = IeeeAddress::new(1, 2, 3, 4, 5, 6, 7, 8);
const REMOTE: IeeeAddress = IeeeAddress::new(8, 7, 6, 5, 4, 3, 2, 1);
const PAN_ID: u16 = 0x1234;
const EXTENDED_PAN_ID: [u8; 8] = [1; 8];
const KEY_BYTES: [u8; 16] = [42; 16];
const KEY_SEQUENCE: u8 = 0;
const NWK_UPDATE_ID: u8 = 0;
const SECURITY_LEVEL: u8 = 5;
const COUNTER: u32 = 10;

fn snapshot() -> NetworkBackup {
    NetworkBackup::new(
        BackupSource::new("test-stack".into(), "test-firmware".into()).unwrap(),
        NetworkParameters::new(
            COORDINATOR,
            PAN_ID,
            EXTENDED_PAN_ID,
            Channel::MIN,
            ChannelMask::ALL,
            NWK_UPDATE_ID,
            SECURITY_LEVEL,
        )
        .unwrap(),
        NetworkKeyState::new(NetworkKey::new(KEY_BYTES), KEY_SEQUENCE, COUNTER),
        Vec::new(),
        vec![DeviceBackup::new(
            REMOTE,
            None,
            true,
            Some(LinkKeyState::new(
                NetworkKey::new(KEY_BYTES),
                COUNTER,
                COUNTER,
            )),
            Some(COUNTER),
        )],
        Vec::new(),
    )
    .unwrap()
}

impl NetworkDump for TestDriver {
    async fn dump_network(&mut self) -> Result<NetworkBackup, HardwareError> {
        Ok(snapshot())
    }
}

#[test]
fn running_coordinator_checks_dump_support_at_runtime() {
    Runtime::new().unwrap().block_on(async {
        for enabled in [false, true] {
            let (hardware, incoming) = channel(CAPACITY);
            let (events, _received) = channel(CAPACITY);
            let driver = TestDriver::new(hardware);
            let (ncp, driver) = if enabled {
                let (ncp, actor) = driver
                    .actor_builder(NonZeroUsize::MIN)
                    .with_network_dump()
                    .build();
                (ncp, tokio::spawn(actor))
            } else {
                let (ncp, actor) = driver.into_actor(NonZeroUsize::MIN);
                (ncp, tokio::spawn(actor))
            };
            let (coordinator, runtime) = Coordinator::start_managed(
                ncp,
                Descriptor::default(),
                incoming,
                events,
                CoordinatorConfig::default(),
            )
            .unwrap();
            let capabilities = coordinator.backup_capabilities().await.unwrap();
            assert_eq!(capabilities.can_dump(), enabled);
            assert!(!capabilities.can_restore());
            let result = coordinator.dump_network().await;
            if enabled {
                assert_eq!(result.unwrap(), snapshot());
            } else {
                assert!(matches!(
                    result,
                    Err(Error::Hardware(HardwareError::Unsupported(
                        HardwareOperation::DumpNetwork
                    )))
                ));
            }
            runtime.shutdown().await;
            drop(coordinator);
            timeout(DEADLINE, driver).await.unwrap().unwrap();
        }
    });
}

#[test]
fn backup_preserves_actor_unavailable_errors() {
    Runtime::new().unwrap().block_on(async {
        let (hardware, incoming) = channel(CAPACITY);
        let (events, _received) = channel(CAPACITY);
        let (ncp, actor) = TestDriver::new(hardware).into_actor(NonZeroUsize::MIN);
        let driver = tokio::spawn(actor);
        let (coordinator, runtime) = Coordinator::start_managed(
            ncp,
            Descriptor::default(),
            incoming,
            events,
            CoordinatorConfig::default(),
        )
        .unwrap();
        driver.abort();
        assert!(driver.await.is_err());
        assert!(matches!(
            coordinator.backup_capabilities().await,
            Err(Error::Hardware(HardwareError::ActorUnavailable))
        ));
        assert!(matches!(
            coordinator.dump_network().await,
            Err(Error::Hardware(HardwareError::ActorUnavailable))
        ));
        runtime.shutdown().await;
    });
}

#[derive(Default)]
pub(super) struct RecoveryState {
    restored: Option<NetworkBackup>,
    reject_restore: bool,
    fail_write: bool,
    started: Option<oneshot::Sender<()>>,
    gate: Option<oneshot::Receiver<()>>,
}

impl NetworkRestore for TestDriver {
    async fn validate_restore(
        &mut self,
        _backup: &NetworkBackup,
        _options: RestoreOptions,
    ) -> Result<(), HardwareError> {
        if self.recovery.reject_restore {
            return Err(BackupError::Incompatible("test target").into());
        }
        Ok(())
    }

    async fn restore_network(
        &mut self,
        backup: NetworkBackup,
        _options: RestoreOptions,
    ) -> Result<RestoreReport, HardwareError> {
        if let Some(started) = self.recovery.started.take() {
            started.send(()).unwrap();
        }
        if let Some(gate) = self.recovery.gate.take() {
            gate.await.unwrap();
        }
        self.recovery.restored = Some(backup.clone());
        if self.recovery.fail_write {
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
fn restore_only_driver_is_discovered_and_unsupported_dump_is_reported() {
    Runtime::new().unwrap().block_on(async {
        let (hardware, incoming) = channel(CAPACITY);
        let (events, _received) = channel(CAPACITY);
        let (ncp, actor) = TestDriver::new(hardware)
            .actor_builder(NonZeroUsize::MIN)
            .with_network_restore()
            .build();
        let driver = tokio::spawn(actor);
        let mut startup = CoordinatorStartup::new(
            ncp,
            Descriptor::default(),
            incoming,
            events,
            CoordinatorConfig::default(),
        );
        let capabilities = startup.backup_capabilities().await.unwrap();
        assert!(!capabilities.can_dump());
        assert!(capabilities.can_restore());
        assert!(matches!(
            startup.dump_network().await,
            Err(Error::Hardware(HardwareError::Unsupported(
                HardwareOperation::DumpNetwork
            )))
        ));
        startup
            .restore_network(snapshot(), options())
            .await
            .unwrap();
        drop(startup);
        assert!(
            timeout(DEADLINE, driver)
                .await
                .unwrap()
                .unwrap()
                .recovery
                .restored
                .is_some()
        );
    });
}

#[test]
fn unsupported_restore_blocks_startup_and_creates_no_protocol_runtime() {
    Runtime::new().unwrap().block_on(async {
        let (hardware, incoming) = channel(CAPACITY);
        let (events, mut received) = channel(CAPACITY);
        let (ncp, actor) = TestDriver::new(hardware).into_actor(NonZeroUsize::MIN);
        let driver = tokio::spawn(actor);
        let mut startup = CoordinatorStartup::new(
            ncp,
            Descriptor::default(),
            incoming,
            events,
            CoordinatorConfig::default(),
        );
        assert!(!startup.backup_capabilities().await.unwrap().can_restore());
        assert!(matches!(
            startup.restore_network(snapshot(), options()).await,
            Err(Error::Hardware(HardwareError::Unsupported(
                HardwareOperation::RestoreNetwork
            )))
        ));
        assert!(matches!(
            startup.start_managed(),
            Err(Error::RestoreIncomplete)
        ));
        assert!(timeout(DEADLINE, received.recv()).await.unwrap().is_none());
        assert!(
            timeout(DEADLINE, driver)
                .await
                .unwrap()
                .unwrap()
                .recovery
                .restored
                .is_none()
        );
    });
}

#[test]
fn startup_restores_before_protocols_start_and_preserves_config_and_buffered_events() {
    Runtime::new().unwrap().block_on(async {
        let (hardware, incoming) = channel(CAPACITY);
        hardware
            .send(zb_hw::Event::Network(zb_hw::NetworkEvent::Up))
            .await
            .unwrap();
        let (events, mut received) = channel(CAPACITY);
        let (ncp, actor) = TestDriver::new(hardware)
            .actor_builder(NonZeroUsize::MIN)
            .with_network_dump()
            .with_network_restore()
            .build();
        let driver = tokio::spawn(actor);
        let config = CoordinatorConfig {
            inbox_capacity: NonZeroUsize::MIN,
            ..CoordinatorConfig::default()
        };
        let mut startup =
            CoordinatorStartup::new(ncp, Descriptor::default(), incoming, events, config);
        let original = startup.dump_network().await.unwrap();
        let expected = original.prepare_restore(options()).unwrap();
        assert_eq!(
            startup.restore_network(original, options()).await.unwrap(),
            RestoreReport::from_verified_backup(&expected)
        );
        assert!(matches!(
            received.try_recv(),
            Err(tokio::sync::mpsc::error::TryRecvError::Empty)
        ));
        let (coordinator, runtime) = startup.start_managed().unwrap();
        assert_eq!(coordinator.zcl.max_capacity(), NonZeroUsize::MIN.get());
        assert!(matches!(
            timeout(DEADLINE, received.recv()).await.unwrap(),
            Some(Event::Network(Network::Up))
        ));
        assert_eq!(coordinator.dump_network().await.unwrap(), snapshot());
        runtime.shutdown().await;
        drop(coordinator);
        assert_eq!(
            timeout(DEADLINE, driver)
                .await
                .unwrap()
                .unwrap()
                .recovery
                .restored,
            Some(expected)
        );
    });
}

#[test]
fn restore_errors_preserve_the_hardware_failure_and_prevent_startup() {
    Runtime::new().unwrap().block_on(async {
        for reject in [false, true] {
            let (hardware, incoming) = channel(CAPACITY);
            let (events, mut received) = channel(CAPACITY);
            let mut backend = TestDriver::new(hardware);
            backend.recovery.reject_restore = reject;
            backend.recovery.fail_write = !reject;
            let (ncp, actor) = backend
                .actor_builder(NonZeroUsize::MIN)
                .with_network_restore()
                .build();
            let driver = tokio::spawn(actor);
            let mut startup = CoordinatorStartup::new(
                ncp,
                Descriptor::default(),
                incoming,
                events,
                CoordinatorConfig::default(),
            );
            let error = startup
                .restore_network(snapshot(), options())
                .await
                .unwrap_err();
            if reject {
                assert!(matches!(
                    error,
                    Error::Hardware(HardwareError::Backup(BackupError::Incompatible(_)))
                ));
            } else {
                let Error::Hardware(HardwareError::Backup(BackupError::RestoreFailed {
                    state,
                    source,
                })) = error
                else {
                    panic!("hardware failure preserved");
                };
                assert_eq!(state, RestoreFailureState::PossiblyModified);
                assert_eq!(source.to_string(), "test write failure");
            }
            assert!(matches!(startup.start(), Err(Error::RestoreIncomplete)));
            assert!(timeout(DEADLINE, received.recv()).await.unwrap().is_none());
            let backend = timeout(DEADLINE, driver).await.unwrap().unwrap();
            assert_eq!(backend.recovery.restored.is_none(), reject);
        }
    });
}

#[test]
fn counter_exhaustion_blocks_startup_without_writes() {
    Runtime::new().unwrap().block_on(async {
        let (hardware, incoming) = channel(CAPACITY);
        let (events, _received) = channel(CAPACITY);
        let (ncp, actor) = TestDriver::new(hardware)
            .actor_builder(NonZeroUsize::MIN)
            .with_network_restore()
            .build();
        let driver = tokio::spawn(actor);
        let mut startup = CoordinatorStartup::new(
            ncp,
            Descriptor::default(),
            incoming,
            events,
            CoordinatorConfig::default(),
        );
        let exhausted = RestoreOptions::new(NonZeroU32::new(u32::MAX).unwrap(), NonZeroU32::MIN);
        assert!(matches!(
            startup.restore_network(snapshot(), exhausted).await,
            Err(Error::Hardware(HardwareError::Backup(
                BackupError::CounterExhausted
            )))
        ));
        assert!(matches!(
            startup.start_managed(),
            Err(Error::RestoreIncomplete)
        ));
        assert!(
            timeout(DEADLINE, driver)
                .await
                .unwrap()
                .unwrap()
                .recovery
                .restored
                .is_none()
        );
    });
}

#[test]
fn cancelled_restore_cannot_be_silently_skipped_at_startup() {
    Runtime::new().unwrap().block_on(async {
        let (hardware, incoming) = channel(CAPACITY);
        let (events, _received) = channel(CAPACITY);
        let (started, wait_started) = oneshot::channel();
        let (resume, gate) = oneshot::channel();
        let mut backend = TestDriver::new(hardware);
        backend.recovery.started = Some(started);
        backend.recovery.gate = Some(gate);
        let (ncp, actor) = backend
            .actor_builder(NonZeroUsize::MIN)
            .with_network_restore()
            .build();
        let driver = tokio::spawn(actor);
        let mut startup = CoordinatorStartup::new(
            ncp,
            Descriptor::default(),
            incoming,
            events,
            CoordinatorConfig::default(),
        );
        {
            let mut restore = pin!(startup.restore_network(snapshot(), options()));
            let mut wait_started = pin!(wait_started);
            timeout(
                DEADLINE,
                poll_fn(|context| {
                    assert!(restore.as_mut().poll(context).is_pending());
                    wait_started.as_mut().poll(context)
                }),
            )
            .await
            .unwrap()
            .unwrap();
        }
        assert!(matches!(
            startup.start_managed(),
            Err(Error::RestoreIncomplete)
        ));
        resume.send(()).unwrap();
        assert!(
            timeout(DEADLINE, driver)
                .await
                .unwrap()
                .unwrap()
                .recovery
                .restored
                .is_some()
        );
    });
}

#[test]
fn explicit_successful_restore_can_resolve_an_earlier_failed_attempt() {
    Runtime::new().unwrap().block_on(async {
        let (hardware, incoming) = channel(CAPACITY);
        let (events, _received) = channel(CAPACITY);
        let (ncp, actor) = TestDriver::new(hardware)
            .actor_builder(NonZeroUsize::MIN)
            .with_network_restore()
            .build();
        let driver = tokio::spawn(actor);
        let mut startup = CoordinatorStartup::new(
            ncp,
            Descriptor::default(),
            incoming,
            events,
            CoordinatorConfig::default(),
        );
        let exhausted = RestoreOptions::new(NonZeroU32::new(u32::MAX).unwrap(), NonZeroU32::MIN);
        assert!(
            startup
                .restore_network(snapshot(), exhausted)
                .await
                .is_err()
        );
        startup
            .restore_network(snapshot(), options())
            .await
            .unwrap();
        let (coordinator, runtime) = startup.start_managed().unwrap();
        runtime.shutdown().await;
        drop(coordinator);
        assert!(
            timeout(DEADLINE, driver)
                .await
                .unwrap()
                .unwrap()
                .recovery
                .restored
                .is_some()
        );
    });
}

#[test]
fn prepared_startup_without_a_restore_preserves_the_detached_start_path() {
    Runtime::new().unwrap().block_on(async {
        let (hardware, incoming) = channel(CAPACITY);
        let (events, mut received) = channel(CAPACITY);
        let (ncp, actor) = TestDriver::new(hardware).into_actor(NonZeroUsize::MIN);
        let driver = tokio::spawn(actor);
        let startup = CoordinatorStartup::new(
            ncp,
            Descriptor::default(),
            incoming,
            events,
            CoordinatorConfig::default(),
        );
        let coordinator = startup.start().unwrap();
        // Stopping the hardware actor closes its event source and terminates detached protocols.
        driver.abort();
        assert!(driver.await.is_err());
        assert!(matches!(
            timeout(DEADLINE, received.recv()).await.unwrap(),
            Some(Event::Network(Network::Error(
                crate::NetworkError::HardwareEventStreamClosed
            )))
        ));
        drop(coordinator);
    });
}
