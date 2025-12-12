use std::fmt::Display;

use repr_discriminant::ReprDiscriminant;

/// Scan Duration enumeration.
///
/// For `Zero` to `Five`, the duration is calculated as `<const> * (2^n)` symbols.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, ReprDiscriminant)]
#[repr(u8)]
pub enum ScanDuration {
    /// Scan duration of `<const> * (2^0)` symbols.
    Zero {
        /// Perform an energy scan if `true`.
        energy_scan: bool,
    } = 0x00,
    /// Scan duration of `<const> * (2^1)` symbols.
    One {
        /// Perform an energy scan if `true`.
        energy_scan: bool,
    } = 0x01,
    /// Scan duration of `<const> * (2^2)` symbols.
    Two {
        /// Perform an energy scan if `true`.
        energy_scan: bool,
    } = 0x02,
    /// Scan duration of `<const> * (2^3)` symbols.
    Three {
        /// Perform an energy scan if `true`.
        energy_scan: bool,
    } = 0x03,
    /// Scan duration of `<const> * (2^4)` symbols.
    Four {
        /// Perform an energy scan if `true`.
        energy_scan: bool,
    } = 0x04,
    /// Scan duration of `<const> * (2^5)` symbols.
    Five {
        /// Perform an energy scan if `true`.
        energy_scan: bool,
    } = 0x05,
    /// Channel change request.
    ChannelChange {
        /// Network update ID.
        nwk_update_id: u8,
    } = 0xFE,
    /// `aps_channel_mask_list` and `nwk_manager_addr` attribute change request.
    AttributeChange {
        /// Network update ID.
        nwk_update_id: u8,
        /// Network manager address.
        nwk_manager_addr: u16,
    } = 0xFF,
}

impl Display for ScanDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanDuration::Zero { energy_scan } => {
                write!(f, "Zero {{ energy_scan: {energy_scan} }}")
            }
            ScanDuration::One { energy_scan } => {
                write!(f, "One {{ energy_scan: {energy_scan} }}")
            }
            ScanDuration::Two { energy_scan } => {
                write!(f, "Two {{ energy_scan: {energy_scan} }}")
            }
            ScanDuration::Three { energy_scan } => {
                write!(f, "Three {{ energy_scan: {energy_scan} }}")
            }
            ScanDuration::Four { energy_scan } => {
                write!(f, "Four {{ energy_scan: {energy_scan} }}")
            }
            ScanDuration::Five { energy_scan } => {
                write!(f, "Five {{ energy_scan: {energy_scan} }}")
            }
            ScanDuration::ChannelChange { nwk_update_id } => {
                write!(f, "ChannelChange {{ nwk_update_id: {nwk_update_id:#04X} }}")
            }
            ScanDuration::AttributeChange {
                nwk_update_id,
                nwk_manager_addr,
            } => {
                write!(
                    f,
                    "AttributeChange {{ nwk_update_id: {nwk_update_id:#04X}, nwk_manager_addr: {nwk_manager_addr:#06X} }}"
                )
            }
        }
    }
}
