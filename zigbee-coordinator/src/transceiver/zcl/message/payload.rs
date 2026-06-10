//! General-purpose APS frame.

use zcl::Cluster;
use zigbee_hw::Metadata;

/// A simplified APS frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Payload<T> {
    /// APS metadata for transmission.
    metadata: Metadata,
    /// An optional manufacturer code.
    manufacturer_code: Option<u16>,
    /// Command payload.
    command: T,
}

impl<T> Payload<T> {
    /// Create a new frame.
    #[must_use]
    pub const fn new(metadata: Metadata, manufacturer_code: Option<u16>, command: T) -> Self {
        Self {
            metadata,
            manufacturer_code,
            command,
        }
    }

    /// Create a new frame with no manufacturer code.
    #[must_use]
    pub const fn new_native(metadata: Metadata, command: T) -> Self {
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

    /// Retrieve the command payload.
    #[must_use]
    pub const fn command(&self) -> &T {
        &self.command
    }

    /// Consume the frame into its parts.
    #[must_use]
    pub fn into_parts(self) -> (Metadata, Option<u16>, T) {
        (self.metadata, self.manufacturer_code, self.command)
    }
}

impl<T> Payload<T>
where
    T: Into<Cluster>,
{
    /// Convert the frame into a ZCL cluster frame.
    #[must_use]
    pub fn into_cluster(self) -> Payload<Cluster> {
        Payload::new(self.metadata, self.manufacturer_code, self.command.into())
    }
}
