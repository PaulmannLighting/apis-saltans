use zdp::SimpleDescriptor;

use super::Attributes;

/// Information about an endpoint.
#[derive(Debug)]
pub struct EndpointInfo {
    descriptor: SimpleDescriptor,
    attributes: Option<Attributes>,
}

impl EndpointInfo {
    /// Get the descriptor.
    #[must_use]
    pub const fn descriptor(&self) -> &SimpleDescriptor {
        &self.descriptor
    }

    /// Get the attributes.
    #[must_use]
    pub const fn attributes(&self) -> Option<&Attributes> {
        self.attributes.as_ref()
    }

    /// Set the attributes.
    pub const fn set_attributes(&mut self, attributes: Attributes) -> Option<Attributes> {
        self.attributes.replace(attributes)
    }

    /// Consume the endpoint info, returning the descriptor and attributes.
    #[must_use]
    pub fn into_parts(self) -> (SimpleDescriptor, Option<Attributes>) {
        (self.descriptor, self.attributes)
    }
}

impl From<SimpleDescriptor> for EndpointInfo {
    fn from(descriptor: SimpleDescriptor) -> Self {
        Self {
            descriptor,
            attributes: None,
        }
    }
}
