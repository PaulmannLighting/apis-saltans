use apis_saltans_zcl::general::basic::readable::Id;
use apis_saltans_core::{Address, Application};

use super::Device;
use crate::ReadAttributeResult;

/// Message sent to the attribute discovery actor.
#[derive(Debug)]
pub enum Message {
    /// Get the attributes for the given endpoints.
    GetAttributes(Device),

    /// Attributes have been discovered.
    AttributesDiscovered {
        /// The device that has been discovered.
        address: Address,
        /// The application endpoint that has been discovered.
        application: Application,
        /// The attribute results.
        results: Box<[ReadAttributeResult<Id>]>,
    },

    /// Discovery of the given device has failed.
    DiscoveryFailed(Address),
}
