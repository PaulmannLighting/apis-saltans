use tokio::sync::oneshot;
use zb_core::IeeeAddress;
use zb_core::destination::Device;

use super::UpdateResult;
use super::image::ImageTransfer;

/// An image and completion channel registered for one target endpoint.
#[derive(Debug)]
pub(super) struct ScheduledUpdate {
    pub(super) transfer: ImageTransfer,
    pub(super) completion: oneshot::Sender<UpdateResult>,
    pub(super) generation: u64,
}

/// Validated addressing metadata for an inbound OTA request.
#[derive(Clone, Copy, Debug)]
pub(super) struct RequestContext {
    pub(super) destination: Device,
    pub(super) source_ieee_address: Option<IeeeAddress>,
    pub(super) sequence_number: u8,
}
