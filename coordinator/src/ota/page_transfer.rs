use std::time::Duration;

use log::warn;
use tokio::sync::mpsc::{Sender, UnboundedSender};
use tokio::time::sleep;
use zb_core::destination::Device;
use zb_zcl::ota_upgrade::{ImageBlock, ImageBlockResponse, ImageBlockResponsePayload, ImageId};

use super::image::ImageTransfer;
use super::transfer::{TransferEvent, TransferKey, report_failure};
use super::{OTA_PROFILE, Payload, UpdateError, read_image_range, reply_zcl, zcl};

/// State owned by a paced OTA Image Page transfer task.
///
/// The task sends consecutive Image Block responses without blocking the server actor and reports
/// its first read or transmission failure through `events`.
pub(super) struct PageTransfer {
    pub(super) zcl: Sender<zcl::Message>,
    pub(super) image: ImageTransfer,
    pub(super) events: UnboundedSender<TransferEvent>,
    pub(super) key: TransferKey,
    pub(super) destination: Device,
    pub(super) image_id: ImageId,
    pub(super) maximum_data_size: usize,
    pub(super) page_end: usize,
    pub(super) spacing: Duration,
    pub(super) offset: usize,
    pub(super) sequence_number: u8,
    pub(super) block_data: Box<[u8]>,
}

impl PageTransfer {
    /// Send all remaining blocks in the requested page, respecting response spacing.
    pub(super) async fn run(mut self) {
        loop {
            let file_offset =
                u32::try_from(self.offset).expect("validated OTA image offset fits u32");
            let block = ImageBlock::try_new(self.image_id, file_offset, self.block_data)
                .expect("requested OTA blocks never exceed the client's u8 maximum data size");
            let response = ImageBlockResponse::new(ImageBlockResponsePayload::Success(block));
            let Some(hw_response) = reply_zcl(
                &self.zcl,
                self.destination,
                OTA_PROFILE,
                self.sequence_number,
                Payload::from(response).with_aps_acknowledgement(false),
            )
            .await
            else {
                report_failure(&self.events, self.key, UpdateError::Transmission);
                return;
            };
            if let Err(error) = hw_response.await {
                warn!("OTA page transmission failed: {error}");
                report_failure(&self.events, self.key, UpdateError::Transmission);
                return;
            }

            self.offset = self.offset.saturating_add(self.maximum_data_size);
            if self.offset >= self.page_end {
                return;
            }
            sleep(self.spacing).await;
            self.sequence_number = self.sequence_number.wrapping_add(1);
            let block_end = self
                .offset
                .saturating_add(self.maximum_data_size)
                .min(self.page_end);
            self.block_data =
                match read_image_range(&self.image, self.offset, block_end - self.offset).await {
                    Ok(data) => data,
                    Err(status) => {
                        warn!("Failed to read OTA page data: {status}");
                        report_failure(&self.events, self.key, UpdateError::ImageTransfer);
                        return;
                    }
                };
        }
    }
}
