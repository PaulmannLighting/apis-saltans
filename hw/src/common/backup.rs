//! Core Zigbee coordinator identity and security snapshots.

use std::collections::BTreeSet;
use std::fmt::{self, Debug, Formatter};
use std::num::NonZeroU32;
use std::sync::Arc;

use thiserror::Error;
use zb_core::IeeeAddress;
use zb_core::short_id::Device;

use crate::{Channel, ChannelMask};

#[cfg(feature = "serde")]
mod serialization;
#[cfg(test)]
pub(crate) mod tests;

/// Current version of this crate's network backup representation.
pub const NETWORK_BACKUP_VERSION: u32 = 1;
/// Length of a Zigbee AES key in bytes.
pub const NETWORK_KEY_LENGTH: usize = 16;

const MAX_SECURITY_LEVEL: u8 = 7;
const BROADCAST_PAN_ID: u16 = u16::MAX;

/// Failure boundary of a hardware restore.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RestoreFailureState {
    /// No persistent network state was changed.
    Unchanged,

    /// Persistent state may have been partially changed; keep the network stopped.
    PossiblyModified,
}

/// Validation, compatibility, or hardware restoration failure.
#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum BackupError {
    /// The serialized representation uses an unsupported schema version.
    #[error("unsupported network backup version: {0}")]
    UnsupportedVersion(u32),

    /// Network parameters are internally inconsistent.
    #[error("invalid network parameters")]
    InvalidNetworkParameters,

    /// Source metadata is missing.
    #[error("backup source metadata must not be empty")]
    InvalidSource,

    /// An extension has an empty namespace/name or invalid version.
    #[error("invalid backup extension identifier or version")]
    InvalidExtension,

    /// A device IEEE or allocated short address occurs more than once.
    #[error("duplicate device address in network backup")]
    DuplicateDevice,

    /// The coordinator was included as a remote device.
    #[error("coordinator cannot occur in the remote device table")]
    CoordinatorInDeviceTable,

    /// More than one network key has the same sequence number.
    #[error("duplicate network key sequence number")]
    DuplicateKeySequence,

    /// An extension identifier occurs more than once.
    #[error("duplicate backup extension identifier")]
    DuplicateExtension,

    /// The target cannot restore some required network state.
    #[error("incompatible network backup: {0}")]
    Incompatible(&'static str),

    /// The operation requires a different hardware lifecycle state.
    #[error("invalid hardware state for network backup operation")]
    InvalidState,

    /// Advancing an outgoing security counter would exhaust it.
    #[error("outgoing security counter exhausted")]
    CounterExhausted,

    /// A hardware restore failed with an explicit mutation boundary.
    #[error("hardware restore failed ({state:?}): {source}")]
    RestoreFailed {
        /// Whether persistent state may have changed.
        state: RestoreFailureState,
        /// Backend diagnostics, which must not contain secret material.
        #[source]
        source: Arc<dyn std::error::Error + Send + Sync>,
    },
}

impl BackupError {
    /// Preserve a backend restore failure and its mutation boundary.
    pub fn restore_failed<E>(state: RestoreFailureState, source: E) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
    {
        Self::RestoreFailed {
            state,
            source: Arc::new(source),
        }
    }
}

/// AES key material. Debug output is redacted; serialization contains the secret.
///
/// This type does not promise memory zeroization. Avoid unnecessary copies and protect backups
/// at rest and in transit.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Eq, PartialEq)]
pub struct NetworkKey([u8; NETWORK_KEY_LENGTH]);

impl NetworkKey {
    /// Wrap an AES key in its protocol byte order.
    #[must_use]
    pub const fn new(bytes: [u8; NETWORK_KEY_LENGTH]) -> Self {
        Self(bytes)
    }

    /// Access secret key bytes explicitly.
    #[must_use]
    pub const fn expose_secret(&self) -> &[u8; NETWORK_KEY_LENGTH] {
        &self.0
    }
}

impl Debug for NetworkKey {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("NetworkKey([REDACTED])")
    }
}

