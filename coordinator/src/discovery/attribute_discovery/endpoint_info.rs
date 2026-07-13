use zb_zdp::SimpleDescriptor;

use super::DeviceAttributes;

/// Information about an endpoint.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct EndpointInfo {
    descriptor: SimpleDescriptor,
    attributes: Option<DeviceAttributes>,
}

impl EndpointInfo {
    /// Get the descriptor.
    #[must_use]
    pub const fn descriptor(&self) -> &SimpleDescriptor {
        &self.descriptor
    }

    /// Get the attributes.
    #[must_use]
    pub const fn attributes(&self) -> Option<&DeviceAttributes> {
        self.attributes.as_ref()
    }

    /// Set the attributes.
    pub const fn set_attributes(
        &mut self,
        attributes: DeviceAttributes,
    ) -> Option<DeviceAttributes> {
        self.attributes.replace(attributes)
    }

    /// Consume the endpoint info, returning the descriptor and attributes.
    #[must_use]
    pub fn into_parts(self) -> (SimpleDescriptor, Option<DeviceAttributes>) {
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

impl From<EndpointInfo> for crate::EndpointInfo {
    fn from(endpoint_info: EndpointInfo) -> Self {
        Self::new(
            endpoint_info.descriptor,
            endpoint_info.attributes.unwrap_or_default(),
        )
    }
}
