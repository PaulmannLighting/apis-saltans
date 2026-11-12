//! Validated deserialization of invariant-bearing backup records.

use serde::{Deserialize, Deserializer};
use zb_core::IeeeAddress;

use super::{
    BackupExtension, BackupSource, DeviceBackup, NetworkBackup, NetworkKeyState, NetworkParameters,
};
use crate::{Channel, ChannelMask};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BackupSourceData {
    stack: String,
    firmware: String,
}
impl<'de> Deserialize<'de> for BackupSource {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = BackupSourceData::deserialize(deserializer)?;
        let value = Self {
            stack: raw.stack,
            firmware: raw.firmware,
        };
        value.validate().map_err(serde::de::Error::custom)?;
        Ok(value)
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NetworkParametersData {
    coordinator_ieee: IeeeAddress,
    pan_id: u16,
    extended_pan_id: [u8; 8],
    channel: Channel,
    channel_mask: ChannelMask,
    nwk_update_id: u8,
    security_level: u8,
}
impl<'de> Deserialize<'de> for NetworkParameters {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = NetworkParametersData::deserialize(deserializer)?;
        let value = Self {
            coordinator_ieee: raw.coordinator_ieee,
            pan_id: raw.pan_id,
            extended_pan_id: raw.extended_pan_id,
            channel: raw.channel,
            channel_mask: raw.channel_mask,
            nwk_update_id: raw.nwk_update_id,
            security_level: raw.security_level,
        };
        value.validate().map_err(serde::de::Error::custom)?;
        Ok(value)
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct BackupExtensionData {
    namespace: String,
    name: String,
    version: u32,
    payload: Vec<u8>,
}
impl<'de> Deserialize<'de> for BackupExtension {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = BackupExtensionData::deserialize(deserializer)?;
        let value = Self {
            namespace: raw.namespace,
            name: raw.name,
            version: raw.version,
            payload: raw.payload,
        };
        value.validate().map_err(serde::de::Error::custom)?;
        Ok(value)
    }
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct NetworkBackupData {
    version: u32,
    source: BackupSource,
    network: NetworkParameters,
    network_key: NetworkKeyState,
    additional_network_keys: Vec<NetworkKeyState>,
    devices: Vec<DeviceBackup>,
    extensions: Vec<BackupExtension>,
}
impl<'de> Deserialize<'de> for NetworkBackup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = NetworkBackupData::deserialize(deserializer)?;
        let value = Self {
            version: raw.version,
            source: raw.source,
            network: raw.network,
            network_key: raw.network_key,
            additional_network_keys: raw.additional_network_keys,
            devices: raw.devices,
            extensions: raw.extensions,
        };
        value.validate().map_err(serde::de::Error::custom)?;
        Ok(value)
    }
}
