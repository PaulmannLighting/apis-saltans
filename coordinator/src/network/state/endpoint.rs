use serde::{Deserialize, Serialize};
use zb_zdp::SimpleDescriptor;

use super::DeviceAttributes;

/// Information about an endpoint.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct EndpointInfo {
    descriptor: SimpleDescriptor,
    attributes: DeviceAttributes,
}

impl EndpointInfo {
    /// Create a new instance of `Endpoint`.
    #[must_use]
    pub const fn new(descriptor: SimpleDescriptor, attributes: DeviceAttributes) -> Self {
        Self {
            descriptor,
            attributes,
        }
    }

    /// Get the descriptor.
    #[must_use]
    pub const fn descriptor(&self) -> &SimpleDescriptor {
        &self.descriptor
    }

    /// Get the attributes.
    #[must_use]
    pub const fn attributes(&self) -> &DeviceAttributes {
        &self.attributes
    }

    /// Consume the endpoint info, returning the descriptor and attributes.
    #[must_use]
    pub fn into_parts(self) -> (SimpleDescriptor, DeviceAttributes) {
        (self.descriptor, self.attributes)
    }
}
