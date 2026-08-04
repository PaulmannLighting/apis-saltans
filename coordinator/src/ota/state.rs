use zb_aps::apsde::{IndividualEndpoint, NetworkDestination};

/// Validated addressing metadata for an inbound OTA request.
#[derive(Clone, Copy, Debug)]
pub(super) struct RequestContext {
    pub(super) destination: NetworkDestination,
    pub(super) source_endpoint: IndividualEndpoint,
    pub(super) sequence_number: u8,
}
