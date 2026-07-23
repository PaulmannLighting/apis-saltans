use zb_core::IeeeAddress;
use zb_core::destination::Device;

/// Validated addressing metadata for an inbound OTA request.
#[derive(Clone, Copy, Debug)]
pub(super) struct RequestContext {
    pub(super) destination: Device,
    pub(super) source_ieee_address: Option<IeeeAddress>,
    pub(super) sequence_number: u8,
}
