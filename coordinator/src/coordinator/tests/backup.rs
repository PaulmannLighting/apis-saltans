use tokio::runtime::Runtime;
use tokio::sync::mpsc::channel;
use tokio::time::timeout;
use zb_core::IeeeAddress;
use zb_hw::{Driver, NetworkDump};

use super::{CAPACITY, DEADLINE, Descriptor, NonZeroUsize, TestDriver};
use crate::{
    Backup, BackupSource, Channel, ChannelMask, Coordinator, CoordinatorConfig, DeviceBackup,
    Error, HardwareError, HardwareOperation, LinkKeyState, NetworkBackup, NetworkKey,
    NetworkKeyState, NetworkParameters,
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
            let driver = TestDriver { events: hardware };
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
        let (ncp, actor) = TestDriver { events: hardware }.into_actor(NonZeroUsize::MIN);
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
