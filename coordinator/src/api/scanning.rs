pub use zb_hw::{
    Channel, ChannelMask, FoundNetwork, NetworkDescriptor, ScanDuration, ScannedChannel,
};

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
        channel_mask: ChannelMask,
        duration: ScanDuration,
    ) -> impl Future<Output = Result<Vec<FoundNetwork>, Error>> + Send;

    /// Scan channels and return energy/channel observations.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the hardware scan request fails.
    fn scan_channels(
        &self,
        channel_mask: ChannelMask,
        duration: ScanDuration,
    ) -> impl Future<Output = Result<Vec<ScannedChannel>, Error>> + Send;
}

impl Scanning for Coordinator {
    async fn scan_networks(
        &self,
        channel_mask: ChannelMask,
        duration: ScanDuration,
    ) -> Result<Vec<FoundNetwork>, Error> {
        Ok(self.ncp.scan_networks(channel_mask, duration).await?)
    }

    async fn scan_channels(
        &self,
        channel_mask: ChannelMask,
        duration: ScanDuration,
    ) -> Result<Vec<ScannedChannel>, Error> {
        Ok(self.ncp.scan_channels(channel_mask, duration).await?)
    }
}
