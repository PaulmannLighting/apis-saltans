use std::fmt::Display;

use repr_discriminant::ReprDiscriminant;
use zigbee::types::{ChannelList, ChannelsField};

/// Scan Duration enumeration.
///
/// For `Zero` to `Five`, the duration is calculated as `<const> * (2^n)` symbols.
#[derive(Clone, Debug, Eq, PartialEq, Hash, ReprDiscriminant)]
#[repr(u8)]
pub enum EnhancedNwkUpdateParameters {
    /// Scan duration of `<const> * (2^0)` symbols.
    Zero {
        /// The channel to scan.
        channel: ChannelsField,
        /// Perform an energy scan if `true`.
        energy_scan: bool,
    } = 0x00,
    /// Scan duration of `<const> * (2^1)` symbols.
    One {
        /// The channel to scan.
        channel: ChannelsField,
        /// Perform an energy scan if `true`.
        energy_scan: bool,
    } = 0x01,
    /// Scan duration of `<const> * (2^2)` symbols.
    Two {
        /// The channel to scan.
        channel: ChannelsField,
        /// Perform an energy scan if `true`.
        energy_scan: bool,
    } = 0x02,
    /// Scan duration of `<const> * (2^3)` symbols.
    Three {
        /// The channel to scan.
        channel: ChannelsField,
        /// Perform an energy scan if `true`.
        energy_scan: bool,
    } = 0x03,
    /// Scan duration of `<const> * (2^4)` symbols.
    Four {
        /// The channel to scan.
        channel: ChannelsField,
        /// Perform an energy scan if `true`.
        energy_scan: bool,
    } = 0x04,
    /// Scan duration of `<const> * (2^5)` symbols.
    Five {
        /// The channel to scan.
        channel: ChannelsField,
        /// Perform an energy scan if `true`.
        energy_scan: bool,
    } = 0x05,
    /// Channel change request.
    ChannelChange {
        /// The channels to scan.
        scan_channels: ChannelList,
        /// Network update ID.
        nwk_update_id: u8,
    } = 0xFE,
    /// `aps_channel_mask_list` and `nwk_manager_addr` attribute change request.
    AttributeChange {
        /// The channels to scan.
        scan_channels: ChannelList,
        /// Network update ID.
        nwk_update_id: u8,
        /// Network manager address.
        nwk_manager_addr: u16,
    } = 0xFF,
}

impl Display for EnhancedNwkUpdateParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zero {
                channel,
                energy_scan,
            } => {
                write!(
                    f,
                    "Zero {{ channel: {channel:#010X}, energy_scan: {energy_scan} }}"
                )
            }
            Self::One {
                channel,
                energy_scan,
            } => {
                write!(
                    f,
                    "One {{ channel: {channel:#010X}, energy_scan: {energy_scan} }}"
                )
            }
            Self::Two {
                channel,
                energy_scan,
            } => {
                write!(
                    f,
                    "Two {{ channel: {channel:#010X}, energy_scan: {energy_scan} }}"
                )
            }
            Self::Three {
                channel,
                energy_scan,
            } => {
                write!(
                    f,
                    "Three {{ channel: {channel:#010X}, energy_scan: {energy_scan} }}"
                )
            }
            Self::Four {
                channel,
                energy_scan,
            } => {
                write!(
                    f,
                    "Four {{ channel: {channel:#010X}, energy_scan: {energy_scan} }}"
                )
            }
            Self::Five {
                channel,
                energy_scan,
            } => {
                write!(
                    f,
                    "Five {{ channel: {channel:#010X}, energy_scan: {energy_scan} }}"
                )
            }
            Self::ChannelChange {
                scan_channels,
                nwk_update_id,
            } => {
                write!(
                    f,
                    "ChannelChange {{ channel: {scan_channels}, nwk_update_id: {nwk_update_id:#04X} }}"
                )
            }
            Self::AttributeChange {
                scan_channels,
                nwk_update_id,
                nwk_manager_addr,
            } => {
                write!(
                    f,
                    "AttributeChange {{ channel: {scan_channels}, nwk_update_id: {nwk_update_id:#04X}, nwk_manager_addr: {nwk_manager_addr:#06X} }}"
                )
            }
        }
    }
}
