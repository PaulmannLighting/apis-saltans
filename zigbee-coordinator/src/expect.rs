use zcl::global::read_attributes;
use zcl::{Cluster, global};

/// Trait for expecting a specific ZCL command in a message.
pub trait ZclCommand<T> {
    /// Expect a specific ZCL command in a message.
    ///
    /// Returns `Some(command)` if the expected command is found, otherwise `None`.
    fn expect(self) -> Option<T>;
}

impl ZclCommand<read_attributes::Response> for Cluster {
    fn expect(self) -> Option<read_attributes::Response> {
        if let Self::Global(global::Command::ReadAttributesResponse(response)) = self {
            Some(response)
        } else {
            None
        }
    }
}
