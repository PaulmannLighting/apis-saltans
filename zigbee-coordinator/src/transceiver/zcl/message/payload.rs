use zcl::Cluster;
use zigbee_hw::Metadata;

/// A ZCL over ZDP frame payload.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Payload {
    /// APS metadata for transmission.
    metadata: Metadata,
    /// An optional manufacturer code.
    manufacturer_code: Option<u16>,
    /// ZCL payload.
    command: Cluster,
}

impl Payload {
    /// Create a new payload.
    #[must_use]
    pub const fn new(metadata: Metadata, manufacturer_code: Option<u16>, command: Cluster) -> Self {
        Self {
            metadata,
            manufacturer_code,
            command,
        }
    }

    /// Create a new payload with no manufacturer code.
    #[must_use]
    pub const fn new_native(metadata: Metadata, command: Cluster) -> Self {
        Self {
            metadata,
            manufacturer_code: None,
            command,
        }
    }

    /// Retrieve the APS metadata.
    #[must_use]
    pub const fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Retrieve the manufacturer code.
    #[must_use]
    pub const fn manufacturer_code(&self) -> Option<u16> {
        self.manufacturer_code
    }

    /// Retrieve the ZCL command.
    #[must_use]
    pub const fn command(&self) -> &Cluster {
        &self.command
    }

    /// Consume the payload into its parts.
    #[must_use]
    pub fn into_parts(self) -> (Metadata, Option<u16>, Cluster) {
        (self.metadata, self.manufacturer_code, self.command)
    }
}