/// Stack and firmware provenance of a snapshot.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BackupSource {
    stack: String,
    firmware: String,
}

impl BackupSource {
    /// Construct stack and firmware provenance of a snapshot.
    ///
    /// # Errors
    /// Returns an error if the supplied state is inconsistent.
    pub fn new(stack: String, firmware: String) -> Result<Self, BackupError> {
        let value = Self { stack, firmware };
        value.validate()?;
        Ok(value)
    }
    /// Stable stack identifier.
    #[must_use]
    pub fn stack(&self) -> &str {
        &self.stack
    }
    /// Firmware version or build identifier.
    #[must_use]
    pub fn firmware(&self) -> &str {
        &self.firmware
    }
    fn validate(&self) -> Result<(), BackupError> {
        if self.stack.trim().is_empty() || self.firmware.trim().is_empty() {
            return Err(BackupError::InvalidSource);
        }
        Ok(())
    }
}

/// Network identity and configuration for a page-zero Zigbee network.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NetworkParameters {
    coordinator_ieee: IeeeAddress,
    pan_id: u16,
    extended_pan_id: [u8; 8],
    channel: Channel,
    channel_mask: ChannelMask,
    nwk_update_id: u8,
    security_level: u8,
}

impl NetworkParameters {
    /// Construct network identity and configuration for a page-zero Zigbee network.
    ///
    /// # Errors
    /// Returns an error if the supplied state is inconsistent.
    pub fn new(
        coordinator_ieee: IeeeAddress,
        pan_id: u16,
        extended_pan_id: [u8; 8],
        channel: Channel,
        channel_mask: ChannelMask,
        nwk_update_id: u8,
        security_level: u8,
    ) -> Result<Self, BackupError> {
        let value = Self {
            coordinator_ieee,
            pan_id,
            extended_pan_id,
            channel,
            channel_mask,
            nwk_update_id,
            security_level,
        };
        value.validate()?;
        Ok(value)
    }
    /// Coordinator IEEE address to preserve.
    #[must_use]
    pub const fn coordinator_ieee(&self) -> IeeeAddress {
        self.coordinator_ieee
    }
    /// Short PAN identifier.
    #[must_use]
    pub const fn pan_id(&self) -> u16 {
        self.pan_id
    }
    /// Extended PAN identifier in display (big-endian) order.
    #[must_use]
    pub const fn extended_pan_id(&self) -> [u8; 8] {
        self.extended_pan_id
    }
    /// Current operating channel.
    #[must_use]
    pub const fn channel(&self) -> Channel {
        self.channel
    }
    /// Permitted channel-page-zero channels.
    #[must_use]
    pub const fn channel_mask(&self) -> ChannelMask {
        self.channel_mask
    }
    /// Network update identifier.
    #[must_use]
    pub const fn nwk_update_id(&self) -> u8 {
        self.nwk_update_id
    }
    /// Network security level.
    #[must_use]
    pub const fn security_level(&self) -> u8 {
        self.security_level
    }
    const fn validate(&self) -> Result<(), BackupError> {
        if self.pan_id == BROADCAST_PAN_ID
            || self.security_level > MAX_SECURITY_LEVEL
            || ChannelMask::new(self.channel_mask.bits()).is_none()
            || !self.channel_mask.contains(self.channel)
        {
            return Err(BackupError::InvalidNetworkParameters);
        }
        Ok(())
    }
}

/// Network key and associated security state.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NetworkKeyState {
    key: NetworkKey,
    sequence_number: u8,
    tx_counter: u32,
}

impl NetworkKeyState {
    /// Construct network key and associated security state.
    #[must_use]
    pub const fn new(key: NetworkKey, sequence_number: u8, tx_counter: u32) -> Self {
        Self {
            key,
            sequence_number,
            tx_counter,
        }
    }
    /// Secret network key.
    #[must_use]
    pub const fn key(&self) -> &NetworkKey {
        &self.key
    }
    /// Network key sequence number.
    #[must_use]
    pub const fn sequence_number(&self) -> u8 {
        self.sequence_number
    }
    /// Last used outgoing security counter; never the wrapping APS correlation counter.
    #[must_use]
    pub const fn tx_counter(&self) -> u32 {
        self.tx_counter
    }
}

