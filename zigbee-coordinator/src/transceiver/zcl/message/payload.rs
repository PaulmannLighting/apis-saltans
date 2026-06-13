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
    ///
    /// # Safety
    ///
    /// The caller must ensure that the metadata and command match.
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new(
        metadata: Metadata,
        manufacturer_code: Option<u16>,
        command: T,
    ) -> Self {
        Self {
            metadata,
            manufacturer_code,
            command,
        }
    }

    /// Create a new frame with no manufacturer code.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the metadata and command match.
    #[expect(unsafe_code)]
    #[must_use]
    pub const unsafe fn new_native(metadata: Metadata, command: T) -> Self {
        Self {
            metadata,
            manufacturer_code: None,
            command,
        }
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
        Payload {
            metadata: self.metadata,
            manufacturer_code: self.manufacturer_code,
            command: self.command.into(),
        }
    }
}
