use crate::{endpoint, short_id};

/// Broadcast destination with a broadcast short address and broadcast endpoint.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Broadcast {
    address: short_id::Broadcast,
    endpoint: endpoint::Broadcast,
}

impl Broadcast {
    /// Create a broadcast destination from a broadcast address and endpoint selector.
    #[must_use]
    pub const fn new(address: short_id::Broadcast, endpoint: endpoint::Broadcast) -> Self {
        Self { address, endpoint }
    }

    /// Return the destination broadcast short address.
    #[must_use]
    pub const fn address(&self) -> short_id::Broadcast {
        self.address
    }

    /// Return the destination broadcast endpoint selector.
    #[must_use]
    pub const fn endpoint(&self) -> endpoint::Broadcast {
        self.endpoint
    }
}

impl_fmt_pair!(
    Broadcast,
    short_id::Broadcast,
    endpoint::Broadcast,
    |value| (value.address, value.endpoint),
    ":"
);
