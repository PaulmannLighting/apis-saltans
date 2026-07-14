use zb_aps::Data;
use zb_core::endpoint::Reserved;
use zb_core::{Endpoint, FullAddress};
use zb_zcl::{Cluster, Frame};

/// An unsolicited ZCL command received from a device.
///
/// The coordinator resolves the source to a [`FullAddress`] before publishing the event. The
/// source endpoint is retained even when its raw value is reserved and therefore cannot be
/// represented as an [`Endpoint`].
#[derive(Clone, Debug)]
pub struct Zcl {
    src_address: FullAddress,
    src_endpoint: Result<Endpoint, Reserved>,
    command: Cluster,
}

impl Zcl {
    pub(crate) fn new(src_address: FullAddress, aps: Data<Frame<Cluster>>) -> Self {
        let (aps_header, frame) = aps.into_parts();

        Self {
            src_address,
            src_endpoint: aps_header.source_endpoint(),
            command: frame.into_payload(),
        }
    }

    /// Returns the resolved source address.
    #[must_use]
    pub const fn src_address(&self) -> FullAddress {
        self.src_address
    }

    /// Returns the source endpoint.
    ///
    /// # Errors
    ///
    /// Returns the reserved raw endpoint when it is not a valid [`Endpoint`].
    pub const fn src_endpoint(&self) -> Result<Endpoint, Reserved> {
        self.src_endpoint
    }

    /// Consumes the event and returns the parsed ZCL command.
    #[must_use]
    pub fn into_command(self) -> Cluster {
        self.command
    }
}
