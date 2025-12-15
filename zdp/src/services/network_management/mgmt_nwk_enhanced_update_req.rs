mod enhanced_update_parameters;

use std::fmt::Display;
use std::iter::once;

use le_stream::{FromLeStream, ToLeStream};
use zigbee::Cluster;
use zigbee::types::ChannelList;

pub use self::enhanced_update_parameters::EnhancedNwkUpdateParameters;
use crate::{ScanDuration, Service};

/// Management Network Enhanced Update Request.
#[derive(Clone, Debug, Eq, PartialEq, Hash, FromLeStream, ToLeStream)]
pub struct MgmtNwkEnhancedUpdateReq {
    scan_channels: ChannelList,
    scan_duration: u8,
    scan_count_or_nwk_update_id: u8,
    nwk_manager_addr: Option<u16>,
}

impl MgmtNwkEnhancedUpdateReq {
    /// Creates a new `MgmtNwkEnhancedUpdateReq`.
    #[must_use]
    pub fn new(parameters: EnhancedNwkUpdateParameters) -> Self {
        let scan_duration = parameters.discriminant();

        match parameters {
            EnhancedNwkUpdateParameters::Zero {
                channel,
                energy_scan,
            }
            | EnhancedNwkUpdateParameters::One {
                channel,
                energy_scan,
            }
            | EnhancedNwkUpdateParameters::Two {
                channel,
                energy_scan,
            }
            | EnhancedNwkUpdateParameters::Three {
                channel,
                energy_scan,
            }
            | EnhancedNwkUpdateParameters::Four {
                channel,
                energy_scan,
            }
            | EnhancedNwkUpdateParameters::Five {
                channel,
                energy_scan,
            } => Self {
                scan_channels: ChannelList::new(once(channel).collect()),
                scan_duration,
                scan_count_or_nwk_update_id: u8::from(energy_scan),
                nwk_manager_addr: None,
            },
            EnhancedNwkUpdateParameters::ChannelChange {
                scan_channels,
                nwk_update_id,
            } => Self {
                scan_channels,
                scan_duration,
                scan_count_or_nwk_update_id: nwk_update_id,
                nwk_manager_addr: None,
            },
            EnhancedNwkUpdateParameters::AttributeChange {
                scan_channels,
                nwk_update_id,
                nwk_manager_addr,
            } => Self {
                scan_channels,
                scan_duration,
                scan_count_or_nwk_update_id: nwk_update_id,
                nwk_manager_addr: Some(nwk_manager_addr),
            },
        }
    }

    /// Returns the scan channels.
    #[must_use]
    pub const fn scan_channels(&self) -> &ChannelList {
        &self.scan_channels
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

impl Cluster for MgmtNwkEnhancedUpdateReq {
    const ID: u16 = 0x0039;
}

impl Service for MgmtNwkEnhancedUpdateReq {
    const NAME: &'static str = "Mgmt_NWK_Enhanced_Update_req";
}

impl Display for MgmtNwkEnhancedUpdateReq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.scan_duration() {
            Ok(scan_duration) => {
                write!(
                    f,
                    "{} {{ scan_channels: {}, scan_duration: {scan_duration} }}",
                    Self::NAME,
                    self.scan_channels,
                )
            }
            Err(invalid_scan_duration) => {
                write!(
                    f,
                    "{} {{ scan_channels: {}, invalid_scan_duration: {invalid_scan_duration:#04X} }}",
                    Self::NAME,
                    self.scan_channels,
                )
            }
        }
    }
}