/// Per-device APS or Trust Center link key and replay state.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinkKeyState {
    key: NetworkKey,
    tx_counter: u32,
    rx_counter: u32,
}

impl LinkKeyState {
    /// Construct per-device APS or Trust Center link key and replay state.
    #[must_use]
    pub const fn new(key: NetworkKey, tx_counter: u32, rx_counter: u32) -> Self {
        Self {
            key,
            tx_counter,
            rx_counter,
        }
    }
    /// Secret link key.
    #[must_use]
    pub const fn key(&self) -> &NetworkKey {
        &self.key
    }
    /// Last used outgoing link-key security counter.
    #[must_use]
    pub const fn tx_counter(&self) -> u32 {
        self.tx_counter
    }
    /// Highest accepted incoming link-key security counter.
    #[must_use]
    pub const fn rx_counter(&self) -> u32 {
        self.rx_counter
    }
}

/// Persistent device relationship and security state.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceBackup {
    ieee_address: IeeeAddress,
    nwk_address: Option<Device>,
    is_child: bool,
    link_key: Option<LinkKeyState>,
    nwk_rx_counter: Option<u32>,
}

impl DeviceBackup {
    /// Construct persistent device relationship and security state.
    #[must_use]
    pub const fn new(
        ieee_address: IeeeAddress,
        nwk_address: Option<Device>,
        is_child: bool,
        link_key: Option<LinkKeyState>,
        nwk_rx_counter: Option<u32>,
    ) -> Self {
        Self {
            ieee_address,
            nwk_address,
            is_child,
            link_key,
            nwk_rx_counter,
        }
    }
    /// Remote device IEEE address.
    #[must_use]
    pub const fn ieee_address(&self) -> IeeeAddress {
        self.ieee_address
    }
    /// Last known allocated short address, when known.
    #[must_use]
    pub const fn nwk_address(&self) -> Option<Device> {
        self.nwk_address
    }
    /// Whether this device is a direct coordinator child.
    #[must_use]
    pub const fn is_child(&self) -> bool {
        self.is_child
    }
    /// Individual link-key state, when present.
    #[must_use]
    pub const fn link_key(&self) -> Option<&LinkKeyState> {
        self.link_key.as_ref()
    }
    /// Incoming network-key replay counter, when maintained by the stack; other key epochs require an extension.
    #[must_use]
    pub const fn nwk_rx_counter(&self) -> Option<u32> {
        self.nwk_rx_counter
    }
}

/// Required stack-specific core network state.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone, Eq, PartialEq)]
pub struct BackupExtension {
    namespace: String,
    name: String,
    version: u32,
    payload: Vec<u8>,
}

impl BackupExtension {
    /// Construct required stack-specific core network state.
    ///
    /// # Errors
    /// Returns an error if the supplied state is inconsistent.
    pub fn new(
        namespace: String,
        name: String,
        version: u32,
        payload: Vec<u8>,
    ) -> Result<Self, BackupError> {
        let value = Self {
            namespace,
            name,
            version,
            payload,
        };
        value.validate()?;
        Ok(value)
    }
    /// Stable stack or vendor namespace.
    #[must_use]
    pub fn namespace(&self) -> &str {
        &self.namespace
    }
    /// Extension identifier within its namespace.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Extension representation version.
    #[must_use]
    pub const fn version(&self) -> u32 {
        self.version
    }
    /// Secret-bearing payload, interpreted only by compatible backends.
    #[must_use]
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }
    fn validate(&self) -> Result<(), BackupError> {
        if self.namespace.trim().is_empty() || self.name.trim().is_empty() || self.version == 0 {
            return Err(BackupError::InvalidExtension);
        }
        Ok(())
    }
}

impl Debug for BackupExtension {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BackupExtension")
            .field("namespace", &self.namespace)
            .field("name", &self.name)
            .field("version", &self.version)
            .field("payload", &"[REDACTED]")
            .finish()
    }
}

