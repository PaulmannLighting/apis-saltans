use tokio::sync::oneshot;
use zb_core::destination::Device;
use zb_core::{IeeeAddress, Profile};

use super::UpdateResult;
use super::image::ImageTransfer;

#[derive(Debug)]
pub(super) struct ScheduledUpdate {
    pub(super) profile: Profile,
    pub(super) transfer: ImageTransfer,
    pub(super) completion: oneshot::Sender<UpdateResult>,
    pub(super) generation: u64,
}

#[derive(Clone, Copy, Debug)]
pub(super) struct RequestContext {
    pub(super) destination: Device,
    pub(super) profile: Profile,
    pub(super) source_ieee_address: Option<IeeeAddress>,
    pub(super) sequence_number: u8,
}
