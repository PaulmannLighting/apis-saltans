//! General-purpose APS frame.

use zdp::Command;
use zigbee::Cluster;
use zigbee_hw::Metadata;

/// A simplified APS frame.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Payload<T> {
    /// APS metadata for transmission.
    metadata: Metadata,
    /// Command payload.
    command: T,
}

impl<T> Payload<T> {
    /// Create a new frame.
    #[must_use]
    pub const fn new(metadata: Metadata, command: T) -> Self {
        Self { metadata, command }
    }

    /// Consume the frame into its parts.
    #[must_use]
    pub fn into_parts(self) -> (Metadata, T) {
        (self.metadata, self.command)
    }
}

impl<T> Payload<T>
where
    T: Cluster,
{
    /// Create a new frame for a ZDP command.
    #[must_use]
    pub const fn zdp(payload: T) -> Self {
        Self::new(Metadata::zdp(T::ID), payload)
    }
}

impl<T> Payload<T>
where
    T: Into<Command>,
{
    /// Convert the frame into a ZCL cluster frame.
    #[must_use]
    pub fn into_command(self) -> Payload<Command> {
        Payload::new(self.metadata, self.command.into())
    }
}

/// Convert a ZDP command into a frame.
impl From<Command> for Payload<Command> {
    fn from(command: Command) -> Self {
        Self::new(Metadata::zdp(command.cluster_id()), command)
    }
}
