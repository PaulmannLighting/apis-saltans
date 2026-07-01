use apis_saltans_core::{Address, Endpoint};
use apis_saltans_zcl::Cluster;

/// A generic Zigbee event.
#[derive(Clone, Debug)]
pub struct Event {
    src_address: Address,
    src_endpoint: Endpoint,
    command: Cluster,
}

impl Event {
    /// Create a new event.
    #[must_use]
    pub(crate) const fn new(
        src_address: Address,
        src_endpoint: Endpoint,
        command: Cluster,
    ) -> Self {
        Self {
            src_address,
            src_endpoint,
            command,
        }
    }

    /// Return the source address of the event.
    #[must_use]
    pub const fn src_address(&self) -> &Address {
        &self.src_address
    }

    /// Return the source endpoint of the event.
    #[must_use]
    pub const fn src_endpoint(&self) -> Endpoint {
        self.src_endpoint
    }

    /// Return the command of the event.
    #[must_use]
    pub const fn command(&self) -> &Cluster {
        &self.command
    }

    /// Return the parts of the event.
    #[must_use]
    pub fn into_parts(self) -> (Address, Endpoint, Cluster) {
        (self.src_address, self.src_endpoint, self.command)
    }
}
