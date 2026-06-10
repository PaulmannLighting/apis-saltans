//! General-purpose APS frame.

use zdp::Command;
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

    /// Retrieve the APS metadata.
    #[must_use]
    pub const fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Retrieve the command payload.
    #[must_use]
    pub const fn command(&self) -> &T {
        &self.command
    }

    /// Consume the frame into its parts.
    #[must_use]
    pub fn into_parts(self) -> (Metadata, T) {
        (self.metadata, self.command)
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
