use zb_hw::Ncp;
pub use zb_hw::{FoundNetwork, ScannedChannel};

use crate::{Coordinator, Error};

/// Trait for active Zigbee network and channel scanning.
///
/// Scans are delegated to the hardware/NCP. The channel mask and duration use the same
/// interpretation as the underlying hardware API.
pub trait Scanning {
    /// Scan for joinable Zigbee networks.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the hardware scan request fails.
    fn scan_networks(
        &self,
        channel_mask: u32,
        duration: u8,
    ) -> impl Future<Output = Result<Vec<FoundNetwork>, Error>> + Send;

    /// Scan channels and return energy/channel observations.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the hardware scan request fails.
    fn scan_channels(
        &self,
        channel_mask: u32,
        duration: u8,
    ) -> impl Future<Output = Result<Vec<ScannedChannel>, Error>> + Send;
}

impl Scanning for Coordinator {
    async fn scan_networks(
        &self,
        channel_mask: u32,
        duration: u8,
    ) -> Result<Vec<FoundNetwork>, Error> {
        Ok(Ncp::scan_networks(&self.ncp, channel_mask, duration).await?)
    }

    async fn scan_channels(
        &self,
        channel_mask: u32,
        duration: u8,
    ) -> Result<Vec<ScannedChannel>, Error> {
        Ok(Ncp::scan_channels(&self.ncp, channel_mask, duration).await?)
    }
}
