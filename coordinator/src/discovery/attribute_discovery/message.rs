use zb_core::{Application, FullAddress};
use zb_zcl::basic::Id;

use super::IncomingDevice;
use crate::ReadAttributeResult;

/// Message sent to the attribute discovery actor.
#[derive(Debug)]
pub enum Message {
    /// Get the attributes for the given endpoints.
    GetAttributes(IncomingDevice),

    /// Attributes have been discovered.
    AttributesDiscovered {
        /// The device that has been discovered.
        address: FullAddress,
        /// The application endpoint that has been discovered.
        application: Application,
        /// The attribute results.
        results: Box<[ReadAttributeResult<Id>]>,
    },

    /// Discovery of the given device has failed.
    DiscoveryFailed(FullAddress),
}
