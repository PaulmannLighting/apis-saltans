use serde::{Deserialize, Serialize};
use zdp::SimpleDescriptor;

use super::Attributes;

/// Information about an endpoint.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Endpoint {
    descriptor: SimpleDescriptor,
    attributes: Attributes,
}

impl Endpoint {
    /// Create a new instance of `Endpoint`.
    #[must_use]
    pub const fn new(descriptor: SimpleDescriptor, attributes: Attributes) -> Self {
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
    pub const fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    /// Consume the endpoint info, returning the descriptor and attributes.
    #[must_use]
    pub fn into_parts(self) -> (SimpleDescriptor, Attributes) {
        (self.descriptor, self.attributes)
    }
}