/// Complete core network snapshot; no application-level state.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NetworkBackup {
    version: u32,
    source: BackupSource,
    network: NetworkParameters,
    network_key: NetworkKeyState,
    additional_network_keys: Vec<NetworkKeyState>,
    devices: Vec<DeviceBackup>,
    extensions: Vec<BackupExtension>,
}

impl NetworkBackup {
    /// Construct complete core network snapshot; no application-level state.
    ///
    /// # Errors
    /// Returns an error if the supplied state is inconsistent.
    pub fn new(
        source: BackupSource,
        network: NetworkParameters,
        network_key: NetworkKeyState,
        additional_network_keys: Vec<NetworkKeyState>,
        devices: Vec<DeviceBackup>,
        extensions: Vec<BackupExtension>,
    ) -> Result<Self, BackupError> {
        let value = Self {
            version: NETWORK_BACKUP_VERSION,
            source,
            network,
            network_key,
            additional_network_keys,
            devices,
            extensions,
        };
        value.validate()?;
        Ok(value)
    }
    /// Source stack and firmware.
    #[must_use]
    pub const fn source(&self) -> &BackupSource {
        &self.source
    }
    /// Network identity and parameters.
    #[must_use]
    pub const fn network(&self) -> &NetworkParameters {
        &self.network
    }
    /// Active network key state.
    #[must_use]
    pub const fn network_key(&self) -> &NetworkKeyState {
        &self.network_key
    }
    /// Other retained or staged network keys; transition semantics require an extension.
    #[must_use]
    pub fn additional_network_keys(&self) -> &[NetworkKeyState] {
        &self.additional_network_keys
    }
    /// Persistent remote device records.
    #[must_use]
    pub fn devices(&self) -> &[DeviceBackup] {
        &self.devices
    }
    /// Required extensions; unknown extensions must not be silently ignored.
    #[must_use]
    pub fn extensions(&self) -> &[BackupExtension] {
        &self.extensions
    }
    /// Validate structural consistency independently of a target backend.
    ///
    /// # Errors
    /// Returns an error for duplicate addresses, key sequences, or extension identifiers.
    pub fn validate(&self) -> Result<(), BackupError> {
        if self.version != NETWORK_BACKUP_VERSION {
            return Err(BackupError::UnsupportedVersion(self.version));
        }
        self.source.validate()?;
        self.network.validate()?;
        let mut ieee = BTreeSet::new();
        let mut short = BTreeSet::new();
        for device in &self.devices {
            if device.ieee_address == self.network.coordinator_ieee {
                return Err(BackupError::CoordinatorInDeviceTable);
            }
            if !ieee.insert(device.ieee_address)
                || device
                    .nwk_address
                    .is_some_and(|address| !short.insert(address))
            {
                return Err(BackupError::DuplicateDevice);
            }
        }
        let mut sequences = BTreeSet::from([self.network_key.sequence_number]);
        for key in &self.additional_network_keys {
            if !sequences.insert(key.sequence_number) {
                return Err(BackupError::DuplicateKeySequence);
            }
        }
        let mut extensions = BTreeSet::new();
        for extension in &self.extensions {
            extension.validate()?;
            if !extensions.insert((&extension.namespace, &extension.name)) {
                return Err(BackupError::DuplicateExtension);
            }
        }
        Ok(())
    }
}

impl NetworkBackup {
    /// Serialized schema version.
    #[must_use]
    pub const fn version(&self) -> u32 {
        self.version
    }

    /// Prepare a copy with advanced outgoing counters before any hardware writes.
    /// Incoming replay counters and key material are preserved.
    ///
    /// # Errors
    /// Returns an error if any retained network or link-key counter would overflow.
    pub fn prepare_restore(&self, options: RestoreOptions) -> Result<Self, BackupError> {
        let mut prepared = self.clone();
        prepared.network_key.tx_counter = advance(
            prepared.network_key.tx_counter,
            options.network_counter_advance,
        )?;
        for key in &mut prepared.additional_network_keys {
            key.tx_counter = advance(key.tx_counter, options.network_counter_advance)?;
        }
        for device in &mut prepared.devices {
            if let Some(key) = &mut device.link_key {
                key.tx_counter = advance(key.tx_counter, options.link_counter_advance)?;
            }
        }
        Ok(prepared)
    }
}

