use std::fmt::Display;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::Cluster;

pub use self::scan_duration::ScanDuration;
use crate::Service;

mod scan_duration;

/// Management Network Update Request
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct MgmtNwkUpdateReq {
    scan_channels: u32,
    scan_duration: u8,
    scan_count_or_nwk_update_id: u8,
    nwk_manager_addr: Option<u16>,
}

impl MgmtNwkUpdateReq {
    /// Creates a new `MgmtNwkUpdateReq`.
    #[must_use]
    pub const fn new(scan_channels: u32, scan_duration: ScanDuration) -> Self {
        match scan_duration {
            ScanDuration::Zero { energy_scan }
            | ScanDuration::One { energy_scan }
            | ScanDuration::Two { energy_scan }
            | ScanDuration::Three { energy_scan }
            | ScanDuration::Four { energy_scan }
            | ScanDuration::Five { energy_scan } => Self {
                scan_channels,
                scan_duration: scan_duration.discriminant(),
                scan_count_or_nwk_update_id: if energy_scan { 0x01 } else { 0x00 },
                nwk_manager_addr: None,
            },
            ScanDuration::ChannelChange { nwk_update_id } => Self {
                scan_channels,
                scan_duration: scan_duration.discriminant(),
                scan_count_or_nwk_update_id: nwk_update_id,
                nwk_manager_addr: None,
            },
            ScanDuration::AttributeChange {
                nwk_update_id,
                nwk_manager_addr,
            } => Self {
                scan_channels,
                scan_duration: scan_duration.discriminant(),
                scan_count_or_nwk_update_id: nwk_update_id,
                nwk_manager_addr: Some(nwk_manager_addr),
            },
        }
    }

    /// Returns the scan channels.
    #[must_use]
    pub const fn scan_channels(&self) -> u32 {
        self.scan_channels
    }

    /// Returns the scan duration.
    ///
    /// # Errors
    ///
    /// When an invalid scan duration was specified, it returns the appropriate [`u8`] value in the `Err` variant.
    ///
    /// In the case of `0xFF`, this indicates that the required `nwk_manager_addr` field was not set.
    pub const fn scan_duration(&self) -> Result<ScanDuration, u8> {
        match self.scan_duration {
            0x00 => Ok(ScanDuration::Zero {
                energy_scan: self.scan_count_or_nwk_update_id != 0,
            }),
            0x01 => Ok(ScanDuration::One {
                energy_scan: self.scan_count_or_nwk_update_id != 0,
            }),
            0x02 => Ok(ScanDuration::Two {
                energy_scan: self.scan_count_or_nwk_update_id != 0,
            }),
            0x03 => Ok(ScanDuration::Three {
                energy_scan: self.scan_count_or_nwk_update_id != 0,
            }),
            0x04 => Ok(ScanDuration::Four {
                energy_scan: self.scan_count_or_nwk_update_id != 0,
            }),
            0x05 => Ok(ScanDuration::Five {
                energy_scan: self.scan_count_or_nwk_update_id != 0,
            }),
            0xFE => Ok(ScanDuration::ChannelChange {
                nwk_update_id: self.scan_count_or_nwk_update_id,
            }),
            scan_duration @ 0xFF => {
                if let Some(nwk_manager_addr) = self.nwk_manager_addr {
                    Ok(ScanDuration::AttributeChange {
                        nwk_update_id: self.scan_count_or_nwk_update_id,
                        nwk_manager_addr,
                    })
                } else {
                    Err(scan_duration)
                }
            }
            invalid_scan_duration => Err(invalid_scan_duration),
        }
    }
}

impl Cluster for MgmtNwkUpdateReq {
    const ID: u16 = 0x0038;
}

impl Service for MgmtNwkUpdateReq {
    const NAME: &'static str = "Mgmt_NWK_Update_req";
}

impl Display for MgmtNwkUpdateReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.scan_duration() {
            Ok(scan_duration) => {
                write!(
                    f,
                    "{} {{ scan_channels: {:#010X}, scan_duration: {scan_duration} }}",
                    Self::NAME,
                    self.scan_channels,
                )
            }
            Err(invalid_scan_duration) => {
                write!(
                    f,
                    "{} {{ scan_channels: {:#010X}, invalid_scan_duration: {:#04X} }}",
                    Self::NAME,
                    self.scan_channels,
                    invalid_scan_duration
                )
            }
        }
    }
}
