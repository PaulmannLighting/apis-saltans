use std::num::NonZeroUsize;
use std::ops::Range;
use std::time::Duration;

use log::warn;
use tokio::sync::mpsc::Sender;
use tokio::time::sleep;
use zb_aps::TxOptions;
use zb_core::Cluster;
use zb_zcl::ota_upgrade::{
    ImageBlock, ImageBlockResponse, ImageBlockResponsePayload, ImagePageRequest,
};

use super::image::ImageTransfer;
use super::state::RequestContext;
use super::transfer::read_image_range;
use super::zcl::Message;
use super::{OTA_PROFILE, UpdateError, UpdateResult, reply_zcl, request};

const SEQUENCE_STEP: u8 = 1;

/// Validated page bounds and the position of its next block.
pub(super) struct PageProgress {
    remaining: Range<usize>,
    maximum_data_size: NonZeroUsize,
    spacing: Duration,
}

impl PageProgress {
    /// Validate page dimensions and clip the requested page to the image length.
    pub(super) fn new(request: &ImagePageRequest, image_length: usize) -> Option<Self> {
        let offset = usize::try_from(request.file_offset()).ok()?;
        let page_size = NonZeroUsize::new(usize::from(request.page_size()))?;
        let maximum_data_size = NonZeroUsize::new(usize::from(request.maximum_data_size()))?;
        if offset >= image_length {
            return None;
        }
        Some(Self {
            remaining: offset..offset.saturating_add(page_size.get()).min(image_length),
            maximum_data_size,
            spacing: Duration::from_millis(u64::from(request.response_spacing())),
        })
    }

    /// Current block, clipped to both the page and image boundary.
    fn block_range(&self) -> Range<usize> {
        let end = self
            .remaining
            .start
            .saturating_add(self.maximum_data_size.get());
        self.remaining.start..end.min(self.remaining.end)
    }

    /// Advance past the current block, returning whether another block remains.
    fn advance(&mut self) -> bool {
        self.remaining.start = self.block_range().end;
        !self.remaining.is_empty()
    }
}

/// Sends paced Image Block responses for one validated page request.
pub(super) struct PageTransfer {
    zcl: Sender<Message>,
    image: ImageTransfer,
    context: RequestContext,
    progress: PageProgress,
}

impl PageTransfer {
    /// Retain the request's addressing and validated page progress.
    pub(super) const fn new(
        zcl: Sender<Message>,
        image: ImageTransfer,
        context: RequestContext,
        progress: PageProgress,
    ) -> Self {
        Self {
            zcl,
            image,
            context,
            progress,
        }
    }

    /// Send the already-read first block and remaining blocks, respecting response spacing.
    pub(super) async fn run(mut self, mut block_data: Box<[u8]>) -> UpdateResult {
        loop {
            self.send_block(block_data).await?;
            if !self.progress.advance() {
                return Ok(());
            }
            sleep(self.progress.spacing).await;
            self.context.sequence_number = self.context.sequence_number.wrapping_add(SEQUENCE_STEP);
            let range = self.progress.block_range();
            block_data = read_image_range(&self.image, range.start, range.len())
                .await
                .map_err(|status| {
                    warn!("Failed to read OTA page data: {status}");
                    UpdateError::ImageTransfer
                })?;
        }
    }

    /// Submit one block without requesting an APS acknowledgement.
    async fn send_block(&self, data: Box<[u8]>) -> UpdateResult {
        let file_offset = u32::try_from(self.progress.block_range().start)
            .expect("validated OTA image offset fits u32");
        let block = ImageBlock::try_new(self.image.id(), file_offset, data)
            .expect("requested OTA blocks never exceed the client's u8 maximum data size");
        let response = ImageBlockResponse::new(ImageBlockResponsePayload::Success(block));
        let request = request(
            self.context.destination.into(),
            self.context.source_endpoint,
            OTA_PROFILE,
            Cluster::OtaUpgrade.as_u16(),
            response,
        )
        .with_tx_options(TxOptions::empty());
        reply_zcl(&self.zcl, self.context.sequence_number, request)
            .await
            .ok_or(UpdateError::Transmission)
    }
}