/// Explicit outgoing counter advances for a bounded-age backup.
///
/// There is deliberately no default. The caller must bound traffic since the backup, including
/// earlier restores. A finite advance cannot make an arbitrarily stale backup safe. Counters in
/// opaque extensions must be advanced consistently by the backend before writing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RestoreOptions {
    network_counter_advance: NonZeroU32,
    link_counter_advance: NonZeroU32,
}

impl RestoreOptions {
    /// Select explicit advances for every network-key and link-key outgoing counter.
    #[must_use]
    pub const fn new(
        network_counter_advance: NonZeroU32,
        link_counter_advance: NonZeroU32,
    ) -> Self {
        Self {
            network_counter_advance,
            link_counter_advance,
        }
    }

    /// Requested network-key counter advance.
    #[must_use]
    pub const fn network_counter_advance(self) -> NonZeroU32 {
        self.network_counter_advance
    }

    /// Requested link-key counter advance.
    #[must_use]
    pub const fn link_counter_advance(self) -> NonZeroU32 {
        self.link_counter_advance
    }
}

/// Registered actor operations. Target compatibility must still be checked for each backup.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BackupCapabilities {
    dump: bool,
    restore: bool,
}

impl BackupCapabilities {
    #[cfg(feature = "driver")]
    pub(crate) const fn new(dump: bool, restore: bool) -> Self {
        Self { dump, restore }
    }

    /// Whether network dumping was registered for this actor.
    #[must_use]
    pub const fn can_dump(self) -> bool {
        self.dump
    }

    /// Whether network restoration was registered for this actor.
    #[must_use]
    pub const fn can_restore(self) -> bool {
        self.restore
    }

    /// Schema version understood by this API. Extensions require target-specific validation.
    #[must_use]
    pub const fn version(self) -> u32 {
        NETWORK_BACKUP_VERSION
    }
}

/// Verified restored identity and outgoing counters. Contains no key material or extension payloads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RestoreReport {
    coordinator_ieee: IeeeAddress,
    network_counters: Vec<(u8, u32)>,
    link_counters: Vec<(IeeeAddress, u32)>,
}

impl RestoreReport {
    /// Describe state verified against a prepared backup after persistence.
    /// Backends must verify hardware state before constructing this report.
    #[must_use]
    pub fn from_verified_backup(backup: &NetworkBackup) -> Self {
        Self {
            coordinator_ieee: backup.network.coordinator_ieee,
            network_counters: std::iter::once(&backup.network_key)
                .chain(&backup.additional_network_keys)
                .map(|key| (key.sequence_number, key.tx_counter))
                .collect(),
            link_counters: backup
                .devices
                .iter()
                .filter_map(|device| {
                    device
                        .link_key
                        .as_ref()
                        .map(|key| (device.ieee_address, key.tx_counter))
                })
                .collect(),
        }
    }

    /// Verified coordinator IEEE address.
    #[must_use]
    pub const fn coordinator_ieee(&self) -> IeeeAddress {
        self.coordinator_ieee
    }

    /// Verified network key sequence numbers and outgoing counters.
    #[must_use]
    pub fn network_counters(&self) -> &[(u8, u32)] {
        &self.network_counters
    }

    /// Verified link-key partner addresses and outgoing counters.
    #[must_use]
    pub fn link_counters(&self) -> &[(IeeeAddress, u32)] {
        &self.link_counters
    }
}

fn advance(counter: u32, increment: NonZeroU32) -> Result<u32, BackupError> {
    counter
        .checked_add(increment.get())
        .filter(|value| *value < u32::MAX)
        .ok_or(BackupError::CounterExhausted)
}
