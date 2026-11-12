use std::num::NonZeroU32;

use zb_core::IeeeAddress;

use super::{
    BackupError, BackupExtension, BackupSource, DeviceBackup, LinkKeyState, NetworkBackup,
    NetworkKey, NetworkKeyState, NetworkParameters, RestoreOptions, RestoreReport,
};
use crate::{Channel, ChannelMask};

const COORDINATOR: IeeeAddress = IeeeAddress::new(1, 2, 3, 4, 5, 6, 7, 8);
const REMOTE: IeeeAddress = IeeeAddress::new(8, 7, 6, 5, 4, 3, 2, 1);
const PAN_ID: u16 = 0x1234;
const KEY_BYTES: [u8; 16] = [42; 16];
const COUNTER: u32 = 10;
const LINK_INCREMENT: NonZeroU32 = NonZeroU32::new(2).unwrap();

pub(crate) fn snapshot() -> NetworkBackup {
    NetworkBackup::new(
        BackupSource::new("test-stack".into(), "test-firmware".into()).unwrap(),
        NetworkParameters::new(
            COORDINATOR,
            PAN_ID,
            [1; 8],
            Channel::MIN,
            ChannelMask::ALL,
            0,
            5,
        )
        .unwrap(),
        NetworkKeyState::new(NetworkKey::new(KEY_BYTES), 0, COUNTER),
        vec![NetworkKeyState::new(NetworkKey::new(KEY_BYTES), 1, COUNTER)],
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
        vec![
            BackupExtension::new("test-stack".into(), "security".into(), 1, vec![42, 43]).unwrap(),
        ],
    )
    .unwrap()
}

#[test]
fn prepares_all_outgoing_counters_without_changing_snapshot_or_incoming_counters() {
    let backup = snapshot();
    let prepared = backup
        .prepare_restore(RestoreOptions::new(NonZeroU32::MIN, LINK_INCREMENT))
        .unwrap();
    assert_eq!(backup.network_key().tx_counter(), COUNTER);
    assert_eq!(prepared.network_key().tx_counter(), COUNTER + 1);
    assert_eq!(
        prepared.additional_network_keys()[0].tx_counter(),
        COUNTER + 1
    );
    assert_eq!(
        prepared.devices()[0].link_key().unwrap().tx_counter(),
        COUNTER + LINK_INCREMENT.get()
    );
    assert_eq!(
        prepared.devices()[0].link_key().unwrap().rx_counter(),
        COUNTER
    );
    assert_eq!(prepared.devices()[0].nwk_rx_counter(), Some(COUNTER));
    assert_eq!(prepared.network_key().key(), backup.network_key().key());
    assert_eq!(prepared.extensions(), backup.extensions());
    let report = RestoreReport::from_verified_backup(&prepared);
    assert_eq!(report.coordinator_ieee(), COORDINATOR);
    assert_eq!(
        report.network_counters(),
        &[(0, COUNTER + 1), (1, COUNTER + 1)]
    );
    assert_eq!(
        report.link_counters(),
        &[(REMOTE, COUNTER + LINK_INCREMENT.get())]
    );
}

#[test]
fn rejects_exhaustion_in_each_counter_class() {
    let options = RestoreOptions::new(NonZeroU32::MIN, NonZeroU32::MIN);
    for index in 0..3 {
        let mut backup = snapshot();
        match index {
            0 => backup.network_key.tx_counter = u32::MAX,
            1 => backup.additional_network_keys[0].tx_counter = u32::MAX - 1,
            _ => backup.devices[0].link_key.as_mut().unwrap().tx_counter = u32::MAX,
        }
        assert!(matches!(
            backup.prepare_restore(options),
            Err(BackupError::CounterExhausted)
        ));
    }
}

#[test]
fn rejects_duplicate_addresses_key_sequences_and_extensions() {
    let mut backup = snapshot();
    backup.devices.push(backup.devices[0].clone());
    assert!(matches!(
        backup.validate(),
        Err(BackupError::DuplicateDevice)
    ));
    let mut backup = snapshot();
    backup.additional_network_keys[0].sequence_number = backup.network_key.sequence_number;
    assert!(matches!(
        backup.validate(),
        Err(BackupError::DuplicateKeySequence)
    ));
    let mut backup = snapshot();
    backup.extensions.push(backup.extensions[0].clone());
    assert!(matches!(
        backup.validate(),
        Err(BackupError::DuplicateExtension)
    ));
    let mut backup = snapshot();
    backup.devices[0].ieee_address = COORDINATOR;
    assert!(matches!(
        backup.validate(),
        Err(BackupError::CoordinatorInDeviceTable)
    ));
}

#[test]
fn validates_network_parameters_and_metadata() {
    assert!(
        NetworkParameters::new(
            COORDINATOR,
            u16::MAX,
            [1; 8],
            Channel::MIN,
            ChannelMask::ALL,
            0,
            5
        )
        .is_err()
    );
    assert!(
        NetworkParameters::new(
            COORDINATOR,
            PAN_ID,
            [1; 8],
            Channel::MIN,
            ChannelMask::empty(),
            0,
            5
        )
        .is_err()
    );
    assert!(
        NetworkParameters::new(
            COORDINATOR,
            PAN_ID,
            [1; 8],
            Channel::MIN,
            ChannelMask::ALL,
            0,
            8
        )
        .is_err()
    );
    assert!(BackupSource::new(" ".into(), "firmware".into()).is_err());
    assert!(BackupExtension::new("stack".into(), String::new(), 1, vec![]).is_err());
    assert!(BackupExtension::new("stack".into(), "name".into(), 0, vec![]).is_err());
}

#[test]
fn debug_redacts_secrets_at_every_level() {
    let output = format!("{:?}", snapshot());
    assert!(output.contains("[REDACTED]"));
    assert!(!output.contains("[42, 42"));
    assert!(!output.contains("[42, 43]"));
}

#[cfg(feature = "serde")]
#[test]
fn serialization_round_trip_preserves_secrets_and_rejects_invalid_input() {
    let backup = snapshot();
    let json = serde_json::to_value(&backup).unwrap();
    assert_eq!(
        serde_json::from_value::<NetworkBackup>(json.clone()).unwrap(),
        backup
    );
    for (pointer, value) in [
        ("/version", serde_json::json!(999)),
        ("/network/security_level", serde_json::json!(8)),
        ("/network/channel_mask", serde_json::json!(1)),
        ("/network/pan_id", serde_json::json!(65535)),
        ("/extensions/0/version", serde_json::json!(0)),
        ("/source/stack", serde_json::json!("")),
    ] {
        let mut invalid = json.clone();
        *invalid.pointer_mut(pointer).unwrap() = value;
        assert!(
            serde_json::from_value::<NetworkBackup>(invalid).is_err(),
            "accepted {pointer}"
        );
    }
    let mut duplicate = json.clone();
    duplicate["devices"]
        .as_array_mut()
        .unwrap()
        .push(json["devices"][0].clone());
    assert!(serde_json::from_value::<NetworkBackup>(duplicate).is_err());
}
