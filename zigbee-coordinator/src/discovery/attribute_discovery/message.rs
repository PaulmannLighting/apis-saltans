use std::collections::BTreeMap;

use zcl::general::basic::readable::Id;
use zdp::SimpleDescriptor;
use zigbee::{Address, Application, Endpoint};

use crate::ReadAttributeResult;

/// Message sent to the attribute discovery actor.
#[derive(Debug)]
pub enum Message {
    /// Get the attributes for the given endpoints.
    GetAttributes {
        /// The device to get the attributes for.
        address: Address,
        /// The endpoints to get the attributes for.
        endpoints: BTreeMap<Endpoint, SimpleDescriptor>,
    },

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
